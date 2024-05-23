<script lang="ts">
    import Sidebar from './Sidebar.svelte';
    import Upload from './Upload.svelte';
    import {attach, progress,speed, template, currentTemplate} from "./store";
    import {listen, TauriEvent} from "@tauri-apps/api/event";
    import {invoke} from "@tauri-apps/api/core";
    import {createPop} from "./common";
    import {setContext} from "svelte";
    import {writable} from "svelte/store";

    let map;
    invoke('load')
        .then((res) => {
            map = res.streamers;
            for (const streamersKey in map) {
                map[streamersKey].files = [];
                if (map[streamersKey]['videos'].length > 0) {
                    map[streamersKey]['videos'].forEach((value) => {
                        map[streamersKey]['files'].push({
                            filename: value.filename,
                            id: value.filename,
                            title: value.title,
                            desc: value.desc,
                            progress: 100,
                            uploaded: 0,
                            speed_uploaded:0,
                            speed: 0,
                            totalSize: 0,
                            complete: true,
                            process: false,
                        });
                    })
                }
                map[streamersKey].atomicInt = 0;
            }
            $template = map;
            let key = Object.keys($template)[0];
            $currentTemplate.current = key;
            $currentTemplate.selectedTemplate = $template[key];
            console.log(res);
        }).catch((e) => {
            createPop(e);
            console.log(e);
        }
    )

    let fileHover = writable(false);
    setContext("hover", fileHover);
    progress();
    speed();
    listen(TauriEvent.DROP, (date: {payload: {paths: string[]}}) => {
        console.log(TauriEvent.DROP, date);
        let f: {name: string, path: string}[] = [];
        date.payload.paths.forEach((value) => {
            let currentFilename: string | undefined;
            if (value.includes("\\")) {
                currentFilename = value.split("\\").pop();
            } else {
                currentFilename = value.split("/").pop();
            }
            if (!currentFilename) {
                console.error(`unable to extract filename from ${value}`);
                return;
            }

            f.push({
                name: currentFilename,
                path: value
            });
        });
        attach(f);
        $fileHover = false;
        // setContext("hover", fileHover);
    });
    listen(TauriEvent.DRAG, (date) => {
        console.log(TauriEvent.DRAG, date);
        $fileHover = true;
        // setContext("hover", fileHover);
    });
    listen(TauriEvent.DROP_CANCELLED, (date) => {
        console.log(TauriEvent.DROP_CANCELLED, date);
        $fileHover = false;
        // setContext("hover", fileHover);
    });
    let items = [];
    $: items = [...Object.keys($template)];
</script>

<div class="flex items-start">
    <Sidebar items="{items}"/>
    <div class="w-screen h-screen overflow-y-auto overflow-x-hidden">
        <div class="min-h-screen">
            <!--        <Upload selected={current}/>-->
            {#key $currentTemplate.current}
                <Upload selected={$currentTemplate.current} selectedTemplate="{$currentTemplate.selectedTemplate}"/>
            {/key}

            <!--        <slot {current}></slot>-->
        </div>
    </div>
</div>
<style global>
    @import 'filepond/dist/filepond.css';
    @import 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.css';
</style>
