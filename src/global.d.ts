/// <reference types="svelte" />

export type SelectedTemplate = {
    title: string,
    copyright: number,
    source: string,
    tid: number,
    desc: string,
    dynamic: string,
    files: {
        completed: boolean,
        desc: string,
        filename: string,
        id: string,
        process: boolean,
        progress: number,
        speed: number,
        speed_uploaded: number,
        title: string,
        totalSize: number,
        uploaded: number,
    }[],
    [key: string]: any;
};
