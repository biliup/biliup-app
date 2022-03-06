<script lang="ts">
    import Sidebar from './Sidebar.svelte';
    import Upload from './Upload.svelte';
    import {attach, progress, template, currentTemplate} from "./store";
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
</script>

<div class="flex items-start">
    <Sidebar/>
    <div
            class="grid justify-center w-screen h-screen rhs overflow-y-auto overflow-x-hidden">
        <div class="grid items-center justify-around min-h-screen">
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
    @import 'filepond-plugin-image-edit/dist/filepond-plugin-image-edit.css';
</style>
