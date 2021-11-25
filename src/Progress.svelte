<script>
    import { draw } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';
    import {tweened} from "svelte/motion";
    export let complete;
    export let progress = 0;
    export let speed = 0;
    export let name;
    const fitSpeed = tweened(null, {
        duration: 5000,
        // easing: cubicOut
    });
    const fitProgress = tweened(null, {
        duration: 5000,
        // easing: cubicOut
    });
    $: {
        $fitSpeed = speed;
        $fitProgress = progress;
    }
</script>
<div class="flex items-center justify-center space-x-2 px-1">
    <div class="flex-none w-12 items-center justify-center">
        <!--{#if complete}-->
        <!--    <svg xmlns="http://www.w3.org/2000/svg" class="m-auto h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
        <!--        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />-->
        <!--    </svg>-->
        <!--{:else }-->
            <svg xmlns="http://www.w3.org/2000/svg" class="m-auto h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path transition:draw stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
        <!--{/if}-->
    </div>
    <div class="flex-grow w-0">
        <div class="flex">
                    <span class="truncate font-mono w-full">
                            {name}
                        <span style="width: {$fitProgress}%;" class:complete={!complete} class="block max bg-yellow-300 border-yellow-300 border-opacity-60 border rounded-full"></span>
                    </span>

        </div>
        <!--                <div class="w-full h-4 bg-gray-400 rounded-full mt-3">-->
        <!--                    <div style="width: {file.progress}%" class="pl-2 h-full text-center text-xs text-white bg-indigo-500 rounded-full">-->
        <!--                        {file.progress}%-->
        <!--                    </div>-->
        <!--                </div>-->


        <!--                    <progress value="0.75">-->
        <!--                    </progress>-->

    </div>
    <div class="flex-none flex flex-col w-16 h-12 justify-center items-center text-gray-500 font-mono text-xs">
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
</style>
