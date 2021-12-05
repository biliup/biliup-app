<script lang="ts">
    import {isLogin} from './store.js';
    import {fade, scale} from 'svelte/transition';
    import {invoke} from '@tauri-apps/api/tauri';
    import {createPop} from "./common";

    let rememberMe: boolean = false;
    let username;
    let password;
    invoke('login_by_cookie',)
        .then((res) => {
            isLogin.set(true);
            console.log(`Message: ${res}`)
        }).catch((e) => console.log(e))
    invoke('load')
        .then((res) => {
            username = res.user.account.username;
            password = res.user.account.password;
            rememberMe = true;
            // isLogin.set(true);
            console.log(res);
        }).catch((e) => {
            let a = '未命名模板';
            invoke('save', {
                config: {
                    user: {
                        account: {
                            username: '',
                            password: ''
                        }
                    },
                    streamers: {
                        ['未命名模板']: {
                            title: '',
                            copyright: 1,
                            source: '',
                            tid: 171,
                            desc: '',
                            dynamic: '',
                            tag: '',
                            cover: '',
                            desc_format_id: 0,
                            subtitle: {
                                open: 0,
                                lan: ''
                            },
                            videos: [],
                            open_subtitle: false
                        }
                    }
                }
            })
                .then((res) => {
                    console.log(res);
                }).catch((e) => {
                createPop(e, 5000);
                console.log(e);
            })
            console.log(e);
        }
    )

    function login() {
        console.log(rememberMe);
        invoke('login', {username: username, password: password, rememberMe: rememberMe})
            .then((res) => {
                isLogin.set(true);
                console.log(`Message: ${res}`)
            }).catch((e) => {
            createPop(e, 5000);
            console.log(e);

            // e = JSON.parse(e);
            // {"code":0,"data":{"cookie_info":null,"message":
            // "本次登录环境存在风险, 需使用手机号进行验证或绑定",
            // "sso":null,"status":2,
            // "token_info":null,
            // "url":"https://passport.bilibili.com/account/mobile/security/managephone/phone/verify?tmp_token=&requestId=&source=risk"},
            // "message":"0","ttl":1}
            // const webview = new WebviewWindow('theUniqueLabel', {
            //     url: e.data.url
            // })
            // createPop(JSON.stringify(e), 5000);
        })
    }

</script>
<div class="abs min-h-screen flex flex-col sm:justify-center items-center bg-white " transition:fade>
    <div class="relative sm:max-w-sm w-full" transition:scale>
        <div class="card bg-blue-400 shadow-lg  w-full h-full rounded-3xl absolute  transform -rotate-6"></div>
        <div class="card bg-red-400 shadow-lg  w-full h-full rounded-3xl absolute  transform rotate-6"></div>
        <div class="relative w-full rounded-3xl  px-10 py-5 bg-white shadow-md">
            <form class="mt-4">
                <div>
                    <label class="block text-sm text-gray-800 dark:text-gray-200" for="username">用户名</label>
                    <input bind:value={username} class="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring"
                           type="text">
                </div>

                <div class="mt-4">
                    <div class="flex items-center justify-between">
                        <label class="block text-sm text-gray-800 dark:text-gray-200" for="password">密码</label>
                    </div>

                    <input bind:value={password} class="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring"
                           type="password">
                </div>
                <label class="flex items-center mt-4">
                    <input bind:checked={rememberMe} class="form-checkbox" type="checkbox"/>
                    <span class="block ml-2 text-xs font-medium text-gray-700 cursor-pointer">Remember me</span>
                </label>
                <div class="mt-6">
                    <button class="w-full px-4 py-2 tracking-wide text-white transition-colors duration-200 transform bg-gray-700 rounded-md hover:bg-gray-600 focus:outline-none focus:bg-gray-600"
                            on:click|preventDefault={login}>
                        登录
                    </button>
                </div>
            </form>
        </div>
    </div>
</div>

<style>
    .abs {
        /*overflow-y: overlay;*/
        margin-right: calc(100% - 100vw);
    }
</style>
