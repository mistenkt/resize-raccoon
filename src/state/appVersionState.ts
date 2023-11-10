import { signal } from "@preact/signals-react";

const state = signal<string>('');

export const getAppVersion = (): Readonly<string> => state.value;

export const setAppVersion = (newVersion: string) => state.value = newVersion;