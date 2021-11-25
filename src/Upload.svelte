<script lang="ts">
    import DragAndDrop from './Drag-and-drop.svelte';
    import Append from './Append.svelte';
    import {attach, createPop, currentTemplate, isLogin, template, fileselect} from './store.ts';
    import { fly } from 'svelte/transition';
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from "@tauri-apps/api/dialog";
    import {dialog} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";

    export let selected;
    let visible: boolean;
    let title: string = '';
    let nocopyright: boolean = false;
    let source: string = '';
    let tid: number;
    let desc: string = '';
    let dynamic: string = '';
    let tag: string = 'biliup';
    let cover: string = '';

    let tags = [];
    let tempTag;
    $: visible = $currentTemplate['files'].length !== 0;
    function submit() {
        console.log(title);
        console.log(nocopyright);
        console.log(source);
        console.log(tid);
        console.log(desc);
        console.log(dynamic);
        console.log(tag);
        console.log($currentTemplate['files']);
        // const files = [];

        invoke('submit', {studio: {
            title: title,
            copyright: nocopyright === false ? 1 : 2 ,
            source: source,
            tid:tid ?? 171,
            desc: desc,
            dynamic: dynamic,
            tag:tag,
                cover:cover,
                desc_format_id: 0,
                subtitle: {
                    open: 0,
                    lan: ''
                },
                videos: $currentTemplate['files'],
                open_subtitle: false
        }})
            .then((res) => {
                console.log(res);
            }).catch((e) => {
                createPop(e, 5000);
                console.log(e);
            }
        )
    }

    function handleKeypress() {
        if (tags.includes(tempTag)) {
            createPop("已有相同标签");
            tempTag = null;
            return;
        }
        tags = [...tags, tempTag];
        tempTag = null;
        return false;
    }

    function removeTag(tag) {
        tags = tags.filter(t => t !== tag);
        console.log(tag);
    }

</script>
<div>
    <div transition:fly="{{ y: -200, delay: 500 }}" class="shadow-md md:max-w-xl sm:max-w-sm lg:max-w-2xl w-screen p-10 mt-2 mb-2 bg-white rounded-xl">
        <div class="space-y-3">
            <div class="flex flex-col space-y-2">
                <label class="text-sm font-bold text-gray-500 tracking-wide">{selected}</label>
                <input bind:value={title} class="text-base p-2 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500" placeholder="标题">
            </div>
            <div class="grid">
                <div class="grid grid-cols-2  mb-1">
                    <label class="w-auto text-sm font-bold text-gray-500 tracking-wide">视频</label>
                    {#if (visible)}
                    <button type="button" on:click={fileselect} class="justify-self-end py-2 px-4 flex justify-center items-center w-max bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white transition ease-in duration-200 text-center text-xs font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full">
                        <input id="add" on:change={(event)=>attach(event.target.files)} on:change={(event)=> event.target.value=null}
                               type="file" class="hidden" multiple accept=".mp4,.flv,.avi,.wmv,.mov,.webm,.mpeg4,.ts,.mpg,.rm,.rmvb,.mkv,.m4v">
                        <svg class="mr-2 w-3 h-3" fill="currentColor" viewBox="0 0 1792 1792" xmlns="http://www.w3.org/2000/svg">
                            <path d="M1344 1472q0-26-19-45t-45-19-45 19-19 45 19 45 45 19 45-19 19-45zm256 0q0-26-19-45t-45-19-45 19-19 45 19 45 45 19 45-19 19-45zm128-224v320q0 40-28 68t-68 28h-1472q-40 0-68-28t-28-68v-320q0-40 28-68t68-28h427q21 56 70.5 92t110.5 36h256q61 0 110.5-36t70.5-92h427q40 0 68 28t28 68zm-325-648q-17 40-59 40h-256v448q0 26-19 45t-45 19h-256q-26 0-45-19t-19-45v-448h-256q-42 0-59-40-17-39 14-69l448-448q18-19 45-19t45 19l448 448q31 30 14 69z">
                            </path>
                        </svg>
                        添加视频
                    </button>
                    {/if}

                </div>

                {#if $currentTemplate['files'].length === 0}<DragAndDrop/>{:else}<Append/>{/if}
            </div>
            <p class="text-sm text-gray-300">
                File type: .mp4,.flv,.avi,.wmv,.mov,.webm,.mpeg4,.ts,.mpg,.rm,.rmvb,.mkv,.m4v
            </p>
            <div class="mb-3 flex justify-between items-center">
                <div>
                    <div class="relative inline-block w-10 mr-2 align-middle select-none">
                        <input bind:checked={nocopyright} type="checkbox" name="toggle" id="Orange" class="checked:bg-yellow-500 border-blue-200 outline-none focus:outline-none right-4 checked:right-0 duration-200 ease-in absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer "/>
                        <label for="Orange" class="block overflow-hidden h-6 rounded-full bg-gray-100 cursor-pointer">
                        </label>
                    </div>
                    <span class="w-auto text-sm text-gray-500 tracking-wide">
                            是否转载
                </span>
                </div>
                <div class:copyright={nocopyright} class="pl-4 invisible flex-grow">
                    <input bind:value={source} type="text" id="rounded-email" class=" rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="转载来源"/>
                </div>
            </div>
            <div class="flex-grow">
                <input bind:value={tid} type="text" class=" rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="分区"/>
            </div>
            <div class="flex flex-wrap rounded-lg border-transparent border border-gray-300 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent">
                {#each tags as tag}
                    <span class="flex  ml-1 my-1.5 px-3 py-0.5 text-base rounded-full text-white  bg-indigo-500 ">
                        {tag}
                        <button on:click={(e)=>{removeTag(tag)}} class="bg-transparent hover">
                            <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" fill="currentColor" class="ml-2" viewBox="0 0 1792 1792">
                                <path d="M1490 1322q0 40-28 68l-136 136q-28 28-68 28t-68-28l-294-294-294 294q-28 28-68 28t-68-28l-136-136q-28-28-28-68t28-68l294-294-294-294q-28-28-28-68t28-68l136-136q28-28 68-28t68 28l294 294 294-294q28-28 68-28t68 28l136 136q28 28 28 68t-28 68l-294 294 294 294q28 28 28 68z">
                                </path>
                            </svg>
                        </button>
                    </span>
                {/each}

                <input on:keypress={e=>e.key==='Enter' && handleKeypress()} bind:value={tempTag} type="text" class="outline-none rounded-lg flex-1 appearance-none  w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base " placeholder="标签"/>
            </div>
            <div class="text-gray-700">
                <label class="text-sm font-bold text-gray-500 tracking-wide">简介</label>
                <textarea bind:value={desc} class="flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="简介补充: ..." rows="5" cols="40"></textarea>
            </div>
            <div class="text-gray-700">
                <label class="text-sm font-bold text-gray-500 tracking-wide">粉丝动态</label>
                <textarea bind:value={dynamic} class="flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="动态描述" rows="5" cols="40"></textarea>
            </div>
            <button on:click|preventDefault={submit} type="submit" class="p-2 my-5 w-full flex justify-center bg-blue-500 text-gray-100 rounded-full tracking-wide
                          font-semibold  focus:outline-none focus:shadow-outline hover:bg-blue-600 shadow-lg cursor-pointer transition ease-in duration-300">
                Upload
            </button>
        </div>
    </div>
</div>


<style>
    .copyright {
        @apply visible;
    }
</style>
