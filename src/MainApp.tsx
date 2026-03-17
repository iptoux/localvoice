import "./index.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Sidebar } from "./components/layout/Sidebar";
import Dashboard from "./pages/Dashboard";
import History from "./pages/History";
import Dictionary from "./pages/Dictionary";
import Models from "./pages/Models";
import SettingsPage from "./pages/SettingsPage";

export function MainApp() {
  return (
    <BrowserRouter>
      <div className="flex h-screen bg-neutral-950 text-white">
        <Sidebar />
        <main className="flex-1 overflow-auto">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/history" element={<History />} />
            <Route path="/dictionary" element={<Dictionary />} />
            <Route path="/models" element={<Models />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}
