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
import { checkFirstRun } from "./lib/tauri";

export function MainApp() {
  const [showOnboarding, setShowOnboarding] = useState(false);

  useEffect(() => {
    checkFirstRun()
      .then((needsOnboarding) => {
        if (needsOnboarding) setShowOnboarding(true);
      })
      .catch(() => {});
  }, []);

  return (
    <ErrorBoundary>
      <BrowserRouter>
        <div className="flex h-screen bg-neutral-950 text-white">
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
