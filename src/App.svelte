<script lang="ts">
	import Login from './Login.svelte';
	import Home from './Home.svelte';
	import {isLogin} from './store.ts';
	import {overrideItemIdKeyNameBeforeInitialisingDndZones} from "svelte-dnd-action";

	overrideItemIdKeyNameBeforeInitialisingDndZones("filename");

</script>

<main class="bg-gray-100 dark:bg-gray-800" on:dragenter|preventDefault on:dragleave|preventDefault on:dragover|preventDefault
      on:drop|preventDefault>
    {#if $isLogin}
        <Home/>
    {:else}
        <Login/>
    {/if}
    <div class="fixed space-y-2.5 inset-y-0 right-0 w-1/2 pointer-events-none" id="alerts"></div>
</main>


<style global lang="postcss">
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>
