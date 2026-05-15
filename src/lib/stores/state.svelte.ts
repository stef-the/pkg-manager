import { invoke } from '@tauri-apps/api/core';
import { createLogger } from '$lib/utils/logger';
import type { ViewId, PackageManager } from '$lib/types';

const log = createLogger('state');

interface AppState {
	activeView: ViewId;
	activeManagerFilter: PackageManager | 'all';
	sidebarScrollTop: number;
}

const DEFAULT_STATE: AppState = {
	activeView: 'dashboard',
	activeManagerFilter: 'all',
	sidebarScrollTop: 0
};

export async function saveState(state: Partial<AppState>): Promise<void> {
	try {
		const current = await loadState();
		const merged = { ...current, ...state };
		await invoke('save_window_state', { state: JSON.stringify(merged) });
	} catch {
		// Silently fail — not critical
	}
}

export async function loadState(): Promise<AppState> {
	try {
		const raw = await invoke<string>('load_window_state');
		return { ...DEFAULT_STATE, ...JSON.parse(raw) };
	} catch {
		return DEFAULT_STATE;
	}
}

/**
 * Update the window title with outdated count.
 */
export async function updateWindowTitle(outdatedCount: number): Promise<void> {
	try {
		const title = outdatedCount > 0
			? `Pkg Manager (${outdatedCount} outdated)`
			: 'Pkg Manager';
		await invoke('set_window_title', { title });
	} catch {
		// Not in Tauri context
	}
}

/**
 * Update the tray tooltip with status.
 */
export async function updateTrayTooltip(totalPkgs: number, outdatedCount: number): Promise<void> {
	try {
		const tooltip = outdatedCount > 0
			? `Pkg Manager — ${totalPkgs} packages, ${outdatedCount} outdated`
			: `Pkg Manager — ${totalPkgs} packages, all up to date`;
		await invoke('set_tray_tooltip', { tooltip });
	} catch {
		// Not in Tauri context
	}
}
