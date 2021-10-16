import {writable} from "svelte/store";
import Pop from "./Pop.svelte";
import {check_outros, group_outros, transition_out} from "svelte/internal";
let map = new Map();
map.set('未命名模板', {'title': '', 'files': []})
map.set('未命名模板1', {'title': '', 'files': []})

export const isLogin = writable(false);
export const template = writable(map);
export const currentTemplate = writable({});

export function attach(files) {
    currentTemplate.update(temp => {
        function findFile(file) {
            return temp['files'].find(function(existingFile) {
                return (
                    existingFile.name === file.name &&
                    existingFile.lastModified === file.lastModified &&
                    existingFile.size === file.size &&
                    existingFile.type === file.type
                )
            })
        }

        for ( const file of  files) {
            // file.type.match
            console.log(file);
            if (!file.type.match("video.*")){
                createPop('请上传视频文件！');
                continue;
            }
            if (findFile(file)) {
                createPop('请上传非重复视频！');
                continue;
            }
                // temp['files'] = [...temp['files'], ...event.target.files];
            temp['files'].push(file);
            let objectURL = URL.createObjectURL(file);
            console.log(objectURL);
        }
        return temp;
    });
}

function createPop(msg) {
    const pop = new Pop({
        target: document.querySelector('#alerts'),
        intro: true,
        props: {
            msg: msg
        }
    });
    setTimeout(()=>outroAndDestroy(pop), 3000 );
}

// Workaround for https://github.com/sveltejs/svelte/issues/4056
const outroAndDestroy = (instance) => {
    if (instance.$$.fragment && instance.$$.fragment.o) {
        group_outros();
        transition_out(instance.$$.fragment, 0, 0, () => {
            instance.$destroy();
        });
        check_outros();
    } else {
        instance.$destroy();
    }
};
