<script lang="ts">
    import {attach, fileselect} from './store';
    import {flip} from 'svelte/animate';
    import Progress from "./Progress.svelte";
    import DragAndDrop from './Drag-and-drop.svelte';
    import {dndzone} from "svelte-dnd-action";
    import type {SelectedTemplate} from "./global";

    export let selectedTemplate: SelectedTemplate;

    const flipDurationMs = 300;

    function handleDndConsider(e) {
        // items = e.detail.items;
        selectedTemplate['files'] = e.detail.items;
    }

    function handleDndFinalize(e) {
        // items = e.detail.items;
        selectedTemplate['files'] = e.detail.items;
        console.log('Finalize', e.detail.items);
    }

    function sortVideos() {
        selectedTemplate['files'] = selectedTemplate['files'].sort((a, b) => {
            if (order) {
                return a.id.localeCompare(b.id);
            } else {
                return b.id.localeCompare(a.id);
            }
        });
    }

    let items = [];

    let order = false;
    let visible;

    $: if (selectedTemplate) {
        visible = selectedTemplate?.files?.length !== 0;
        items = [...selectedTemplate?.files];
    }
</script>

<div class="grid">
    <div class="grid grid-flow-col mb-1 ">
        <label class="w-auto flex items-center font-bold tracking-wide">
            视频
            <label class="swap swap-rotate ml-2" on:click={sortVideos}>

                <!-- this hidden checkbox controls the state -->
                <input type="checkbox" bind:checked="{order}"/>

                <svg xmlns="http://www.w3.org/2000/svg" class="swap-off fill-current h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
                </svg>
                <svg xmlns="http://www.w3.org/2000/svg" class="swap-on fill-current h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
                </svg>
            </label>
        </label>


        {#if (visible)}
            <button type="button" on:click={fileselect}
                    class="justify-self-end py-1.5 px-3 flex justify-center items-center w-max bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white transition ease-in duration-200 text-center text-xs font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full">
                <input id="add" on:change={(event)=>attach(event.target.files)}
                       on:change={(event)=> event.target.value=null}
                       type="file" class="hidden" multiple
                       accept=".mp4,.flv,.avi,.wmv,.mov,.webm,.mpeg4,.ts,.mpg,.rm,.rmvb,.mkv,.m4v">
                <svg class="mr-2 w-3 h-3" fill="currentColor" viewBox="0 0 1792 1792"
                     xmlns="http://www.w3.org/2000/svg">
                    <path d="M1344 1472q0-26-19-45t-45-19-45 19-19 45 19 45 45 19 45-19 19-45zm256 0q0-26-19-45t-45-19-45 19-19 45 19 45 45 19 45-19 19-45zm128-224v320q0 40-28 68t-68 28h-1472q-40 0-68-28t-28-68v-320q0-40 28-68t68-28h427q21 56 70.5 92t110.5 36h256q61 0 110.5-36t70.5-92h427q40 0 68 28t28 68zm-325-648q-17 40-59 40h-256v448q0 26-19 45t-45 19h-256q-26 0-45-19t-19-45v-448h-256q-42 0-59-40-17-39 14-69l448-448q18-19 45-19t45 19l448 448q31 30 14 69z">
                    </path>
                </svg>
                添加视频
            </button>
        {/if}

    </div>

    {#if !visible}
        <DragAndDrop/>
    {:else}
        <!--{#await promise then value}-->
        <div use:dndzone="{{items: selectedTemplate.files, flipDurationMs}}" on:consider="{handleDndConsider}"
             on:finalize="{handleDndFinalize}" class="bg-[#fafcfd] flex flex-col rounded-lg">
            {#each selectedTemplate.files as file(file.id)}
                <div class="shadow-sm rounded-lg" animate:flip="{{duration: flipDurationMs}}">
                    <Progress bind:file={file} bind:selectedTemplate={selectedTemplate}/>
                    <!--{file.title}-->
                </div>
            {/each}
        </div>
        <!--{/await}-->
    {/if}
</div>


<style>
    .max {
        width: 5rem;
    }
</style>
