<script lang="ts">
	import Login from './Login.svelte';
	import Home from './Home.svelte';
	import { isLogin } from './store.ts';
	import { invoke } from '@tauri-apps/api/tauri';
	// invoke('test')
	// 		.then((res) =>
	// 				console.log(`Message: ${res}`)
	// 		)
	// 		.catch((e) => console.log(e))
</script>

<main class="bg-gray-100 dark:bg-gray-800" on:drop|preventDefault on:dragover|preventDefault on:dragleave|preventDefault on:dragenter|preventDefault>
	{#if $isLogin}
		<Home/>
	{:else}
		<Login/>
	{/if}
	<div id="alerts" class="fixed space-y-2.5 inset-y-0 right-0 w-1/2 pointer-events-none"></div>
</main>


<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>
