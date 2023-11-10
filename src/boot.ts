import { getVersion } from "@tauri-apps/api/app";
import backend from "./utils/backend";
import { setSettings } from "./state/settingsState";
import { setAppVersion } from "./state/appVersionState";
import { BootState, setBootState } from "./state/bootState";
import { batch } from "@preact/signals-react";

Promise.all([
    backend.settings.all(),
    getVersion(),
]).then(([settings, version]) => batch(() => {   
    setSettings(settings);
    setAppVersion(version);
    setBootState(BootState.READY);
}));