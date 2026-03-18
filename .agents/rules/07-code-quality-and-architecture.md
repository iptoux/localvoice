# Rule 07 — Code Quality & Architecture

## Rule

Work as a senior developer in a well-maintained repo. Every change must leave the codebase cleaner, more modular, and more maintainable than before. No exceptions for "quick" fixes.

---

## 1. Before Writing Any Code

**Always analyse first, implement second.**

Before creating a new component, type, utility, hook, or style:
1. Search the existing codebase for similar or reusable implementations
2. Ask: can existing code be extended instead of duplicated?
3. Ask: can something be made generic enough to serve multiple use cases?

**Reuse beats new code. New code is only acceptable when no sensible reuse is possible.**

---

## 2. General Code Quality

- The codebase must remain clean, structured, modular, and maintainable at all times
- No quick-and-dirty solutions, unnecessary duplication, or unclear structures
- Every new piece of code must fit into and improve the existing architecture — never degrade it
- If a pattern already exists in the project, follow it — do not introduce a competing pattern

---

## 3. Component Architecture (Frontend)

- Components must be small, clearly separated, readable, and reusable
- Split large components into meaningful sub-components
- Props must be explicit and clean so components can be reused flexibly
- Business logic must not live directly in UI components — extract it into hooks, stores, or service modules
- Reusable UI elements must be built centrally and consistently

---

## 4. TypeScript Structure

- Types, interfaces, and enums that are reusable or domain-relevant belong in dedicated files (`types.ts`, `*.types.ts`, or `src/types/`)
- Do not scatter reusable types inline inside large component files
- Typing must be clear, consistent, and reused across the codebase — no redundant parallel type definitions

---

## 5. Project Structure

- Keep the folder structure logical, consistent, and scalable
- Group code by responsibility and feature — not by accident
- Reusable components, types, hooks, libs, and helpers must live at well-defined central locations
- Every new file must be placed where a future developer would intuitively look for it

---

## 6. Styling

- **Tailwind CSS first** — always prefer existing utility classes and design patterns already used in the project
- **shadcn/ui components** are preferred for UI building blocks; if a required shadcn/ui component is not yet installed, add it using the package manager already in use in the project (e.g. `pnpm` or `bun`)
- No unnecessary inline styles
- Do not mix styling approaches — if the project uses Tailwind + shadcn, stick to that exclusively

---

## 7. Stack Consistency

- Always follow the existing project conventions: same package manager, same folder structure, same naming conventions, same architectural principles
- Never introduce a new pattern if a consistent one already exists
- If in doubt, match what is already there

---

## 8. Goal for Every Change

Each change must make the codebase:
- **cleaner** — less noise, less duplication
- **more reusable** — shared abstractions where appropriate
- **clearer** — easier for the next developer to understand
- **more type-safe** — stricter, more explicit types
- **more modular** — smaller, single-responsibility units
- **more maintainable** — easier to change safely in the future

**When in doubt, always choose the solution that is architecturally sounder and more maintainable long-term.**
