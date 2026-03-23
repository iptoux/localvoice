import { NavLink } from "react-router-dom";
import { useShallow } from "zustand/react/shallow";
import { useSettingsStore } from "../../stores/settings-store";
import {
  LayoutDashboard,
  History,
  BookOpen,
  Cpu,
  ScrollText,
  Settings,
} from "lucide-react";
import { useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { TIPS } from "../../data/tips";

const TOP_LINKS = [
  { to: "/", label: "Dashboard", icon: LayoutDashboard },
  { to: "/history", label: "History", icon: History },
  { to: "/dictionary", label: "Dictionary", icon: BookOpen },
  { to: "/models", label: "Models", icon: Cpu },
  { to: "/logs", label: "Logs", icon: ScrollText },
];

const BOTTOM_LINKS = [
  { to: "/settings", label: "Settings", icon: Settings },
];

const TIP_INTERVAL_MS = 8_000;

export function Sidebar() {
  const load = useSettingsStore((s) => s.load);
  const loggingEnabled = useSettingsStore(
    useShallow((s) => s.settings["logging.enabled"] !== "false")
  );
  const [appVersion, setAppVersion] = useState<string>("");
  const [tipIndex, setTipIndex] = useState(() => Math.floor(Math.random() * TIPS.length));
  const [visible, setVisible] = useState(true);

  useEffect(() => { load(); }, [load]);
  useEffect(() => { getVersion().then(setAppVersion); }, []);

  useEffect(() => {
    const id = setInterval(() => {
      setVisible(false);
      setTimeout(() => {
        setTipIndex((i) => (i + 1) % TIPS.length);
        setVisible(true);
      }, 300);
    }, TIP_INTERVAL_MS);
    return () => clearInterval(id);
  }, []);

  const visibleTopLinks = TOP_LINKS.filter(
    (l) => l.to !== "/logs" || loggingEnabled
  );

  const tip = TIPS[tipIndex];
  const TipIcon = tip.icon;

  return (
    <nav className="w-48 bg-sidebar text-sidebar-foreground flex flex-col py-6 px-3 shrink-0">
      <div className="px-3 mb-4">
        <img src="/localvoice_logo_transparent.svg" alt="LocalVoice" className="w-full h-auto" />
      </div>
      <div className="flex flex-col gap-1">
        {visibleTopLinks.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            end={to === "/"}
            className={({ isActive }) =>
              `flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors ${
                isActive
                  ? "bg-sidebar-accent text-sidebar-accent-foreground font-medium"
                  : "hover:bg-sidebar-accent/50 text-sidebar-foreground/70"
              }`
            }
          >
            <Icon className="size-4 shrink-0" />
            {label}
          </NavLink>
        ))}
      </div>
      <div className="mt-auto flex flex-col gap-1">
        <div
          className="flex items-start gap-2 px-3 py-2 mb-1 rounded-md bg-sidebar-accent/40 text-sidebar-foreground/50 text-[11px] leading-snug transition-opacity duration-300"
          style={{ opacity: visible ? 1 : 0 }}
        >
          <TipIcon className="size-3 shrink-0 mt-0.5 opacity-60" />
          <span>{tip.text}</span>
        </div>

        {BOTTOM_LINKS.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              `flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors ${
                isActive
                  ? "bg-sidebar-accent text-sidebar-accent-foreground font-medium"
                  : "hover:bg-sidebar-accent/50 text-sidebar-foreground/70"
              }`
            }
          >
            <Icon className="size-4 shrink-0" />
            {label}
          </NavLink>
        ))}

        <div className="border-t border-sidebar-border mt-2 pt-2 px-3">
          <span className="text-[11px] text-sidebar-foreground/30">{appVersion ? `v${appVersion}` : ""}</span>
        </div>
      </div>
    </nav>
  );
}
