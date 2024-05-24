export const INVOKE_COMMANDS = {
    getOthersMyinfo: "get_others_myinfo",
}

export const contentLimitation = {
    titleLength: 80,
    videoPartTitleLength: 80,  // actively limit the length at Progress.svelte
    reprintUrlLength: 200,
    descriptionLengthByZone: (tid: number) => {
        const descV2Enabled = [
            17, 171, 172, 65, 173, 121, 136, 19,  // 游戏区
            201, 124, 228, 207, 208, 209, 229, 122,  // 知识区
            31, 59, 29, 30,  // 部分音乐区
            24, 25, 47, 253, 27,  // 部分动画区
            95, 230, 231, 232, 233,  // 科技区
            20, 154, 156,  // 部分舞蹈区
        ]
        if (descV2Enabled.includes(tid)) return 2000;
        
        return 250;
    },
    individualTagLength: 20,
    tagsCount: 12,
    dynamicMessageLength: 233,
}

export enum CopyrightType {
    original = 1,
    reprint = 2,
}
