import "./index.css";
import { lazy, Suspense, useEffect, useState } from "react";
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import { listen } from "@tauri-apps/api/event";
import { Sidebar } from "./components/layout/Sidebar";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { Onboarding } from "./components/Onboarding";
import { PageSpinner } from "./components/Spinner";
import { checkFirstRun, getSettings } from "./lib/tauri";
import { applyTheme, watchSystemTheme, type Theme } from "./lib/theme";

const Dashboard = lazy(() => import("./pages/Dashboard"));
const History = lazy(() => import("./pages/History"));
const Dictionary = lazy(() => import("./pages/Dictionary"));
const Logs = lazy(() => import("./pages/Logs"));
const Models = lazy(() => import("./pages/Models"));
const SettingsPage = lazy(() => import("./pages/SettingsPage"));

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
        <NavigationListener />
        <div className="flex h-screen bg-background text-foreground">
          <Sidebar />
          <main className="flex-1 overflow-auto">
            <Suspense fallback={<PageSpinner />}>
              <Routes>
                <Route path="/" element={<Dashboard />} />
                <Route path="/history" element={<History />} />
                <Route path="/dictionary" element={<Dictionary />} />
                <Route path="/models" element={<Models />} />
                <Route path="/logs" element={<Logs />} />
                <Route path="/settings" element={<SettingsPage />} />
              </Routes>
            </Suspense>
          </main>
        </div>
        {showOnboarding && (
          <Onboarding onDismiss={() => setShowOnboarding(false)} />
        )}
      </BrowserRouter>
    </ErrorBoundary>
  );
}

function NavigationListener() {
  const navigate = useNavigate();

  useEffect(() => {
    const unlisten = listen<string>("navigate-to", (event) => {
      navigate(event.payload);
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [navigate]);

  return null;
}
