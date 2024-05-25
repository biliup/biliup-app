<script lang="ts">
    import Append from './Append.svelte';
    import {currentTemplate, save_config, template} from './store';
    import {invoke} from "@tauri-apps/api/core";
    import {archivePre, createPop, partition} from "./common";
    import {contentLimitation, CopyrightType} from "./lib/constants";
    import FilePond, {registerPlugin} from 'svelte-filepond';
    import {fly} from 'svelte/transition';
    import {flip} from 'svelte/animate';
    // Import the Image EXIF Orientation and Image Preview plugins
    // Note: These need to be installed separately
    // `npm i filepond-plugin-image-preview filepond-plugin-image-exif-orientation --save`
    import FilePondPluginImageExifOrientation from 'filepond-plugin-image-exif-orientation';
    import FilePondPluginImagePreview from 'filepond-plugin-image-preview';
    import FilePondPluginFileValidateType from 'filepond-plugin-file-validate-type';
    import 'filepond/dist/filepond.css';
    import 'filepond-plugin-image-edit/dist/filepond-plugin-image-edit.css';
    import "filepond-plugin-image-preview/dist/filepond-plugin-image-preview.css";

    import {fetch} from "@tauri-apps/plugin-http";
    import type {SelectedTemplate} from "./global";

    export let selected: string;
    export let selectedTemplate: SelectedTemplate;
    let oldSelected = selected;
    // let title: string = ;

    let noReprint = true;

    let edit = false;

    function update(e) {
        if (!e) {
            console.log(oldSelected);
            delete $template[oldSelected];
            oldSelected = selected
            selectedTemplate.changed = true;
            $template[selected] = selectedTemplate;
            console.log($template);
        }
        edit = e;
    }

    async function deleteTemplate() {
        if (!(await confirm(`确定要移除模板 ${selected} 吗？`))) { // confirm() is returning a Promise
            return;
        }

        let len = Object.keys($template).length;
        const keys = Object.keys($template);
        const index = keys.indexOf(selected);
        if (len==1){
            createPop("已经是最后一个模板无法删除");
            return;
        }
        delete $template[selected];
        if (len > 1) {
            selected = keys[index + 1] || keys[index - 1];
            $currentTemplate.selectedTemplate = $template[selected];
            $currentTemplate.current = selected;
        } else {
            selected = '';
            $currentTemplate.selectedTemplate = null;
            $currentTemplate.current = '';
        }
        $template = $template;
        console.log($template);
        await save_config((ret) => {
            ret.streamers = $template;
        })
        createPop('移除成功', 2000, 'Success');
    }

    async function saveTemplates() {
        // console.log({[selected]: config});
        await save_config((ret) => {
            ret.streamers = $template;
        })
        selectedTemplate.changed = false;
        $template = $template;
        createPop('模板保存成功', 5000, 'Success');
    }

    // console.log()
    let tags = selectedTemplate?.tag ? selectedTemplate?.tag.split(',') : [];
    // $: tags = selectedTemplate?.tag.split(',');

    let parent = '请选择';
    let children = '分区';
    let current;
    let currentChildren;
    $: {
        if ($partition) {
            // tags.flatMap()
            // $partition.flatMap()
            let changed = false;
            for (const partitionElement of $partition) {
                for (const child of partitionElement.children) {
                    if (child.id === selectedTemplate.tid) {
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
    }

    let tagInput: string;
    let autoSubmit = false;
    $: autoSubmit = !!selectedTemplate?.submitCallback;
    function submitCallback() {
        console.log("submitCallback()");
        console.log("selectedTemplate", selectedTemplate);

        selectedTemplate.videos = selectedTemplate.files;
        let dtime = null;
        if (isDtime) {
            dtime = new Date(`${date} ${time}`).valueOf()/1000;
        }

        if (selectedTemplate.desc){
            // <input maxlength={} /> will only limits new inputs, if limitation changes, the existing value will not be automatically truncated
            selectedTemplate.desc = selectedTemplate.desc.substring(0, contentLimitation.descriptionLengthByZone(selectedTemplate.tid));
        }

        let invokeMethod;
        let msg;
        let hires_params = {};
        if (selected?.length > 2 && (selected.startsWith('av') || selected.startsWith('BV'))) {
            invokeMethod = 'edit_video';
            msg = '编辑';
        }else {
            invokeMethod = 'submit';
            msg = '投稿';
            hires_params = { lossless_music: isHiRes ? 1 : 0 };
        }
        let invokeArgs = {
            studio: {
                ...selectedTemplate,
                tag: tags.join(','),
                dtime: dtime,
                no_reprint: $currentTemplate.selectedTemplate.copyright == CopyrightType.original && noReprint ? 1 : 0,
                ...hires_params,
            }
        };
        console.log("invokeArgs", invokeArgs);
        invoke(invokeMethod, invokeArgs)
        .then((res) => {
            console.log("res", res);
            if (typeof res == "object" && "bvid" in res) {
                createPop(`${selected} - ${msg}成功: ${res.bvid}`, 5000, 'Success');
            }
            else if (typeof res == "object" && "data" in res
                && typeof res.data == "object" && "bvid" in res.data
            ) {
                createPop(`${selected} - ${msg}成功: ${res.data.bvid}`, 5000, 'Success');
            }
            else {
                createPop(`${selected} - ${msg}成功：${res}`, 5000, 'Success');
            }

            lastSubmissionTime = new Date();
        }).catch((e) => {
                createPop(e, 5000);
                console.log(e);
            }
        );
    }
    function submit() {
        if (selectedTemplate.atomicInt === 0) {
            return submitCallback();
        }
        selectedTemplate.submitCallback = submitCallback;
        autoSubmit = true;
    }

    function cancelSubmit() {
        selectedTemplate.submitCallback = null;
        autoSubmit = false;
    }

    function handleTagEnter() {
        if (!tagInput) {  // otherwise the new tag content is "undefined" or "null"
            return;
        }

        if (tagInput.length > contentLimitation.individualTagLength) {
            createPop(`标签长度超过${contentLimitation.individualTagLength}个字符，无法添加`, 5000);
            return;
        }

        if (tags.includes(tagInput)) {
            createPop("已有相同标签", 5000);
            tagInput = "";
            return;
        }
        if (tags.length > contentLimitation.tagsCount) {
            createPop(`标签数量超过${contentLimitation.tagsCount}个，无法添加`, 5000);
            tagInput = "";
            return;
        }

        tags = [...tags, tagInput];
        selectedTemplate.tag = tags.join(',');
        tagInput = "";
        return false;
    }

    function removeTag(tag) {
        tags = tags.filter(t => t !== tag);
        selectedTemplate.tag = tags.join(',');
        console.log(tag);
    }


    function callback(detailTid, detailParent, detailChildren) {
        selectedTemplate.tid = detailTid;
        parent = detailParent;
        children = detailChildren;
    }
    if (selectedTemplate.dtime === 0) {
        selectedTemplate.dtime = null;
    }
    let dtime;
    let isDtime = selectedTemplate.dtime !== null;
    let date;
    let time;

    if (isDtime) {
        dtime = new Date(selectedTemplate.dtime * 1000);
        date = dtime.getFullYear() + '-' + pad(dtime.getMonth() + 1) + '-' + pad(dtime.getDate());
        time = pad(dtime.getHours()) + ':' + pad(dtime.getMinutes());
    }

    //pad a value with leading zeros
    function pad(value) {
        return ('00'+value).slice(-2);
    }


    $: if (isDtime) {
        selectedTemplate.dtime = new Date(`${date} ${time}`).valueOf()/1000;
    } else {
        selectedTemplate.dtime = null;
    }

    let isHiRes = selectedTemplate.lossless_music;
    let hiResFieldDisabled = false;
    if (selected?.length > 2 && (selected.startsWith('av') || selected.startsWith('BV'))) {
        hiResFieldDisabled = true;
    }

    // Register the plugins
    registerPlugin(
        FilePondPluginFileValidateType,
        FilePondPluginImageExifOrientation,
        FilePondPluginImagePreview,
    );

    // a reference to the component, used to call FilePond methods
    let pond;
    let labelIdle = `<span class="filepond--label-action">选择</span>并上传视频封面`;
    // pond.getFiles() will return the active files

    // the name to use for the internal file input
    let name = 'filepond';

    // handle filepond events
    function handleInit() {
        console.log('FilePond has initialised');
    }

    function handleRemoveFile(err, fileItem) {
        selectedTemplate.cover = '';
        console.log('A file has been removed', fileItem);
    }
    let filepondServer = {
        process: (fieldName, file: File, metadata, load, error, progress, abort, transfer, options) => {
            progress(false, 0, 0);

            file.arrayBuffer().then((buffer) => {
                return invoke('cover_up', {
                    input: Array.prototype.slice.call(new Uint8Array(buffer))
                });
            }).then((res) => {
                console.log(`${selectedTemplate.title} cover+${selected}`, res);
                selectedTemplate.cover = res;
                load(res);
            }).catch((e) => {
                error(e);
                createPop(e, 5000);
                console.log(e);
            });
            // Should expose an abort method so the request can be cancelled
            return {
                abort: () => {
                    // This function is entered if the user has tapped the cancel button
                    // request.abort();

                    // Let FilePond know the request has been cancelled
                    abort();
                },
            };
        },

        load: (source, load, error, progress, abort, headers) => {
            console.log("load: (source, load, error, progress, abort, headers)", source);

            progress(false, 0, 0);

            // Should call the load method with a file object or blob when done
            (async () => {
                try {
                    const res = await fetch(source, {method: "GET"});
                    console.log("fetch(source, {method: 'GET'}) => res", res);
                    load(await res.blob());
                } catch (e) {
                    error(e);
                    createPop(e, 5000);
                    console.log(e);
                }
            })();

            // Should expose an abort method so the request can be cancelled
            return {
                abort: () => {
                    // User tapped cancel, abort our ongoing actions here

                    // Let FilePond know the request has been cancelled
                    abort();
                },
            };
        },
    };
    let uploadedCover = selectedTemplate?.cover ? [{
        // the server file reference
        source: selectedTemplate.cover,

        // set type to local to indicate an already uploaded file
        options: {
            type: 'local',
        },
    }] : null;

    let lastSubmissionTime: Date;

    let inputsAreValid: boolean;
    $: {
        selectedTemplate; // for reactivity
        tags; // for reactivity
        inputsAreValid = checkInputFields(false);
    }
    let inputsViolations: string[];

    function checkInputFields(createPopup: boolean): boolean {
        let canSubmit = true;
        let currentInputViolations = [];

        if (selectedTemplate.title.length > contentLimitation.titleLength) {
            if (createPopup){
                createPop(`标题长度超过${contentLimitation.titleLength}个字符，无法提交，当前为${selectedTemplate.title.length}个字符`, 5000);
            } else {
                currentInputViolations.push(`标题长度超过${contentLimitation.titleLength}个字符，无法提交，当前为${selectedTemplate.title.length}个字符`);
            }
            canSubmit = false;
        }

        if (selectedTemplate.title.length === 0) {
            // if (createPopup) createPop(`标题不能为空`, 5000);
            if (createPopup) {
                createPop(`标题不能为空`, 5000);
            } else {
                currentInputViolations.push(`标题不能为空`);
            }
            canSubmit = false;
        }

        if ($currentTemplate.selectedTemplate.copyright == CopyrightType.reprint && selectedTemplate.source.length > contentLimitation.reprintUrlLength){
            // if (createPopup) createPop(`转载来源长度超过${contentLimitation.reprintUrlLength}个字符，无法提交，当前为${selectedTemplate.source.length}个字符`, 5000);
            if (createPopup) {
                createPop(`转载来源长度超过${contentLimitation.reprintUrlLength}个字符，无法提交，当前为${selectedTemplate.source.length}个字符`, 5000);
            } else {
                currentInputViolations.push(`转载来源长度超过${contentLimitation.reprintUrlLength}个字符，无法提交，当前为${selectedTemplate.source.length}个字符`);
            }
            canSubmit = false;
        }

        if ($currentTemplate.selectedTemplate.copyright == CopyrightType.reprint && selectedTemplate.source.length === 0){
            if (createPopup) {
                createPop(`转载来源不能为空`, 5000);
            } else {
                currentInputViolations.push(`转载来源不能为空`);
            }
            canSubmit = false;
        }

        if (tags.length > contentLimitation.tagsCount) {
            // if (createPopup) createPop(`标签数量超过${contentLimitation.tagsCount}个，无法提交，当前为${tags.length}个`, 5000);
            if (createPopup) {
                createPop(`标签数量超过${contentLimitation.tagsCount}个，无法提交，当前为${tags.length}个`, 5000);
            } else {
                currentInputViolations.push(`标签数量超过${contentLimitation.tagsCount}个，无法提交，当前为${tags.length}个`);
            }
            canSubmit = false;
        }

        if (tags.length === 0) {
            if (createPopup) {
                createPop(`标签不能为空`, 5000);
            } else {
                currentInputViolations.push(`标签不能为空`);
            }
            canSubmit = false;
        }

        if (selectedTemplate.desc.length > contentLimitation.descriptionLengthByZone(selectedTemplate.tid)) {
            if (createPopup) {
                createPop(`简介长度超过${contentLimitation.descriptionLengthByZone(selectedTemplate.tid)}个字符，无法提交，当前为${selectedTemplate.desc.length}个字符`, 5000);
            } else {
                currentInputViolations.push(`简介长度超过${contentLimitation.descriptionLengthByZone(selectedTemplate.tid)}个字符，无法提交，当前为${selectedTemplate.desc.length}个字符`);
            }
            canSubmit = false;
        }

        if (selectedTemplate.dynamic.length > contentLimitation.dynamicMessageLength) {
            if (createPopup) {
                createPop(`粉丝动态长度超过${contentLimitation.dynamicMessageLength}个字符，无法提交，当前为${selectedTemplate.dynamic.length}个字符`, 5000);
            } else {
                currentInputViolations.push(`粉丝动态长度超过${contentLimitation.dynamicMessageLength}个字符，无法提交，当前为${selectedTemplate.dynamic.length}个字符`);
            }
            canSubmit = false;
        }

        if (selectedTemplate.files.length === 0) {
            if (createPopup) {
                createPop(`新增稿件分P不能为空`, 5000);
            } else {
                currentInputViolations.push(`新增稿件分P不能为空`);
            }
            canSubmit = false;
        }

        selectedTemplate.files.forEach((file, index) => {
            if (file.title.length > contentLimitation.titleLength) {
                if (createPopup) {
                    createPop(`P${index+1} ${file.title} 名称长度超过${contentLimitation.titleLength}个字符，无法提交，当前为${file.title.length}个字符`, 5000);
                } else {
                    currentInputViolations.push(`P${index+1} ${file.title} 名称长度超过${contentLimitation.titleLength}个字符，无法提交，当前为${file.title.length}个字符`);
                }
                canSubmit = false;
            }
        });

        inputsViolations = currentInputViolations;
        return canSubmit;
    }
</script>
<div in:fly="{{ y: 200, duration: 400 }}">
    <div class="px-6 pt-3 pb-10 my-2 mr-12" >
        <div class="space-y-3">
            <div class="flex justify-between">
                <label class="text-lg font-bold tracking-wide mb-2">
                    {#if (edit)}
                        <input on:focusout={()=> update(false)} bind:value={selected}
                               class="w-full p-1 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500"
                               placeholder="标题">
                    {:else}
                        <div class="p-1">
                            {selected}
                            <svg on:click={()=> update(true)}
                                 xmlns="http://www.w3.org/2000/svg"
                                 class="cursor-pointer inline h-5 w-5 hover:text-blue-700" fill="none"
                                 viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                      d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"/>
                            </svg>
                        </div>
                    {/if}
                </label>
                <div class="flex flex-row-reverse">
                    <button class="ml-2 py-2 px-2 flex justify-center items-center bg-red-600 hover:bg-red-700 focus:ring-red-500 focus:ring-offset-red-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  w-8 h-8 rounded-lg " on:click|preventDefault={deleteTemplate}
                            type="button">
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                             xmlns="http://www.w3.org/2000/svg">
                            <path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" stroke-linecap="round" stroke-linejoin="round"
                                  stroke-width="2"/>
                        </svg>
                    </button>
                    <button class="py-2 px-2 flex justify-center items-center bg-blue-600 hover:bg-blue-700 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  w-8 h-8 rounded-lg " on:click|preventDefault={saveTemplates}
                            type="button">
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                             xmlns="http://www.w3.org/2000/svg">
                            <path d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" stroke-linecap="round" stroke-linejoin="round"
                                  stroke-width="2"/>
                        </svg>
                    </button>
                </div>
            </div>
            {#if selectedTemplate.title.length <= contentLimitation.titleLength}
                <input bind:value={selectedTemplate.title}
                       class="bg-[#f9fcfd] w-full text-base p-2 rounded-lg focus:outline-none focus:border-indigo-500 border border-gray-300"
                       placeholder="标题，长度限制{contentLimitation.titleLength}个字符" />
            {:else}
                <input bind:value={selectedTemplate.title}
                       class="w-full text-base p-2 rounded-lg focus:outline-none focus:border-indigo-500 bg-red-100 border border-red-300"
                       placeholder="标题，长度限制{contentLimitation.titleLength}个字符" />
            {/if}

            <Append bind:selectedTemplate={selectedTemplate}/>
            <p class="text-sm text-gray-300">
                File type: .mp4,.flv,.avi,.wmv,.mov,.webm,.mpeg4,.ts,.mpg,.rm,.rmvb,.mkv,.m4v
            </p>
            <div class="app">
                <FilePond bind:this={pond} {name}
                          labelIdle="{labelIdle}"
                          server="{filepondServer}"
                          files="{uploadedCover}"
                          credits="{false}"
                          onremovefile="{handleRemoveFile}"
                          acceptedFileTypes="image/png, image/jpeg, image/gif"
                          />
            </div>
            <div class="bg-[#fafcfd] border rounded-md px-2 py-1">
                <div>
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text font-bold">自制</span>
                            <input type="radio" name="radio-10" class="radio checked" checked={$currentTemplate.selectedTemplate.copyright === CopyrightType.original} on:click={()=>{$currentTemplate.selectedTemplate.copyright = CopyrightType.original}} />
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text font-bold">转载</span>
                            <input type="radio" name="radio-10" class="radio checked" checked={$currentTemplate.selectedTemplate.copyright === CopyrightType.reprint} on:click={()=>{$currentTemplate.selectedTemplate.copyright = CopyrightType.reprint}} />
                        </label>
                    </div>
                </div>

                {#if $currentTemplate.selectedTemplate.copyright === CopyrightType.original}
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text">自制声明：未经作者授权 禁止转载</span>
                            <input type="checkbox" bind:checked="{noReprint}" class="checkbox">
                        </label>
                    </div>
                {:else}
                    {#if selectedTemplate.source.length <= contentLimitation.reprintUrlLength}
                        <input bind:value={selectedTemplate.source} class="input w-full" placeholder="转载来源" type="text"/>
                    {:else}
                        <input bind:value={selectedTemplate.source} class="input w-full bg-red-100 border border-red-300" placeholder="转载来源" type="text"/>
                    {/if}
                {/if}
            </div>


            <div class="flex w-52" use:archivePre={{callback, current, currentChildren}}>
                <button class="border border-gray-300 relative w-full bg-white rounded-md pl-3 pr-10 py-3 text-left cursor-default focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                        type="button">
                    <span class="flex items-center">
                        <span class="ml-1 block truncate">
                            {parent} → {children}
                        </span>
                    </span>
                    <span class="ml-3 absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                        <svg aria-hidden="true" class="h-5 w-5 text-gray-400" fill="currentColor"
                             viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path clip-rule="evenodd"
                                  d="M10 3a1 1 0 01.707.293l3 3a1 1 0 01-1.414 1.414L10 5.414 7.707 7.707a1 1 0 01-1.414-1.414l3-3A1 1 0 0110 3zm-3.707 9.293a1 1 0 011.414 0L10 14.586l2.293-2.293a1 1 0 011.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
                                  fill-rule="evenodd">
                            </path>
                        </svg>
                    </span>
                </button>
                <!--                <input bind:this={archivePre} bind:value={tid} type="text" class=" rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent" placeholder="分区"/>-->
            </div>
            <div class="flex flex-wrap rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent">
                {#each tags as tag(tag)}
                    <span animate:flip="{{duration: 300}}" class="flex  ml-1 my-1.5 px-3 py-0.5 text-base rounded-full text-white  bg-indigo-500 ">
                        {tag}
                        <button on:click={(e)=>{removeTag(tag)}} class="bg-transparent hover">
                            <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" fill="currentColor"
                                 class="ml-2" viewBox="0 0 1792 1792">
                                <path d="M1490 1322q0 40-28 68l-136 136q-28 28-68 28t-68-28l-294-294-294 294q-28 28-68 28t-68-28l-136-136q-28-28-28-68t28-68l294-294-294-294q-28-28-28-68t28-68l136-136q28-28 68-28t68 28l294 294 294-294q28-28 68-28t68 28l136 136q28 28 28 68t-28 68l-294 294 294 294q28 28 28 68z">
                                </path>
                            </svg>
                        </button>
                    </span>
                {/each}

                <input bind:value={tagInput} class="outline-none rounded-lg flex-1 appearance-none  w-full py-2 px-4 bg-white text-gray-700 placeholder-gray-400 shadow-sm text-base " on:keypress={e=>e.key==='Enter' && handleTagEnter()}
                       placeholder="标签，回车输入"
                       type="text"
                />
            </div>
            <div class="text-gray-700">
                <!-- Svelte: A11y: A form label must be associated with a control. -->
                <div class="label">
                    <span class="text-sm font-bold text-gray-500 tracking-wide">
                        简介
                        <sub>{selectedTemplate.desc.length}/{contentLimitation.descriptionLengthByZone(selectedTemplate.tid)}</sub>
                    </span>
                </div>
                {#if selectedTemplate.desc.length <= contentLimitation.descriptionLengthByZone(selectedTemplate.tid)}
                    <textarea bind:value={selectedTemplate.desc}
                              class="textarea textarea-bordered w-full"
                              cols="40" rows="4" placeholder="简介补充: ..."
                    ></textarea>
                {:else}
                    <textarea bind:value={selectedTemplate.desc}
                              class="textarea textarea-bordered w-full bg-red-100 border border-red-300"
                              cols="40" rows="4" placeholder="简介补充: ..."
                    ></textarea>
                {/if}
            </div>
            <div class="text-gray-700">
                <!-- Svelte: A11y: A form label must be associated with a control. -->
                <div class="label">
                    <span class="text-sm font-bold text-gray-500 tracking-wide">
                        粉丝动态
                        <sub>{selectedTemplate.dynamic.length}/{contentLimitation.dynamicMessageLength}</sub>
                    </span>
                </div>
                {#if selectedTemplate.dynamic.length <= contentLimitation.dynamicMessageLength}
                    <textarea bind:value={selectedTemplate.dynamic}
                              class="textarea textarea-bordered w-full"
                              cols="40" rows="1" placeholder="动态描述"
                    ></textarea>
                {:else}
                    <textarea bind:value={selectedTemplate.dynamic}
                              class="textarea textarea-bordered w-full bg-red-100 border border-red-300"
                              cols="40" rows="1" placeholder="动态描述"
                    ></textarea>
                {/if}
            </div>
            <div class="flex items-center">
                <input type="checkbox" class="toggle my-2" bind:checked="{isDtime}">
                <span class="ml-2 text-sm font-bold text-gray-500 tracking-wide">开启定时发布</span>
                {#if (isDtime)}
                    <input class="mx-3 border rounded-lg border-gray-300 py-1 px-2" type="date" bind:value={date}/>
                    <input class="mx-3 border rounded-lg border-gray-300 py-1 px-2" type="time" bind:value={time}/>
                {/if}
            </div>
            {#if (!hiResFieldDisabled)}
            <div class="flex items-center">
                <input type="checkbox" class="toggle my-2" bind:checked="{isHiRes}">
                <span class="ml-2 text-sm font-bold text-gray-500 tracking-wide">Hi-Res无损音质</span>
            </div>
            {/if}
            {#if (autoSubmit)}
                <div class="flex justify-center items-center">
                    <button type="button" class="inline-flex items-center px-4 py-2 font-semibold leading-6 text-sm shadow rounded-md text-white bg-indigo-500 hover:bg-indigo-400 transition ease-in-out duration-150 cursor-not-allowed" disabled>
                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        等待视频上传完后会自动提交...
                    </button>

                    <!-- svelte-ignore a11y-missing-attribute -->
                    <a class="cursor-pointer" on:click={cancelSubmit}>
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-red-400 hover:stroke-rose-500 transition ease-in-out duration-150 ml-2.5 h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </a>
                </div>
            {:else}
                {#if !inputsAreValid}
                    <h3 class="text-center text-xl font-extrabold text-red-500 decoration-8">部分内容长度不符合要求，请谨慎提交</h3>
                    {#each inputsViolations as violation}
                        <p class="text-center text-red-500">{violation}</p>
                    {/each}
                {/if}
                <button class="p-2 my-5 w-full flex justify-center bg-blue-500 text-gray-100 rounded-full tracking-wide
                          font-semibold  focus:outline-none focus:shadow-outline hover:bg-blue-600 shadow-lg cursor-pointer transition ease-in duration-300" on:click|preventDefault={submit} type="submit">
                    提交视频
                </button>
            {/if}
            {#if lastSubmissionTime}
                <div class="text-sm text-gray-500 text-center">
                    <p>上次提交时间: {lastSubmissionTime?.toLocaleString()}</p>
                </div>
            {/if}
        </div>
    </div>
</div>
