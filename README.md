# Pkg Manager

A cross-platform package manager GUI built with SvelteKit, Tailwind CSS v4, and Tauri v2.

<!-- screenshot -->

## Install

Download the latest release for your platform:

| Platform | Download |
|----------|----------|
| macOS (Apple Silicon) | [Pkg Manager_0.1.0_aarch64.dmg](https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_aarch64.dmg) |
| macOS (Intel) | [Pkg Manager_0.1.0_x64.dmg](https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_x64.dmg) |
| Windows | [Pkg Manager_0.1.0_x64-setup.exe](https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_x64-setup.exe) |
| Linux (deb) | [pkg-manager_0.1.0_amd64.deb](https://github.com/stef-the/pkg-manager/releases/latest/download/pkg-manager_0.1.0_amd64.deb) |
| Linux (AppImage) | [pkg-manager_0.1.0_amd64.AppImage](https://github.com/stef-the/pkg-manager/releases/latest/download/pkg-manager_0.1.0_amd64.AppImage) |

Or check the [Releases page](https://github.com/stef-the/pkg-manager/releases) for all options.

### One-line Install

#### macOS (Apple Silicon)

```bash
curl -L https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_aarch64.dmg -o /tmp/PkgManager.dmg && hdiutil attach /tmp/PkgManager.dmg && cp -R "/Volumes/Pkg Manager/Pkg Manager.app" /Applications/ && hdiutil detach "/Volumes/Pkg Manager" && xattr -cr "/Applications/Pkg Manager.app" && open "/Applications/Pkg Manager.app"
```

#### macOS (Intel)

```bash
curl -L https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_x64.dmg -o /tmp/PkgManager.dmg && hdiutil attach /tmp/PkgManager.dmg && cp -R "/Volumes/Pkg Manager/Pkg Manager.app" /Applications/ && hdiutil detach "/Volumes/Pkg Manager" && xattr -cr "/Applications/Pkg Manager.app" && open "/Applications/Pkg Manager.app"
```

#### Linux (Debian / Ubuntu)

```bash
curl -L https://github.com/stef-the/pkg-manager/releases/latest/download/pkg-manager_0.1.0_amd64.deb -o /tmp/pkg-manager.deb && sudo dpkg -i /tmp/pkg-manager.deb && pkg-manager
```

#### Linux (AppImage)

```bash
curl -L https://github.com/stef-the/pkg-manager/releases/latest/download/pkg-manager_0.1.0_amd64.AppImage -o ~/.local/bin/pkg-manager && chmod +x ~/.local/bin/pkg-manager && ~/.local/bin/pkg-manager
```

#### Windows (PowerShell)

```powershell
Invoke-WebRequest -Uri "https://github.com/stef-the/pkg-manager/releases/latest/download/Pkg.Manager_0.1.0_x64-setup.exe" -OutFile "$env:TEMP\PkgManagerSetup.exe"; Start-Process "$env:TEMP\PkgManagerSetup.exe" -Wait
```

#### Windows (winget)

```powershell
winget install stef-the.PkgManager
```

> *winget source coming soon — for now use the PowerShell command above.*

## Features

### Package Managers

Manage packages across nine package managers from a single interface:

- **brew** -- macOS (Homebrew)
- **npm** -- Node.js (global packages)
- **mas** -- Mac App Store
- **pip** -- Python
- **cargo** -- Rust
- **winget** -- Windows
- **apt** -- Debian / Ubuntu / WSL
- **flatpak** -- Linux
- **snap** -- Linux

Each adapter supports list, search, install, uninstall, update, and outdated operations.

### Interface

- **Nord theme** with dark, light, and system modes
- **System tray** -- minimize to tray, launch on startup (configurable)
- **Keyboard shortcuts** -- `Cmd+K` command palette, `Cmd+1`-`6` view switching, `Cmd+R` refresh
- **Package pinning** -- pin packages to prevent accidental updates
- **History log** -- track every install, uninstall, and update
- **Task runner** -- batch operations with pause and cancel support
- **Debloat tool** -- identify and remove unused packages
- **Export / Import** -- save and restore your package lists
- **Terminal** -- embedded terminal for manual commands
- **Cleanup / Doctor** -- free disk space and diagnose issues

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) 22.x or later
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Tauri CLI](https://tauri.app/) (`npm install -g @tauri-apps/cli`)

### Install and Run

```bash
git clone https://github.com/stef-the/pkg-manager.git
cd pkg-manager
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

The compiled application will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
src/
  routes/             SvelteKit pages
  lib/
    assets/           Static assets (icons, images)
    components/       Svelte components
    stores/           Svelte stores (reactive state)
    types/            TypeScript type definitions
    utils/            Utility functions
  app.css             Tailwind CSS entry point
  app.html            HTML template
src-tauri/
  src/
    adapters/         Package manager adapters (brew, npm, apt, etc.)
    commands.rs       Tauri command handlers
    error.rs          Error types
    models.rs         Shared data models
    lib.rs            Tauri plugin setup
    main.rs           Entry point
  Cargo.toml          Rust dependencies
  tauri.conf.json     Tauri configuration
```

## Development Commands

| Command                | Description                              |
| ---------------------- | ---------------------------------------- |
| `npm run dev`          | SvelteKit dev server (browser only)      |
| `npm run tauri:dev`    | Tauri dev mode (native window + HMR)     |
| `npm run tauri:build`  | Build native application                 |
| `npm run check`        | Svelte / TypeScript type checking        |
| `npm run test`         | Run Vitest unit tests                    |
| `npm run test:watch`   | Run Vitest in watch mode                 |
| `npm run test:e2e`     | Run Playwright end-to-end tests          |
| `npm run lint`         | ESLint                                   |
| `npm run format`       | Prettier                                 |

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a pull request

Please make sure `npm run check`, `npm run lint`, and `npm run test` pass before submitting.

## License

[MIT](LICENSE)
