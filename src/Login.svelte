<script lang="ts">
    import { isLogin, createPop } from './store.js';
    import { scale, fade, blur } from 'svelte/transition';
    import { invoke } from '@tauri-apps/api/tauri';

    let rememberMe: boolean = false;
    let username;
    let password;
    // invoke('login_by_cookie', )
    //     .then((res) => {
    //         isLogin.set(true);
    //         console.log(`Message: ${res}`)
    //     }).catch((e) => console.log(e))
    invoke('load_account', )
        .then((res) => {
            username = res.account.username;
            password = res.account.password;
            rememberMe = true;
            isLogin.set(true);
            console.log(res);
        }).catch((e) => console.log(e, 5000))
    function login() {
        console.log(rememberMe);
        invoke('login', { username: username,password: password, rememberMe: rememberMe  })
            .then((res) => {
                    isLogin.set(true);
                    console.log(`Message: ${res}`)
            }).catch((e) => createPop(e, 5000))
    }

</script>
    <div transition:fade class="abs min-h-screen flex flex-col sm:justify-center items-center bg-white ">
        <div transition:scale class="relative sm:max-w-sm w-full">
            <div class="card bg-blue-400 shadow-lg  w-full h-full rounded-3xl absolute  transform -rotate-6"></div>
            <div class="card bg-red-400 shadow-lg  w-full h-full rounded-3xl absolute  transform rotate-6"></div>
            <div class="relative w-full rounded-3xl  px-10 py-5 bg-white shadow-md">
                <form class="mt-4">
                    <div>
                        <label for="username" class="block text-sm text-gray-800 dark:text-gray-200">用户名</label>
                        <input type="text" bind:value={username}
                               class="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring">
                    </div>

                    <div class="mt-4">
                        <div class="flex items-center justify-between">
                            <label for="password" class="block text-sm text-gray-800 dark:text-gray-200">密码</label>
                        </div>

                        <input type="password" bind:value={password}
                               class="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring">
                    </div>
                    <label class="flex items-center mt-4">
                        <input type="checkbox" class="form-checkbox" bind:checked={rememberMe}/>
                        <span class="block ml-2 text-xs font-medium text-gray-700 cursor-pointer">Remember me</span>
                    </label>
                    <div class="mt-6">
                        <button on:click|preventDefault={login}
                                class="w-full px-4 py-2 tracking-wide text-white transition-colors duration-200 transform bg-gray-700 rounded-md hover:bg-gray-600 focus:outline-none focus:bg-gray-600">
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
