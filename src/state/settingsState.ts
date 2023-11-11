import { signal } from "@preact/signals-react";
import Settings from "../types/SettingsType";
import {produce} from 'immer';
import backend from "../utils/backend";

const state = signal<Settings>({} as Settings);

export const updateSettings = (updatedSettings: Partial<Settings>) => {
    let oldSettings = produce(state.value, draft => draft);

    state.value = produce(state.value, draft => ({
        ...draft,
        ...updatedSettings
    }));

    backend.settings.update(state.value).catch(() => {
        setSettings(oldSettings);
    });
}

export const setSettings = (newSettings: Settings) => {
    state.value = produce(state.value, () => newSettings);
}

export const getSettings = (): Readonly<Settings> => state.value;