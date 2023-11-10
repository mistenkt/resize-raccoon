import { signal } from "@preact/signals-react";
import { Screen } from "../types/ScreenTypes";
import { produce } from "immer";

interface ScreenState {
    screen: Screen;
    params?: any;
}

const state = signal<ScreenState>({screen: Screen.HOME});

export const getScreen = (): Readonly<Screen> => state.value.screen;
export const getScreenParams = () => state.value.params || {};
export const setScreen = (newScreen: Screen, params?: any) => {
    state.value = produce(state.value, () => ({
        screen: newScreen,
        params
    }));
}