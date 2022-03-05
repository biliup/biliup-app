<script lang="ts">
    import {currentTemplate, send, template} from "./store.ts";
    import {fly} from 'svelte/transition';
    import {flip} from 'svelte/animate';
    import {invoke} from "@tauri-apps/api/tauri";
    import {fetch, ResponseType} from "@tauri-apps/api/http";
    import {open} from "@tauri-apps/api/shell";
    import {configDir} from "@tauri-apps/api/path";

    export let current;
    let face = 'noface.jpg';
    let name = null;
    invoke('get_myinfo').then((ret) => {
        console.log(ret);
        console.log(ret['data']['face']);
        fetch(<string>ret['data']['face'], {method: "GET", responseType: ResponseType.Binary}).then((res)=>{
            face = 'data:image/jpeg;base64,' + arrayBufferToBase64(res.data);
        })
        name = ret['data']['name'];
    });

    function arrayBufferToBase64(buffer){
        var binary = '';
        var bytes = new Uint8Array(buffer);
        var len = bytes.byteLength;
        for (var i = 0; i < len; i++) {
            //将 Unicode 编码转换为一个字符串:
            //fromCharCode() 可接受一个指定的 Unicode 值，然后返回一个字符串。
            binary += String.fromCharCode(bytes[i]);
        }
        //window.btoa()：将ascii字符串或二进制数据转换成一个base64编码过的字符串,该方法不能直接作用于Unicode字符串.
        return window.btoa(binary);
    }

    let items = [];
    $: items = [...Object.keys($template)];

    $: {
        // for (const templateKey of Object.entries($template)) {
        //     console.log('?', templateKey);
        //     templateKey[0] = '测试0';
        // }
        if ($template[current]) $currentTemplate = $template[current];
    }

    async function add() {
        let name = '未命名模板' + Object.keys($template).length;
        $template[name] = {
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
            open_subtitle: false,
            atomicInt: 0
        };
        // $currentTemplate = $template[name];
        let res = await invoke('load');
        res.streamers = $template;
        await invoke('save', {config: res});
    }

    function select(item) {
        // $currentTemplate = $template[item];
        current = item;
    }

    async function openConfigDir(){
        await open(await configDir()+'biliup');
    }
    let lines = ['ws', 'qn', 'auto', 'bda2', 'kodo'];
    let line = 'auto';
    let limit = 3;

    async function loadSettings() {
        let ret = await invoke('load');
        console.log(ret);
        if (ret.line === null) {
            line = 'auto';
        } else {
            line = ret.line;
        }
        limit = <number>ret['limit'];
    }

    async function saveSettings() {
        let ret = await invoke('load');
        console.log(ret);
        if (line === 'auto') {
            ret.line = null;
        } else {
            ret.line = line;
        }
        ret.limit = limit;
        await invoke('save', {config: ret});
    }
</script>
<div class="flex flex-col w-72 h-screen px-4 py-8 bg-white border-r dark:bg-gray-800 dark:border-gray-600 overflow-auto"
     transition:fly={{delay: 400, x: -100}}>
    <div class="flex items-center px-3 -mx-2">
        <img class="object-cover rounded-full h-9 w-9" src="{face}" alt="avatar"/>
        <div data-tip="打开配置文件夹" class="tooltip">
            <h4 on:click={openConfigDir} class="mx-2 font-medium text-gray-800 dark:text-gray-200 hover:underline truncate">{name}</h4>
        </div>

        <label for="my-modal-2" data-tip="设置" class="cursor-pointer tooltip" on:click={loadSettings}>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
            </svg>
        </label>
        <input type="checkbox" id="my-modal-2" class="modal-toggle">
        <div class="modal">
            <div class="modal-box">
                <div class="space-y-2.5">
                    <h4>单视频并发数：{limit}</h4>
                    <input type="range" max="128" min="1" bind:value={limit} class="range">
<!--                    <button class="btn btn-outline">线路: AUTO</button>-->
                    <h4>上传线路选择：</h4>
                    <div class="btn-group">
                        {#each lines as l}
                            <input type="radio" bind:group={line} value="{l}" data-title="{l}" class="btn btn-outline">
                        {/each}
                    </div>
                </div>

                <div class="modal-action">
                    <label for="my-modal-2" on:click={saveSettings} class="btn btn-accent">Save</label>
                    <label for="my-modal-2" class="btn">Close</label>
                </div>
            </div>
        </div>
    </div>

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
