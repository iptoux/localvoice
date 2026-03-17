import { NavLink } from "react-router-dom";

const LINKS = [
  { to: "/", label: "Dashboard" },
  { to: "/history", label: "History" },
  { to: "/dictionary", label: "Dictionary" },
  { to: "/models", label: "Models" },
  { to: "/settings", label: "Settings" },
];

export function Sidebar() {
  return (
    <nav className="w-48 bg-neutral-900 text-neutral-200 flex flex-col py-6 px-3 gap-1 shrink-0">
      <span className="text-xs font-semibold text-neutral-500 uppercase tracking-wider px-3 mb-2">
        LocalVoice
      </span>
      {LINKS.map(({ to, label }) => (
        <NavLink
          key={to}
          to={to}
          end={to === "/"}
          className={({ isActive }) =>
            `px-3 py-2 rounded-md text-sm transition-colors ${
              isActive
                ? "bg-neutral-700 text-white"
                : "hover:bg-neutral-800 text-neutral-300"
            }`
          }
        >
          {label}
        </NavLink>
      ))}
    </nav>
  );
}
