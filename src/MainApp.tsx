import "./index.css";
import { useEffect, useState } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Sidebar } from "./components/layout/Sidebar";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { Onboarding } from "./components/Onboarding";
import Dashboard from "./pages/Dashboard";
import History from "./pages/History";
import Dictionary from "./pages/Dictionary";
import Logs from "./pages/Logs";
import Models from "./pages/Models";
import SettingsPage from "./pages/SettingsPage";
import { checkFirstRun, getSettings } from "./lib/tauri";
import { applyTheme, watchSystemTheme, type Theme } from "./lib/theme";

export function MainApp() {
  const [showOnboarding, setShowOnboarding] = useState(false);

  useEffect(() => {
    checkFirstRun()
      .then((needsOnboarding) => {
        if (needsOnboarding) setShowOnboarding(true);
      })
      .catch(() => {});
  }, []);

  // Apply persisted theme on mount and watch for OS changes.
  useEffect(() => {
    let currentTheme: Theme = "dark";
    getSettings()
      .then((s) => {
        currentTheme = (s["app.theme"] as Theme) || "dark";
        applyTheme(currentTheme);
      })
      .catch(() => applyTheme("dark"));

    return watchSystemTheme(() => currentTheme);
  }, []);

  return (
    <ErrorBoundary>
      <BrowserRouter>
        <div className="flex h-screen bg-background text-foreground">
          <Sidebar />
          <main className="flex-1 overflow-auto">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/history" element={<History />} />
              <Route path="/dictionary" element={<Dictionary />} />
              <Route path="/models" element={<Models />} />
              <Route path="/logs" element={<Logs />} />
              <Route path="/settings" element={<SettingsPage />} />
            </Routes>
          </main>
        </div>
        {showOnboarding && (
          <Onboarding onDismiss={() => setShowOnboarding(false)} />
        )}
      </BrowserRouter>
    </ErrorBoundary>
  );
}
