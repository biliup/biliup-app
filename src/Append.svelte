<script>
    import {currentTemplate, attach} from './store.ts';
    import { fade } from 'svelte/transition';
    import { flip } from 'svelte/animate';

</script>

<div in:fade class="flex flex-col">
    {#each $currentTemplate.files as file}
        <div class="flex items-center justify-center space-x-2 px-1">
            <div class="flex-none w-12 items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="m-auto h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
            </div>
            <div class="flex-grow">
                <span class="font-mono inline-block">
                            {file.name} <span class="block max bg-yellow-300 border-yellow-300 border-opacity-60 border rounded-full"></span>
                </span>
                <div class="w-full h-2 bg-gray-400 rounded-full mt-3">
                    <div class="w-3/4 h-full text-center text-xs text-white bg-green-500 rounded-full">
                    </div>
                </div>
            </div>
            <div class="flex-none w-16 h-16 ">
                <!-- This item will not grow -->
            </div>
        </div>
    {/each}
</div>

<style>
    .max {
        width: 5rem;
    }
</style>
