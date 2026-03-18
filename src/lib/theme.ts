export type Theme = "light" | "dark" | "system";

/** Applies the given theme to the document root element. */
export function applyTheme(theme: Theme): void {
  const root = document.documentElement;

  if (theme === "system") {
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.classList.toggle("dark", prefersDark);
  } else {
    root.classList.toggle("dark", theme === "dark");
  }
}

/** Listens for OS-level color scheme changes and re-applies "system" theme. */
export function watchSystemTheme(getCurrentTheme: () => Theme): () => void {
  const mq = window.matchMedia("(prefers-color-scheme: dark)");
  const handler = () => {
    if (getCurrentTheme() === "system") {
      applyTheme("system");
    }
  };
  mq.addEventListener("change", handler);
  return () => mq.removeEventListener("change", handler);
}
