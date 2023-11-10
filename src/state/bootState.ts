import { signal } from "@preact/signals-react";

export enum BootState {
    BOOTING = 'BOOTING',
    READY = 'READY',
}

const state = signal<BootState>(BootState.BOOTING);

export const getBootState = (): Readonly<BootState> => state.value;
export const setBootState = (newState: BootState) => state.value = newState;