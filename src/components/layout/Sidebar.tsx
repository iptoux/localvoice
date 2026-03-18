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
  MousePointerClick,
} from "lucide-react";
import { useEffect } from "react";

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

export function Sidebar() {
  const load = useSettingsStore((s) => s.load);
  const loggingEnabled = useSettingsStore(
    useShallow((s) => s.settings["logging.enabled"] !== "false")
  );

  useEffect(() => { load(); }, [load]);

  const visibleTopLinks = TOP_LINKS.filter(
    (l) => l.to !== "/logs" || loggingEnabled
  );

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
        {/* Tip: right-click the pill */}
        <div className="flex items-start gap-2 px-3 py-2 mb-1 rounded-md bg-sidebar-accent/40 text-sidebar-foreground/50 text-[11px] leading-snug">
          <MousePointerClick className="size-3 shrink-0 mt-0.5 opacity-60" />
          <span>Right-click the pill to expand quick actions</span>
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
          <span className="text-[11px] text-sidebar-foreground/30">v0.1.0</span>
        </div>
      </div>
    </nav>
  );
}
