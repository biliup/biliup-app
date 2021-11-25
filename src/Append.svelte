<script>
    import {currentTemplate, attach} from './store.ts';
    import { fade } from 'svelte/transition';
    import { flip } from 'svelte/animate';
    import {listen} from "@tauri-apps/api/event";
    import {tweened} from "svelte/motion";
    import {onMount} from "svelte";
    import Progress from "./Progress.svelte";
    async function progress() {
        const unlisten = await listen('progress', event => {
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
            // console.log(event);
            for (const file of $currentTemplate['files']) {
                if (file.filename === event.payload[0]) {
                    // file.progress = Math.round(event.payload[1] * 100) / 100;
                    // $speed = Math.round(event.payload[1] * 100) / 100;
                    file.speed = event.payload[2];
                    // file.progress.ldBar.set(Math.round(event.payload[1] * 100) / 100);
                    file.progress = event.payload[1];
                    if (Math.round(event.payload[1] * 100) === 10000) file.complete = true;
                    // unsubscribe = speed.subscribe(s => {
                    //     console.log(s);
                    //     file.progress =  Math.round(s * 100) / 100;
                        $currentTemplate = $currentTemplate;
                    // });
                }
            }
            // unsubscribe();
            // currentTemplate.update(c=>c);

            // console.log($currentTemplate['files']);
        });
    }
    progress();
    // onMount(() => {
    //
    //     // var bar1 = new ldBar(".ldBar");
    //     // console.log('the component has mounted');
    //     // console.log(document.getElementsByClassName('ldBar'));
    //     // var bar2 = document.getElementsByClassName('ldBar').ldBar;
    //     // bar2.set(60);
    // });

    // let prog = 75;
</script>
<div in:fade class="flex flex-col">
    {#each $currentTemplate.files as file}
        <Progress name={file.name} complete={file.complete} bind:progress={file.progress} speed={file.speed}/>
    {/each}
</div>

<style>
    .max {
        width: 5rem;
    }
    /*.ldBar-label {*/
    /*    display: none;*/
            /*color: #09f;*/
        /*font-family: 'varela round';*/
        /*font-size: 2.5em;*/
        /*font-weight: 900;*/
    /*}*/
</style>
