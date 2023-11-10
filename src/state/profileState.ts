import { signal } from '@preact/signals-react';
import {Profile} from '../types/ProfileTypes';
import { produce } from 'immer';
import backend from '../utils/backend';

const state = signal<Profile[]>([]);

export const getProfiles = (): Readonly<Profile[]> => state.value;

export const setProfiles = (newProfiles: Profile[]) => {
    state.value = produce(state.value, () => newProfiles);
}

export const removeProfile = (uuid: string) => setProfiles(state.value.filter(profile => profile.uuid !== uuid));
export const refreshProfiles = async () => {
    const profiles = await backend.profile.all();
    setProfiles(profiles);
}

export const updateProfile = (updatedProfile: Partial<Profile>) => {
    state.value = produce(state.value, draft => {
        const index = draft.findIndex(profile => profile.uuid === updatedProfile.uuid);
        draft[index] = {
            ...draft[index],
            ...updatedProfile
        }
    });
}

backend.profile.all().then(setProfiles).catch(console.error);