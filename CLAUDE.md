# Pkg Manager

Cross-platform package manager GUI built with SvelteKit + Tauri.

## Stack

- **Frontend:** SvelteKit (Svelte 5 runes mode) + Tailwind CSS v4 + TypeScript
- **Native:** Tauri v2 (Rust backend, compiles to native app)
- **Testing:** Vitest (unit) + Playwright (e2e)
- **Linting:** ESLint + Prettier

## Architecture

This is a Tauri app — the frontend is a SvelteKit static build served inside a native webview.
The Rust backend (src-tauri/) handles system calls to package managers (brew, apt, winget, npm, etc.)
via Tauri commands. The frontend communicates with the backend via `@tauri-apps/api`.

### Key decisions

- **adapter-static** — Tauri needs static HTML output, not a Node server
- **Tailwind CSS v4** — imported via `@tailwindcss/vite` plugin, no config file needed
- **Nord theme** — user preference, use Nord color palette throughout
- **System tray** — app should minimize to system tray, launch on startup (minimized)
- **Cross-platform** — must work on macOS and Windows (WSL integration for apt on Windows)

## Commands

```bash
npm run dev          # SvelteKit dev server (browser)
npm run tauri:dev    # Tauri dev mode (native window + hot reload)
npm run tauri:build  # Build native app
npm run test         # Vitest unit tests
npm run test:watch   # Vitest watch mode
npm run test:e2e     # Playwright end-to-end tests
npm run check        # Svelte type checking
npm run lint         # ESLint
npm run format       # Prettier
```

## Project structure

```
src/
  routes/           SvelteKit pages
  lib/
    components/     Svelte components
    stores/         Svelte stores (reactive state)
    types/          TypeScript type definitions
    utils/          Utility functions
  app.css           Tailwind CSS entry point
  app.html          HTML template
src-tauri/
  src/
    main.rs         Tauri entry point
    lib.rs          Tauri commands (system calls to package managers)
  Cargo.toml        Rust dependencies
  tauri.conf.json   Tauri configuration
```

## Package manager adapters

Each package manager should have its own adapter module in the Rust backend:
- `brew` — macOS (Homebrew)
- `apt` — Linux / WSL
- `winget` — Windows
- `npm` — Node.js packages (global)
- Possibly: `cargo`, `pip`, `gh extensions`

Each adapter implements a common trait: list, search, install, uninstall, update, outdated.

## User preferences

- Nord color palette for all UI
- Clean, minimal design — think linear.app aesthetic
- System tray with quick actions
- Launch minimized on startup (configurable)
- Dark mode only
