import {type Writable, writable} from "svelte/store";
import {open} from "@tauri-apps/plugin-dialog";
import {sep} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/core";
import {crossfade, fly} from "svelte/transition";
import {listen} from "@tauri-apps/api/event";
import {createPop} from "./common";
import type {BiliupConfig, SelectedTemplate} from "./global";


export const isLogin = writable(false);
export const template = writable({});

export const currentTemplate: Writable<{current: string, selectedTemplate: SelectedTemplate}> = writable({
    current: '',
    selectedTemplate: {
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
    }
});

export const [send, receive] = crossfade({
    duration: 800,
    fallback: (node, params) => {
        return fly(node, {x: 200, delay: 500});
    },
});
export const fileselect = () => {
    console.log("fileselect()")
    let properties = {
        // defaultPath: 'C:\\',
        multiple: true,
        directory: false,
        filters: [{
            extensions: ['mp4', 'flv', 'avi', 'wmv', 'mov', 'webm', 'mpeg4', 'ts', 'mpg', 'rm', 'rmvb', 'mkv', 'm4v'],
            name: ""
        }]
    };
    open(properties).then((pathStr) => {
        console.log("pathStr", pathStr);
        if (!pathStr) return;
        attach(pathStr);
    });
};

export function attach(files: {name: string, path: string}[]) {
    console.log("attach(files)", files);
    currentTemplate.update(temp => {
        function findFile(file) {
            return temp.selectedTemplate['files'].find((existingFile) => existingFile.id === file);
        }

        for (const file of files) {
            if (findFile(file)) {
                createPop('请上传非重复视频！');
                continue;
            }
            let filename = file.name;

            // temp['files'] = [...temp['files'], ...event.target.files];
            temp.selectedTemplate['files'].push({
                filename: file.path,
                id: file.name,
                title: filename.substring(0, filename.lastIndexOf(".")),
                desc: '',
                progress: 0,
                uploaded: 0,
                speed_uploaded: 0,
                speed: 0,
                totalSize: 0,
                complete: false,
                process: false,
            });
            // let objectURL = URL.createObjectURL(file);
            // console.log(objectURL);
        }
        const res = allComplete(temp.selectedTemplate['files'], temp.selectedTemplate);
        console.log(res);
        return temp;
    });
}

function allComplete(files: SelectedTemplate['files'], temp: SelectedTemplate) {
    console.log("allComplete(files, temp)", files, temp);
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

function upload(video: {title: string, filename: string, desc: string, [key:string]: any}, temp) {
    // const files = [];
    console.log("upload(video, temp)", video, temp);
    video.start = Date.now();
    invoke('upload', {
        video: video
    }).then((res) => {
        console.log("invoke(upload, video)", res);
        // temp.atomicInt--;
        // video.filename = res[0].filename;
        // video.speed = res[1];
        // video.complete = true;
        // video.progress = 100;
        currentTemplate.update(t => {
            t.selectedTemplate.files.forEach(file => {
                if (file.id === video.id) {
                    console.log(`file.id(${file.id}) === video.id(${video.id})`)
                    file.filename = res.filename;
                    const millis = Date.now() - file.start;
                    file.speed = file.totalSize / 1000 / millis;
                    file.complete = true;
                    file.progress = 100;
                }
                else {
                    console.error(`file.id(${file.id}) !== video.id(${video.id})`);
                }
            })
            return t;
        });
        console.log(`Message:`, res);
    }).catch((e) => {
        createPop(`${video.filename}: ${e}`, 5000);
    }).finally(() => {
        temp.atomicInt--;
        if (allComplete(temp['files'], temp)) {
            console.log(temp.submitCallback);
            if (temp.submitCallback) {
                temp.submitCallback();
                temp.submitCallback=null;
            }
            console.log("allComplete");
            return;
        }
    })
}

function extractFilename(path: string) {
    if (path.includes("/")) {
        return path.substring(path.lastIndexOf("/") + 1);
    }
    else {
        return path.substring(path.lastIndexOf("\\") + 1);
    }
}

export async function progress() {
    return await listen('progress', (event: {payload: any[]}) => {
        // console.log('listen(progress)', event);
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        // console.log('!', event);
        currentTemplate.update((cur) => {
            console.log(cur.selectedTemplate['files']);
            for (const file of cur.selectedTemplate['files']) {
                if (file.id === extractFilename(event.payload[0])) {
                    // file.progress = Math.round(event.payload[1] * 100) / 100;
                    // $speed = Math.round(event.payload[1] * 100) / 100;
                    file.totalSize = event.payload[2];
                    file.uploaded = event.payload[1];
                    // file.progress.ldBar.set(Math.round(event.payload[1] * 100) / 100);
                    file.progress = file.uploaded / file.totalSize * 100;
                    if (Math.round(file.progress * 100) === 10000) file.complete = true;

                    return cur;
                }
            }
            return cur;
        })
    });

}
export async function speed() {
    return await listen('speed', (event: {payload: any[]}) => {
        // console.log('listen(speed)', event);
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        // console.log('!', event);
        currentTemplate.update((cur) => {
            for (const file of cur.selectedTemplate['files']) {
                if (file.id === extractFilename(event.payload[0])) {
                    // file.progress = Math.round(event.payload[1] * 100) / 100;
                    // $speed = Math.round(event.payload[1] * 100) / 100;
                    file.totalSize = event.payload[2];
                    const millis = Date.now() - file.start;
                    file.speed_uploaded += event.payload[1];
                    file.speed = file.speed_uploaded / 1000 / millis;
                    // file.progress.ldBar.set(Math.round(event.payload[1] * 100) / 100);

                    return cur;
                }
            }
            return cur;
        })
    });

}

export async function save_config(configModifier: (config: BiliupConfig) => void) {
    // TODO: use Writable for configs
    try {
        let config = await load_config();
        configModifier(config);
        return await invoke('save', {
            config: config
        });
    } catch (e) {
        createPop(e, 5000);
        console.log(e);
    }
}


export async function load_config(): Promise<BiliupConfig> {
    let config: BiliupConfig = await invoke("load");
    console.log("config", config);
    return config;
}
