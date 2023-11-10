import { invoke } from "@tauri-apps/api/tauri";
import caseConvert from "./caseConvert";
import { Profile } from "../types/ProfileTypes";
import Process from "../types/ProcessType";
import Settings from "../types/SettingsType";
import { ToastType } from "../components/toast/toast.types";
import { addToast } from "../components/toast/ToastSystem";
import translate from "./i18n/translate";
import TranslationKeys from "./i18n/TranslationKeys";

const invokeWithToast = async (command: string, params?: any): Promise<any> => {
    let success;
    let error: any;

    const invokePromise = invoke(command, params)
        .then(response => {
            success = response;
        })
        .catch(err => {
            error = err;
        });

    const timeoutPromise = new Promise(resolve => setTimeout(resolve, 500));
    await Promise.all([invokePromise, timeoutPromise]);

    if(success || !error) {
        return success
    }

    const [errorDomain, errorType] = error.error?.split('.');

    const message = errorDomain && errorType ? translate(`errors.${errorDomain}.${errorType}` as TranslationKeys) : error?.message;
    
    addToast({
        type: ToastType.ERROR,
        message
    });
};

const profile = {
    all: async () => {
        const profiles: any = await invokeWithToast("profile_get");
        return profiles.map(caseConvert.toCamel) as Profile[];
    },
    apply: async (profile: Profile) => invokeWithToast("profile_apply", { profile: caseConvert.toSnake(profile) }),
    update: async (profile: Profile) => invokeWithToast("profile_update", { profile: caseConvert.toSnake(profile) }),
    add: async (profile: Profile) => invokeWithToast("profile_add", { profile: caseConvert.toSnake(profile) }),
    delete: async (profile: Profile) => invokeWithToast("profile_delete", { profile: caseConvert.toSnake(profile) }),
}

const process = {
    running: async () => {
        const processes: any = await invokeWithToast("process_get");
        return processes.map(caseConvert.toCamel) as Process[];
    }
}

const settings = {
    all: async (): Promise<Settings> => {
        const settings: Settings = await invokeWithToast("settings_get");
        return caseConvert.toCamel(settings);
    },
    update: async (settings: Settings) => invokeWithToast("settings_update", {
        settings: caseConvert.toSnake(settings)
    })
}




const backend = {
    profile,
    process,
    settings
}

export default backend;