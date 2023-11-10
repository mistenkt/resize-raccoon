import { useTranslation as useTrans } from 'react-i18next';
import TranslationKeys from './TranslationKeys';

export const useTranslation = () => {
    const { t } = useTrans();
    return (key: TranslationKeys) => t(key);
};