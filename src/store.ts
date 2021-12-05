import {writable} from "svelte/store";
import {open} from "@tauri-apps/api/dialog";
import {sep} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";
import {crossfade, fly} from "svelte/transition";
import {listen} from "@tauri-apps/api/event";
import {createPop} from "./common";


export const isLogin = writable(false);
export const template = writable({});
export const currentTemplate = writable({
    title: '',
    files: [],
    copyright: 1,
    source: "",
    tid: 0,
    desc: "",
    dynamic: "",
    tag: '',
    videos: [],
    changed: false
});
export const [send, receive] = crossfade({
    duration: 800,
    fallback: (node, params) => {
        return fly(node, {x: 200, delay: 500});
    },
});
export const fileselect = () => {
    let properties = {
        // defaultPath: 'C:\\',
        multiple: true,
        // directory: false,
        filters: [{
            extensions: ['mp4', 'flv', 'avi', 'wmv', 'mov', 'webm', 'mpeg4', 'ts', 'mpg', 'rm', 'rmvb', 'mkv', 'm4v'],
            name: ""
        }]
    };
    open(properties).then((pathStr) => {
        console.log(pathStr);
        if (!pathStr) return;
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

        for (const file of files) {
            // file.type.match
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
                name: split[split.length - 1],
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

function allComplete(files, temp) {
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
    }).then((res) => {
        // temp.atomicInt--;
        video.filename = res[0].filename;
        video.speed = res[1];
        video.complete = true;
        video.progress = 100;

        console.log(`Message:`, res);
    }).catch((e) => {
        createPop(e, 5000);
        console.log(e);
    }).finally(() => {
        temp.atomicInt--;
        currentTemplate.update(t => t);
        if (allComplete(temp['files'], temp)) {
            console.log("???");
            return;
        }
    })
}

export async function progress() {
    return await listen('progress', event => {
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        // console.log('!', event);
        currentTemplate.update((cur) => {
            for (const file of cur['files']) {
                if (file.filename === event.payload[0]) {
                    // file.progress = Math.round(event.payload[1] * 100) / 100;
                    // $speed = Math.round(event.payload[1] * 100) / 100;
                    file.speed = event.payload[2];
                    // file.progress.ldBar.set(Math.round(event.payload[1] * 100) / 100);
                    file.progress = event.payload[1];
                    if (Math.round(event.payload[1] * 100) === 10000) file.complete = true;

                    return cur;
                }
            }
            return cur;
        })
    });
}
