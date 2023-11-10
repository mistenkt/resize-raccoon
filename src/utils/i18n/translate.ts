import TranslationKeys from "./TranslationKeys";
import i18n from "./i18n";

const translate = (
    key: TranslationKeys,
    options?: Record<string, any>
): string => {
    return i18n.t(key, options);
};

export default translate;