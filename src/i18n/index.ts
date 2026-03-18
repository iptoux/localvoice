import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './en.json';
import de from './de.json';

export const SUPPORTED_UI_LANGUAGES = ['en', 'de'] as const;
export type UILanguage = (typeof SUPPORTED_UI_LANGUAGES)[number];

const resources = {
  en: { translation: en },
  de: { translation: de },
};

export const initI18n = (language: UILanguage = 'en') => {
  return i18n.use(initReactI18next).init({
    resources,
    lng: language,
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
    },
  });
};

export default i18n;
