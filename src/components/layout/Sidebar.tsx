import { NavLink } from "react-router-dom";
import {
  LayoutDashboard,
  History,
  BookOpen,
  Cpu,
  ScrollText,
  Settings,
} from "lucide-react";

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
  return (
    <nav className="w-48 bg-neutral-900 text-neutral-200 flex flex-col py-6 px-3 shrink-0">
      <span className="text-xs font-semibold text-neutral-500 uppercase tracking-wider px-3 mb-2">
        LocalVoice
      </span>
      <div className="flex flex-col gap-1">
        {TOP_LINKS.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            end={to === "/"}
            className={({ isActive }) =>
              `flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors ${
                isActive
                  ? "bg-neutral-700 text-white"
                  : "hover:bg-neutral-800 text-neutral-300"
              }`
            }
          >
            <Icon className="size-4 shrink-0" />
            {label}
          </NavLink>
        ))}
      </div>
      <div className="mt-auto flex flex-col gap-1">
        {BOTTOM_LINKS.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              `flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors ${
                isActive
                  ? "bg-neutral-700 text-white"
                  : "hover:bg-neutral-800 text-neutral-300"
              }`
            }
          >
            <Icon className="size-4 shrink-0" />
            {label}
          </NavLink>
        ))}
      </div>
    </nav>
  );
}
