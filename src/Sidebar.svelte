<script lang="ts">
    import {currentTemplate, send, template} from "./store.ts";
    import {fly} from 'svelte/transition';
    import {flip} from 'svelte/animate';

    export let current;
    let items = [];
    $: items = [...Object.keys($template)];

    $: {
        // for (const templateKey of Object.entries($template)) {
        //     console.log('?', templateKey);
        //     templateKey[0] = '测试0';
        // }
        if ($template[current]) $currentTemplate = $template[current];
    }

    function add() {
        $template['未命名模板' + Object.keys($template).length] = {
            title: '',
            files: [],
            copyright: 1,
            source: "",
            tid: 0,
            desc: "",
            tag: '',
            dynamic: "",
            cover: '',
            desc_format_id: 0,
            subtitle: {
                open: 0,
                lan: ''
            },
            videos: [],
            open_subtitle: false
        };
    }

    function select(item) {
        // $currentTemplate = $template[item];
        console.log('???', $template);
        current = item;
    }
</script>
<div class="flex flex-col w-72 h-screen px-4 py-8 bg-white border-r dark:bg-gray-800 dark:border-gray-600 overflow-auto"
     transition:fly={{delay: 400, x: -100}}>
    <h2 class="text-3xl font-semibold text-gray-800 dark:text-white">模板</h2>

    <div class="flex flex-col justify-between flex-1 mt-6">
        <nav>
            {#each items as item(item)}

                <a animate:flip="{{duration: 300}}" class:selected="{current === item}"
                   on:click="{() => select(item)}">
                    {#if ($template[item].changed)}
                        <span class="flex absolute h-1.5 w-1.5 top-0 right-0 flex">
                          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-purple-400 opacity-75"></span>
                          <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-purple-500"></span>
                        </span>
                    {/if}
                    <svg class="flex-none w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M19 11H5M19 11C20.1046 11 21 11.8954 21 13V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V13C3 11.8954 3.89543 11 5 11M19 11V9C19 7.89543 18.1046 7 17 7M5 11V9C5 7.89543 5.89543 7 7 7M7 7V5C7 3.89543 7.89543 3 9 3H15C16.1046 3 17 3.89543 17 5V7M7 7H17"
                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    {#if current !== item}
                        <div out:send={{key: item}}></div>
                    {/if}
                    <span class="ml-4 font-medium truncate">{item}</span>
                </a>

            {/each}
        </nav>

        <button class="mt-6 py-2 px-4 flex justify-center items-center  bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full"
                on:click={add}
                type="button">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                 xmlns="http://www.w3.org/2000/svg">
                <path d="M12 4v16m8-8H4" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
            </svg>
        </button>
    </div>
</div>

<style>
    .selected {
        @apply text-gray-700 bg-gray-200;
    }

    nav > a {
        @apply flex cursor-pointer items-center px-3 py-2 mt-1 text-gray-600 transition-colors duration-200 transform rounded-md hover:bg-gray-200 hover:text-gray-700;
    }

</style>
