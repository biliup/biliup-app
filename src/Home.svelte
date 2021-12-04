<script lang="ts">
    import Sidebar from './Sidebar.svelte';
    import Upload from './Upload.svelte';
    import { fade } from 'svelte/transition';
    import {template, currentTemplate, progress, isLogin, attach} from "./store";
    import {listen} from "@tauri-apps/api/event";
    import {invoke} from "@tauri-apps/api/tauri";
    import {createPop} from "./common";
    import {setContext} from "svelte";
    import {writable} from "svelte/store";
    let map;
    let current;
    invoke('load')
        .then((res) => {
            map = res.streamers;
            for (const streamersKey in map) {
                map[streamersKey].files = [];
                map[streamersKey].atomicInt = 0;
            }
            $template = map;
            current = Object.keys($template)[0];
            console.log(res);
        }).catch((e) => {
            createPop(e);
            console.log(e);
        }
    )
    // map.set('未命名模板', {'title': '', 'files': [], 'atomicInt': 0, });
    // map.set('未命名模板1', {'title': '', 'files': [], 'atomicInt': 0, });
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
    <Sidebar bind:current>
        <!--{#key current}-->
        <!--    <Upload selected={current}/>-->
        <!--{/key}-->
<!--            <svelte:component this={$currentTemplate['component']} selected={current}/>-->
    </Sidebar>
    <div
            class="grid justify-center w-screen h-screen rhs overflow-y-auto overflow-x-hidden">
        <div class="grid items-center justify-around min-h-screen">
            <!--        <Upload selected={current}/>-->
            {#key current}
                <Upload selected={current}/>
            {/key}
            <!--        <slot {current}></slot>-->
        </div>
    </div>
</div>

