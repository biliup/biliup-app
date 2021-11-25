import {writable} from "svelte/store";
import Pop from "./Pop.svelte";
import {check_outros, group_outros, transition_out} from "svelte/internal";
import {open} from "@tauri-apps/api/dialog";
import {sep} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";
let map = new Map();
map.set('未命名模板', {'title': '', 'files': [], 'atomicInt': 0})
map.set('未命名模板1', {'title': '', 'files': [], 'atomicInt': 0})

export const isLogin = writable(false);
export const template = writable(map);
export const currentTemplate = writable({});

export const fileselect = () => {
    let properties = {
        // defaultPath: 'C:\\',
        multiple: true,
        // directory: false,
        filters: [{
            extensions: ['mp4','flv','avi','wmv','mov','webm','mpeg4','ts','mpg','rm','rmvb','mkv','m4v'], name: ""
        }]
    };
    open(properties).then((pathStr) => {
        console.log(pathStr);
        attach(pathStr);
    });
};

export function attach(files) {
    currentTemplate.update(temp => {
        function findFile(file) {
            // return temp['files'].find(function(existingFile) {
            //     return (
            //         existingFile.name === file.name &&
            //         existingFile.lastModified === file.lastModified &&
            //         existingFile.size === file.size &&
            //         existingFile.type === file.type
            //     )
            // });
            return temp['files'].find((existingFile) => existingFile.filename === file);
        }

        for ( const file of  files) {
            // file.type.match
            console.log(file);
            // if (!file.type.match("video.*")){
            //     createPop('请上传视频文件！');
            //     continue;
            // }
            if (findFile(file)) {
                createPop('请上传非重复视频！');
                continue;
            }
            let split = file.split(sep);

                // temp['files'] = [...temp['files'], ...event.target.files];
            temp['files'].push({
                filename: file,
                name: split[split.length-1] ,
                desc: '',
                progress: 0,
                speed: 0,
                complete: false,
                process: false,
            });
            // let objectURL = URL.createObjectURL(file);
            // console.log(objectURL);
        }
        const res = allComplete(temp['files'], temp);
        console.log(res);
        return temp;
    });
}

function allComplete(files, temp){
    // console.log(temp);
    for (const file of files) {
        if (!file.complete && !file.process && temp.atomicInt < 1) {
            temp.atomicInt++;
            file.process = true;
            upload(file, temp);
            console.log(temp);
            return false;
        }
    }
    return true;
}

function upload(video, temp) {
    // const files = [];

    invoke('upload', {
        video: video,
    })
        .then((res) => {
            temp.atomicInt--;
            video.filename = res[0].filename;
            video.speed = res[1];
            video.complete = true;
            video.progress = 100;
            currentTemplate.update(t => t);
            if (allComplete(temp['files'], temp)) {
                console.log("???");
                return;
            }
            console.log(`Message:`, res);
        }).catch((e) => {
            createPop(e, 5000);
            console.log(e);
        }
    )
}

export function createPop(msg, duration=3000) {
    const pop = new Pop({
        target: document.querySelector('#alerts'),
        intro: true,
        props: {
            msg: msg
        }
    });
    setTimeout(()=>outroAndDestroy(pop), duration );
}

// Workaround for https://github.com/sveltejs/svelte/issues/4056
const outroAndDestroy = (instance) => {
    if (instance.$$.fragment && instance.$$.fragment.o) {
        group_outros();
        transition_out(instance.$$.fragment, 0, 0, () => {
            instance.$destroy();
        });
        check_outros();
    } else {
        instance.$destroy();
    }
};
