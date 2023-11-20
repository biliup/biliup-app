import tippy, {animateFill} from "tippy.js";
// import 'tippy.js/dist/tippy.css'; // optional for styling
// import 'tippy.js/dist/backdrop.css';
import 'tippy.js/animations/shift-away.css';
import 'tippy.js/themes/light.css';
import Partition from "./Partition.svelte";
import {writable} from "svelte/store";
import Pop from "./Pop.svelte";
import {check_outros, group_outros, transition_out} from "svelte/internal";
import { invoke } from "@tauri-apps/api";
import { save_config, template, currentTemplate } from './store'

export let partition = writable(null)

export function archivePre(node, combine) {
    let off;
    let detail;
    let partition;
    tippy(node, {
        // content: `1313`,
        arrow: false,
        trigger: 'click',
        allowHTML: true,
        theme: 'light',
        placement: 'bottom-start',
        animateFill: true,
        plugins: [animateFill],
        inertia: true,
        interactive: true,
        onCreate(instance) {

            partition = new Partition({
                target: <Element>instance.popper.firstChild.lastChild,
                props: {
                    current: combine.current,
                    currentChildren: combine.currentChildren
                }
            });
            off = partition.$on('tid', event => {
                combine.callback(event.detail.tid, event.detail.parent, event.detail.children);
                instance.hide();
                console.log(event);
                detail = event.detail;
            });
        },
        onShown(instance) {
            // @ts-ignore
            instance.popper.firstChild.lastChild.firstChild.firstChild.scrollTo({
                top: detail?.scroll[0]?.offsetTop - 3,
                // left: 100,
                behavior: 'smooth'
            });
            // @ts-ignore
            instance.popper.firstChild.lastChild.firstChild.lastChild.scrollTo({
                top: detail?.scroll[1]?.offsetTop - 8,
                // left: 100,
                behavior: 'smooth'
            });
            // console.log(instance.popper.firstChild.lastChild.firstChild.firstChild.scrollTop);
        },
        onDestroy(instance) {
            off();
        },
    });
    return {
        update(combine) {
            // partition = newDuration;
            partition.$set({
                current: combine.current,
                currentChildren: combine.currentChildren
            });
        },
    };
}

const notificationHistory = [];
export const notifyHistory = writable(notificationHistory);

export function createPop(msg, duration = 3000, mode = 'Error') {
    invoke('log', {level: mode, msg: msg})
    notificationHistory.push({
        type: mode,
        msg: msg,
        date: new Date(),
    });
    notifyHistory.set(notificationHistory);
    const pop = new Pop({
        target: document.querySelector('#alerts'),
        intro: true,
        props: {
            msg: msg,
            mode: mode
        }
    });
    setTimeout(() => outroAndDestroy(pop), duration);
}

// 获取视频稿件数据
export async function getManuscriptInfo(tempName: string, templateCollection) {
    const submitted = tempName?.length > 2 &&
        (tempName.startsWith('av') || tempName.startsWith('BV'))

    if (!submitted) return false

    if (!(await invoke('is_vid', { input: tempName }))) return false

    try {
        templateCollection[tempName] = await invoke('show_video', { input: tempName })
    } catch (e) {
        tempName = null
        createPop(e, 5000)
        return
    }

    templateCollection[tempName]['files'] = []
    templateCollection[tempName]['videos'].forEach(value => {
        templateCollection[tempName]['files'].push({
            filename: value.filename,
            id: value.filename,
            title: value.title,
            desc: value.desc,
            progress: 100,
            uploaded: 0,
            speed_uploaded: 0,
            speed: 0,
            totalSize: 0,
            complete: true,
            process: false,
        })
    })
    templateCollection[tempName].atomicInt = 0

    // 解决添加模板后不刷新的BUG
    template.set(templateCollection)
    currentTemplate.update(data => {
        const obj = { ...data }
        obj.selectedTemplate.files = templateCollection[tempName]['files']
        obj.selectedTemplate.videos = templateCollection[tempName]['videos']
        return obj
    })

    await save_config(ret => {
        ret.streamers = templateCollection
    })

    return true
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
