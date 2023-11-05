import { invoke } from "@tauri-apps/api/tauri";
import caseConvert from "./caseConvert";
import { Profile } from "../types/ProfileTypes";
import Process from "../types/ProcessType";
import Settings from "../types/SettingsType";


const profile = {
    all: async () => {
        const profiles: any = await invoke("profile_get");
        return profiles.map(caseConvert.toCamel) as Profile[];
    },
    apply: async (profile: Profile) => invoke("profile_apply", { profile: caseConvert.toSnake(profile) }),
    update: async (profile: Profile) => invoke("profile_update", { profile: caseConvert.toSnake(profile) }),
    add: async (profile: Profile) => invoke("profile_add", { profile: caseConvert.toSnake(profile) }),
    delete: async (profile: Profile) => invoke("profile_delete", { profile: caseConvert.toSnake(profile) }),
}

const process = {
    running: async () => {
        const processes: any = await invoke("process_get");
        return processes.map(caseConvert.toCamel) as Process[];
    }
}

const settings = {
    all: async (): Promise<Settings> => {
        const settings: Settings = await invoke("settings_get");
        return caseConvert.toCamel(settings);
    },
    update: async (settings: Settings) => invoke("settings_update", {
        settings: caseConvert.toSnake(settings)
    })
}




const backend = {
    profile,
    process,
    settings
}

export default backend;