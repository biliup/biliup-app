<script lang="ts">
    import {currentTemplate, template, save_config, isLogin, load_config} from "./store";
    import {fly} from 'svelte/transition';
    import {flip} from 'svelte/animate';
    import {invoke} from "@tauri-apps/api/core";
    import {fetch} from "@tauri-apps/plugin-http";
    import {open} from "@tauri-apps/plugin-shell";
    import {configDir} from "@tauri-apps/api/path";
    import Modal from "./Modal.svelte";
    import {createPop} from "./common";
    import {readDir, BaseDirectory, remove, copyFile} from "@tauri-apps/plugin-fs";
    import type {BiliupConfig} from "./global";
    import {INVOKE_COMMANDS} from "./lib/constants";

    let face = 'noface.jpg';
    let name = null;
    invoke('get_myinfo', {fileName: "cookies.json"}).then(async (ret) => {
        console.log("get_myinfo", ret);
        let resp = await fetch(<string>ret['data']['face'], {method: "GET"});
        face = 'data:image/jpeg;base64,' + arrayBufferToBase64(await resp.arrayBuffer());
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

    export let items = [];

    async function add() {
        tempName = tempName?.trim();
        if (tempName?.length > 2 && (tempName.startsWith('av') || tempName.startsWith('BV'))) {
            if (await invoke('is_vid', {input: tempName})) {
                try {
                    $template[tempName] = await invoke('show_video', {input: tempName});
                } catch (e) {
                    tempName = null;
                    createPop(e, 5000);
                    return;
                }
                $template[tempName]['files'] = [];
                $template[tempName]['videos'].forEach((value) => {
                    $template[tempName]['files'].push({
                        filename: value.filename,
                        id: value.filename,
                        title: value.title,
                        desc: value.desc,
                        progress: 100,
                        uploaded: 0,
                        speed_uploaded: 0,
                        speed: 0,
                        totalSize: 0,
                        complete: true,
                        process: false,
                    });
                })
                $template[tempName].atomicInt = 0;
                await save_config((ret) => {
                    ret.streamers = $template;
                })
                tempName = null;
                return;
            }
        }
        let name = tempName ?? '未命名模板' + Object.keys($template).length;
        if (name in $template) {
            createPop('模板名称已存在', 5000);
            return;
        }
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
        await save_config((ret) => {
            ret.streamers = $template;
        })
        tempName = null;
    }

    function select(item) {
        $currentTemplate.selectedTemplate = $template[item];
        $currentTemplate.current = item;
    }

    async function getConfigDir(): Promise<string>{
        let configDirectory = await configDir();
        if (!configDirectory.endsWith('/')) {
            configDirectory += '/';
        }
        return configDirectory;
    }

    async function openConfigDir(){
        let configDirectory = await getConfigDir();
        configDirectory += "biliup";
        console.log("openConfigDir", configDirectory);
        await open(configDirectory);
    }
    let lines: string[] = ['ws', 'qn', 'auto', 'bda2', 'kodo', 'cos', 'cos-internal'];
    $: console.log("lines", lines);
    let line: string = 'auto';
    $: console.log("line", line);
    let limit: number = 3;
    let checkInputsBeforeSubmit: boolean = true;

    async function loadSettings() {
        let ret = await load_config();
        if (ret.line === null) {
            line = 'auto';
        } else {
            line = ret.line;
        }
        limit = ret.limit;

        if (ret.checkInputsBeforeSubmit == undefined || ret.checkInputsBeforeSubmit == null) {
            checkInputsBeforeSubmit = true;
        } else {
            checkInputsBeforeSubmit = ret.checkInputsBeforeSubmit;
        }
    }

    async function saveSettings() {
        await save_config((ret) => {
            if (line === 'auto') {
                ret.line = null;
            } else {
                ret.line = line;
            }
            ret.limit = limit;
            ret.checkInputsBeforeSubmit = checkInputsBeforeSubmit;
        })
    }

    let tempName: string;

    // Reads the `$APPDIR/users` directory recursively
    async function readBiliupUsersDir() {
        let entries = await readDir(await getConfigDir() + "biliup/users");
        console.log("entries", entries);
        for (const entry of entries) {
            console.log("entry.name ", entry.name);
            let ret = await invoke(INVOKE_COMMANDS.getOthersMyinfo, {fileName: `users/${entry.name}`});
            console.log(INVOKE_COMMANDS.getOthersMyinfo, ret);
            let newVar = {
                name: ret['data']['name'],
                face: 'noface.jpg',
                mid: ret['data']['mid']
            };
            user = newVar;
            people = [...people, newVar]
            let res = await fetch(<string>ret['data']['face'], {method: "GET"});
            newVar.face = 'data:image/jpeg;base64,' + arrayBufferToBase64(await res.arrayBuffer());
            people = [...people]
        }
    }

    readBiliupUsersDir().then(() => {
        console.log("people", people);
    });

    let people = [];
    async function processNewUser() {
        await invoke('logout');
        await remove(`${await configDir()}/biliup/cookies.json`);
        isLogin.set(false);
    }
    let user;
    async function processChangeUser() {
        console.log("user", user);
        await copyFile(`${await configDir()}/biliup/users/${user.mid}.json`, `${await configDir()}/biliup/cookies.json`);
        await invoke('logout');
        face = user.face;
        name = user.name;
        // isLogin.set(false);
    }

    $: console.log("user", user);
    $: console.log("face", face);
</script>
<div class="flex flex-col w-72 h-screen px-4 pt-8 bg-inherit overflow-auto"
     transition:fly={{delay: 400, x: -100}}>
    <div class="flex items-center justify-between">
        <div class="flex items-center flex-none">
            <Modal>
                <img slot="open-modal" class="object-cover rounded-full h-9 w-9 cursor-pointer hover:ring-2 hover:ring-purple-600 hover:ring-offset-2" src="{face}" alt="avatar"/>
                <div slot="box" let:componentId>
                    <label for="{componentId}" class="btn btn-sm btn-circle absolute right-2 top-2">✕</label>
                    <label for="{componentId}" on:click={processNewUser} class="group block max-w-xs mx-auto rounded-lg p-2 bg-white ring-1 ring-slate-900/5 shadow-lg space-y-3 hover:bg-sky-500 hover:ring-sky-500">
                        <div class="flex items-center space-x-3">
                            <svg class="h-6 w-6 stroke-sky-500 group-hover:stroke-white" fill="none" viewBox="0 0 24 24">
                                <!--                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">-->
                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                                <!--                        </svg>-->
                            </svg>
                            <h3 class="text-slate-900 group-hover:text-white text-sm font-semibold">注销并添加新账号</h3>
                        </div>
                    </label>
                    <ul role="list" class="p-6 divide-y divide-slate-200">
                        {#each people as person}
                            <!-- Remove top/bottom padding when first/last child -->
                            <li class="flex items-center py-0.5 first:pt-0 last:pb-0 ">
                                <img class="h-8 w-8 rounded-full" src="{person.face}" alt="" />
                                <div class="form-control w-full mx-2">
                                    <label class="label cursor-pointer">
                                        <span class="label-text">{person.name}</span>
                                        <input type="radio" name="radio-6" class="radio checked:bg-blue-500"
                                               bind:group={user} value={person} checked />
                                    </label>
                                </div>
                            </li>
                        {/each}
                    </ul>
                    <div class="modal-action">
                        <label for="{componentId}" on:click={processChangeUser} class="btn">切换账号</label>
                    </div>
                </div>
            </Modal>
            <div data-tip="打开配置文件夹" class="tooltip">
                <h4 on:click={openConfigDir} class="ml-2 font-medium text-gray-800 hover:underline truncate max-w-[8rem]">{name}</h4>
            </div>
        </div>

        <Modal>
            <a slot="open-modal" class="flex cursor-pointer tooltip items-center" data-tip="设置" on:click={loadSettings} >
                <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
                </svg>
            </a>

            <div slot="box" let:componentId>
                <div class="space-y-2.5">
                    <h4>单视频并发数：{limit}</h4>
                    <input type="range" max="128" min="1" bind:value={limit} class="range  range-xs">
                    <!--                    <button class="btn btn-outline">线路: AUTO</button>-->
                    <h4>上传线路选择：</h4>
                    <div class="join">
                        {#each lines as l}
                            <input type="radio" bind:group={line} value={l} data-title={l} aria-label={l} class="join-item btn btn-outline">
                        {/each}
                    </div>
<!--                    <h4>-->
<!--                        提交前检查内容长度：-->
<!--                        <input type="checkbox" bind:value={checkInputsBeforeSubmit}>-->
<!--                    </h4>-->

                </div>

                <div class="modal-action">
                    <label for="{componentId}" on:click={saveSettings} class="btn btn-accent">Save</label>
                    <label for="{componentId}" class="btn">Close</label>
                </div>
            </div>
        </Modal>

    </div>


    <div class="flex flex-col justify-between flex-1 mt-6">
        <nav class="">
            {#each items as item(item)}

                <a animate:flip="{{duration: 300}}" class:selected="{$currentTemplate.current === item}"
                   on:click="{() => select(item)}">
                    {#if ($template[item].changed)}
                        <span class="flex absolute h-1.5 w-1.5 top-0 right-0 flex">
                          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-purple-400 opacity-75"></span>
                          <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-purple-500"></span>
                        </span>
                    {/if}

                    {#if (item.startsWith('av') || item.startsWith('BV'))}
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-none h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
                        </svg>
                        <span class="ml-4 font-medium truncate">{$template[item].title}</span>
                    {:else}
                        <svg class="flex-none w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M19 11H5M19 11C20.1046 11 21 11.8954 21 13V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V13C3 11.8954 3.89543 11 5 11M19 11V9C19 7.89543 18.1046 7 17 7M5 11V9C5 7.89543 5.89543 7 7 7M7 7V5C7 3.89543 7.89543 3 9 3H15C16.1046 3 17 3.89543 17 5V7M7 7H17"
                                  stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                        <span class="ml-4 font-medium truncate">{item}</span>
                    {/if}
                </a>

            {/each}
        </nav>

        <div class="sticky bottom-0 bg-inherit">
            <Modal>
                <a slot="open-modal" class="mt-2.5 mb-5 py-2 px-4 flex justify-center items-center bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-full"
                        type="button">
                    <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                         xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 4v16m8-8H4" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
                    </svg>
                </a>
                <div slot="box" let:componentId>
                    <div class="form-control w-full max-w-xs">
                        <label class="label">
                            <span class="label-text">输入BV或av号可编辑现有稿件</span>
                            <span class="label-text-alt">av971158452</span>
                        </label>
                        <input type="text" bind:value={tempName} placeholder="{'未命名模板' + Object.keys($template).length}" class="input input-bordered w-full max-w-xs" />
                        <label class="label">
                            <span class="label-text-alt">输入其他将新建投稿模板</span>
                            <span class="label-text-alt">BV1ip4y1x7Gi</span>
                        </label>
                    </div>

                    <div class="modal-action">
                        <label for="{componentId}" on:click={add} class="btn btn-accent">添加模板</label>
                        <label for="{componentId}" class="btn">Close</label>
                    </div>
                </div>
            </Modal>
        </div>
    </div>
</div>

<!-- TODO: enable this -->
<style lang="postcss">
    .selected {
        @apply text-gray-700 bg-gray-200;
    }

    nav > a {
        @apply flex cursor-pointer items-center px-3 py-2 mt-1 text-gray-600 transition-colors duration-200 transform rounded-md hover:bg-gray-200 hover:text-gray-700;
    }

</style>
