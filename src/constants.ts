import Preset from "./types/PresetType";
import translate from "./utils/i18n/translate";

export const presets: Preset[] = [
    {
        name: translate('profile.preset.options.triple1080p'),
        windowWidth: 5760,
        windowHeight: 1080,
        windowPosY: 0,
        windowPosX: -1920
    },
    {
        name: translate('profile.preset.options.triple1440p'),
        windowWidth: 7680,
        windowHeight: 1440,
        windowPosY: 0,
        windowPosX: -2560
    },
    {
        name: translate('profile.preset.options.triple4k'),
        windowWidth: 11520,
        windowHeight: 2160,
        windowPosY: 0,
        windowPosX: -3840
    }
]
