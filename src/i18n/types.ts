export type UILanguage = 'en' | 'de';

export interface TranslationNamespace {
  common: typeof import('./en.json').common;
  pill: typeof import('./en.json').pill;
  sidebar: typeof import('./en.json').sidebar;
  dashboard: typeof import('./en.json').dashboard;
  history: typeof import('./en.json').history;
  dictionary: typeof import('./en.json').dictionary;
  models: typeof import('./en.json').models;
  settings: typeof import('./en.json').settings;
  logs: typeof import('./en.json').logs;
  onboarding: typeof import('./en.json').onboarding;
  errors: typeof import('./en.json').errors;
}
