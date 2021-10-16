<script lang="ts">
    import DragAndDrop from './Drag-and-drop.svelte';
    import Append from './Append.svelte';
    import {attach, currentTemplate, template} from './store.ts';
    import { fly } from 'svelte/transition';

    export let selected;
    let visible: boolean;
    $: visible = $currentTemplate['files'].length !== 0;
</script>
<div>
    <div transition:fly="{{ y: -200, delay: 500 }}" class="md:max-w-xl sm:max-w-sm lg:max-w-2xl w-screen p-10 mt-2 mb-2 bg-white rounded-xl">
        <form class="space-y-3" action="#" method="POST">
            <div class="flex flex-col space-y-2">
                <label class="text-sm font-bold text-gray-500 tracking-wide">{selected}</label>
                <input class="text-base p-2 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500" placeholder="mail@gmail.com">
            </div>
            <div class="grid">
                <div class="grid grid-cols-2  mb-1">
                    <label class="w-auto text-sm font-bold text-gray-500 tracking-wide">视频</label>
                    {#if (visible)}
                    <button type="button" on:click={()=> document.getElementById('add').click()} class="justify-self-end py-2 px-4 flex justify-center items-center w-max bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white transition ease-in duration-200 text-center text-xs font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full">
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

            <button type="submit" class="p-2 my-5 w-full flex justify-center bg-blue-500 text-gray-100 rounded-full tracking-wide
                          font-semibold  focus:outline-none focus:shadow-outline hover:bg-blue-600 shadow-lg cursor-pointer transition ease-in duration-300">
                Upload
            </button>
        </form>
    </div>
</div>


<style>
    /*.has-mask {*/
    /*    position: absolute;*/
    /*    clip: rect(10px, 150px, 130px, 10px);*/
    /*}*/
</style>
