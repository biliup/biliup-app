<script lang="ts">
    import DragAndDrop from './Drag-and-drop.svelte';
    import Append from './Append.svelte';
    import {attach, currentTemplate, isLogin, template, fileselect, receive, send} from './store.ts';
    import { fly, fade } from 'svelte/transition';
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from "@tauri-apps/api/dialog";
    import {dialog} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";
    import {onMount} from "svelte";
    import Partition from "./Partition.svelte";
    import {archivePre, createPop, partition} from "./common";

    export let selected;
    let oldSelected = selected;
    // let title: string = ;
    let nocopyright: boolean = $currentTemplate.copyright === 2;
    $ : nocopyright = $currentTemplate.copyright === 2;

    function handleClick(e) {
        console.log(e.target.checked);
        $currentTemplate.copyright = e.target.checked ? 2 : 1;
    }

    // let source: string = $currentTemplate?.source;
    // let tid: number = $currentTemplate?.tid;
    // let desc: string = $currentTemplate?.desc;
    // let dynamic: string = $currentTemplate?.dynamic;
    // let cover: string = $currentTemplate?.cover;
    let edit = false;
    function update(e) {
        if (!e) {
            console.log(oldSelected);
            delete $template[oldSelected];
            oldSelected = selected
            $currentTemplate.changed = true;
            $template[selected] = $currentTemplate;
            console.log($template);
        }
        edit = e;
    }
    function del() {
        delete $template[selected];
        $template = $template;
        console.log($template);
        invoke('load', )
            .then((res) => {
                invoke('save', {config: {
                        user: res.user,
                        streamers: $template,
                    }})
                    .then((res) => {
                        console.log(res,);
                        createPop('移除成功', 2000, 'Success');
                    }).catch((e) => {
                        createPop(e, 5000);
                        console.log(e);
                    }
                )
            }).catch((e) => console.log(e, 5000))
    }
    function save() {
        // console.log({[selected]: config});
        $currentTemplate.tag = tags.join(',');
        invoke('load', )
            .then((res) => {
                invoke('save', {config: {
                        user: res.user,
                        streamers: $template,
                    }})
                    .then((res) => {
                        console.log(res,);
                        $currentTemplate.changed = false;
                        $template = $template;
                        createPop('保存成功', 5000, 'Success');
                    }).catch((e) => {
                        createPop(e, 5000);
                        console.log(e);
                    }
                )
            }).catch((e) => console.log(e, 5000))
    }
    let tags = [];
    let parent = '请选择';
    let children = '分区';
    let current;
    let currentChildren;
    $:  {
        if ($partition) {
            // tags.flatMap()
            // $partition.flatMap()
            let changed = false;
            for (const partitionElement of $partition) {
                for (const child of partitionElement.children) {
                    if (child.id === $currentTemplate.tid) {
                        parent = partitionElement.name;
                        children = child.name;
                        current = partitionElement.id;
                        currentChildren = child.id;
                        changed = true;
                    }
                }
                // typeList = typeList.concat(partitionElement.children);
                // console.log(partitionElement.children);
            }
            if (!changed) {
                parent = '请选择';
                children = '分区';
                current = null;
                currentChildren = null;
            }
            // console.log(typeList);
        }
        tags = $currentTemplate.tag.length === 0 ? [] : $currentTemplate.tag.split(',');
    }
    let tempTag;
    function submit() {
        $currentTemplate.videos = $currentTemplate?.files;
            // title: title,
            // copyright: nocopyright === false ? 1 : 2 ,
            // source: source,
            // tid: tid ?? 171,
            // desc: desc,
            // dynamic: dynamic,
            // cover: '',
            // desc_format_id: 0,
            // subtitle: {
            //     open: 0,
            //     lan: ''
            // },
            // open_subtitle: false
        // console.log($currentTemplate['files']);
        // const files = [];
        // config.videos = $currentTemplate['files'];
        invoke('submit', {studio: {
                ...$currentTemplate,
                tag: ['biliup', ...tags].join(',')
            }})
            .then((res) => {
                console.log(res);
                createPop('投稿成功', 5000, 'Success');
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


    function callback(detailTid, detailParent, detailChildren) {
        $currentTemplate?.tid = detailTid;
        parent = detailParent;
        children = detailChildren;
    }

</script>
<div>
    <div in:receive={{key: selected}} class="shadow-md md:max-w-xl sm:max-w-sm lg:max-w-2xl w-screen px-10 pt-3 pb-10 mt-2 mb-2 bg-white rounded-xl">
        <div class="space-y-3">
            <div class="flex flex-row-reverse">
                <button on:click|preventDefault={del} type="button" class="ml-2 py-2 px-2 flex justify-center items-center bg-red-600 hover:bg-red-700 focus:ring-red-500 focus:ring-offset-red-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  w-8 h-8 rounded-lg ">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                </button>
                <button on:click|preventDefault={save} type="button" class="py-2 px-2 flex justify-center items-center bg-blue-600 hover:bg-blue-700 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  w-8 h-8 rounded-lg ">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                    </svg>
                </button>
            </div>
            <div class="flex flex-col">
                <label class="text-sm font-bold text-gray-500 tracking-wide mb-2">
                    {#if (edit)}
                        <input on:focusout={()=> update(false)} bind:value={selected} class="w-full p-1 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500" placeholder="标题">
                    {:else}
                        <div class="p-1">
                            {selected}
                            <svg on:click={()=> update(true)}
                                 xmlns="http://www.w3.org/2000/svg" class="cursor-pointer inline h-5 w-5 hover:text-blue-700" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                            </svg>
                        </div>
                    {/if}


                </label>
                <input bind:value={$currentTemplate.title} class="text-base p-2 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500" placeholder="标题">
            </div>
            <Append/>
            <p class="text-sm text-gray-300">
                File type: .mp4,.flv,.avi,.wmv,.mov,.webm,.mpeg4,.ts,.mpg,.rm,.rmvb,.mkv,.m4v
            </p>
            <div class="mb-3 flex justify-between items-center">
                <div>
                    <div class="relative inline-block w-10 mr-2 align-middle select-none">
<!--                        bind:checked={nocopyright}-->
                        <input checked={nocopyright} on:change={(event) => handleClick(event)} type="checkbox" name="toggle" id="Orange" class="checked:bg-blue-600  border-blue-200 outline-none focus:outline-none right-4 checked:right-0 duration-200 ease-in absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer "/>
                        <label for="Orange" class="block overflow-hidden h-6 rounded-full bg-gray-100 cursor-pointer">
                        </label>
                    </div>
                    <span class="w-auto text-sm text-gray-500 tracking-wide">
                            是否转载
                </span>
                </div>
                <div class:copyright={nocopyright} class="pl-4 invisible flex-grow">
                    <input bind:value={$currentTemplate.source} type="text" id="rounded-email" class=" rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="转载来源"/>
                </div>
            </div>
            <div use:archivePre={{callback, current, currentChildren}} class="flex w-52">
                <button type="button" class="border border-gray-300 relative w-full bg-white rounded-md pl-3 pr-10 py-3 text-left cursor-default focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                    <span class="flex items-center">
                        <span class="ml-1 block truncate">
                            {parent} → {children}
                        </span>
                    </span>
                            <span class="ml-3 absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                        <svg class="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M10 3a1 1 0 01.707.293l3 3a1 1 0 01-1.414 1.414L10 5.414 7.707 7.707a1 1 0 01-1.414-1.414l3-3A1 1 0 0110 3zm-3.707 9.293a1 1 0 011.414 0L10 14.586l2.293-2.293a1 1 0 011.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd">
                            </path>
                        </svg>
                    </span>
                </button>
<!--                <input bind:this={archivePre} bind:value={tid} type="text" class=" rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="分区"/>-->
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
                <textarea bind:value={$currentTemplate.desc} class="flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="简介补充: ..." rows="5" cols="40"></textarea>
            </div>
            <div class="text-gray-700">
                <label class="text-sm font-bold text-gray-500 tracking-wide">粉丝动态</label>
                <textarea bind:value={$currentTemplate.dynamic} class="flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="动态描述" rows="5" cols="40"></textarea>
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
