export type PackageManager = 'brew' | 'npm' | 'winget' | 'mas' | 'pip' | 'cargo' | 'apt' | 'flatpak' | 'snap' | 'nix' | 'scoop' | 'choco' | 'dnf' | 'pacman' | 'conda';

export interface Package {
	name: string;
	version: string;
	description: string;
	manager: PackageManager;
}

export interface OutdatedPackage {
	name: string;
	currentVersion: string;
	latestVersion: string;
	description: string;
	manager: PackageManager;
}

export interface ManagerInfo {
	id: PackageManager;
	name: string;
	available: boolean;
	version: string;
}

export interface SystemStats {
	os: string;
	arch: string;
	hostname: string;
	managers: ManagerInfo[];
}

export interface PackageDetail {
	name: string;
	version: string;
	description: string;
	manager: PackageManager;
	homepage: string;
	license: string;
	repository: string;
	dependencies: string[];
	installSize: string;
	installedOn: string;
}

export type ThemeMode = 'dark' | 'light' | 'system';

export type DateFormat = 'dd/mm/yyyy' | 'mm/dd/yyyy' | 'yyyy-mm-dd';

export type ViewId =
	| 'dashboard'
	| 'installed'
	| 'outdated'
	| 'search'
	| 'browse'
	| 'managers'
	| 'cleanup'
	| 'export-import'
	| 'logs'
	| 'terminal'
	| 'debloat'
	| 'history'
	| 'apps';

export interface ToastItem {
	id: string;
	message: string;
	type: 'success' | 'error' | 'warning' | 'info';
	timeout: number;
}
