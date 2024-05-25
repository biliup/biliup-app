/// <reference types="svelte" />

export type SelectedTemplate = {
    title: string,
    copyright: number,
    source: string,
    tid: number,
    desc: string,
    dynamic: string,
    files: {
        complete: boolean,
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

export type BiliupConfig = {
    limit: number,
    line: string,
    streamers: {
        [key: string]: {
            aid: number,
            copyright: number,
            cover: string,
            desc: string,
            desc_format_id: number,
            dolby: number,
            dtime: any,
            dynamic: string,
            interactive: number,
            lossless_music: number,
            mission_id: number,
            no_reprint: number,
            open_elec: boolean | null,
            open_subtitle: string,
            source: string,
            subtitle: {lan: string, open: number}
            tag: string,
            tid: number,
            title: string
            up_close_danmu: boolean,
            up_close_reply: boolean,
            up_selection_reply: boolean,
        }
    },
    user: any,
    checkInputsBeforeSubmit: boolean,
}
