<script lang="ts">
    import {tweened} from "svelte/motion";
    import {emit} from "@tauri-apps/api/event";
    import {getCurrent} from "@tauri-apps/api/window";

    import {contentLimitation} from "./lib/constants";
    import type {SelectedTemplate} from "./global";

    export let selectedTemplate: SelectedTemplate;
    export let file;
    let complete = file.complete;
    let progress = 0;
    let totalSize = 0;
    let speed = 0;
    $:  {
        complete = file.complete;
        totalSize = file.totalSize;
        progress = file.progress;
        speed = file.speed;
    }
    let id = file.id;
    const fitSpeed = tweened(null, {
        duration: 2500,
        // easing: cubicOut
    });
    const fitProgress = tweened(null, {
        duration: 1000,
        // easing: cubicOut
    });

    $: {
        $fitSpeed = speed;
        $fitProgress = progress;
        // console.log("speed", speed);
        // console.log("progress", progress);
    }
    const current = getCurrent();

    function strToHexCharCode(str: string): string {
        if(str === "")
            return "";
        let hexCharCode = [];
        for(let i = 0; i < str.length; i++) {
            hexCharCode.push((str.charCodeAt(i)).toString(16));
        }
        return hexCharCode.join("");
    }
    async function remove() {
        let index = selectedTemplate.files.findIndex(value => value.id === id);

        // TODO: change confirmation prompt to modal
        if (!(await confirm(`确定要移除 ${selectedTemplate.files[index].title} 吗？`))) { // confirm() is returning a Promise
            return;
        }

        await emit(strToHexCharCode(id));
        selectedTemplate.files = selectedTemplate.files.filter(value => value.id !== id);
    }
</script>
<div class="flex items-center justify-center space-x-2 px-1">
    <div class="parent-svg flex-none w-12 items-center justify-center">
        <!--{#if complete}-->
        <!--    <svg xmlns="http://www.w3.org/2000/svg" class="m-auto h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
        <!--        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />-->
        <!--    </svg>-->
        <!--{:else }-->
        <svg class="svg m-auto h-7 w-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"
             xmlns="http://www.w3.org/2000/svg">
            <path d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" stroke-linecap="round" stroke-linejoin="round"
                  stroke-width="2"/>
        </svg>
        <svg class="del animate-bounce m-auto h-7 w-7" fill="currentColor" on:click={remove}
             viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path clip-rule="evenodd"
                  d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                  fill-rule="evenodd"/>
        </svg>

        <!--{/if}-->
    </div>
    <div class="flex-grow w-0">
        <div class="flex">
            <span class="w-full">
                <div class="flex w-full justify-between">
                    {#if file.title.length <= contentLimitation.videoPartTitleLength}
                        <input bind:value="{file.title}" class="bg-inherit w-full truncate"/>
                    {:else}
                        <input bind:value="{file.title}" class="bg-inherit w-full truncate bg-red-100 border border-red-300"/>
                    {/if}
                    <!--{title}-->
                    <div class="text-gray-500 min-w-fit text-sm">{(totalSize/1024/1024).toFixed(2)} MiB</div>
                </div>
                <span class="block max bg-yellow-300 border-yellow-300 border-opacity-60 border rounded-full" class:complete={!complete}
                      style="width: {$fitProgress}%;"></span>
            </span>
        </div>
    </div>
    <div class="flex-none flex flex-col w-20 h-12 justify-center items-center text-gray-500 font-mono text-xs">
        <!-- This item will not grow -->
        <div>
            <!--{Math.round($fitSpeed * 100) / 100} MB/s-->
            {$fitSpeed.toFixed(2)} MB/s
        </div>
        <div>
            {$fitProgress.toFixed(2)} %
        </div>
    </div>
</div>
<style>
    .complete {
        @apply animate-pulse;
    }

    .del {
        display: none;
    }

    .parent-svg:hover > .del {
        display: block;
    }

    .parent-svg:hover > .svg {
        display: none;
    }

</style>
