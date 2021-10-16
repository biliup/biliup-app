<script lang="ts">
    import {template, currentTemplate} from "./store.ts";
    import { fly, blur } from 'svelte/transition';
    let current = $template.keys().next().value;
    currentTemplate.set($template.get(current));
    function add() {
        template.update( map => map.set('未命名模板'+map.size, {'title': '', 'files': []}));
    }
    function select(item) {
        currentTemplate.set($template.get(item));
        current = item;
    }
</script>
<div transition:fly={{delay: 400, x: -100}} class="flex flex-col w-72 h-screen px-4 py-8 bg-white border-r dark:bg-gray-800 dark:border-gray-600 overflow-auto">
    <h2 class="text-3xl font-semibold text-gray-800 dark:text-white">模板</h2>

    <div class="flex flex-col justify-between flex-1 mt-6">
        <nav>
            {#each [...$template.keys()] as item}
                <a class:selected="{current === item}"
                   on:click="{() => select(item)}">
                    <svg class="flex-none w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M19 11H5M19 11C20.1046 11 21 11.8954 21 13V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V13C3 11.8954 3.89543 11 5 11M19 11V9C19 7.89543 18.1046 7 17 7M5 11V9C5 7.89543 5.89543 7 7 7M7 7V5C7 3.89543 7.89543 3 9 3H15C16.1046 3 17 3.89543 17 5V7M7 7H17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                    </svg>

                    <span class="mx-4 font-medium truncate">{item}</span>
                </a>
            {/each}
        </nav>

        <button on:click={add}
                type="button" class="mt-6 py-2 px-4 flex justify-center items-center  bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
        </button>
    </div>
</div>
<div class="grid justify-center w-screen h-screen rhs overflow-y-auto">
    <div class="grid items-center justify-around min-h-screen">
        <slot {current}></slot>
    </div>
</div>

<style>
    .selected {
        @apply text-gray-700 bg-gray-200;
    }
    nav > a {
        @apply flex cursor-pointer items-center px-4 py-2 mt-1 text-gray-600 transition-colors duration-200 transform rounded-md hover:bg-gray-200 hover:text-gray-700;
    }

</style>
