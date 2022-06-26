<script lang="ts">
    import Sidebar from './Sidebar.svelte';
    import Upload from './Upload.svelte';
    import {attach, progress,speed, template, currentTemplate} from "./store";
    import {listen} from "@tauri-apps/api/event";
    import {invoke} from "@tauri-apps/api/tauri";
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
    listen("tauri://file-drop", (date) => {
        console.log("1", date);
        attach(date.payload);
        $fileHover = false;
        // setContext("hover", fileHover);
    });
    listen("tauri://file-drop-hover", (date) => {
        console.log("2", date);
        $fileHover = true;
        // setContext("hover", fileHover);
    });
    listen("tauri://file-drop-cancelled", (date) => {
        console.log("3", date);
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
