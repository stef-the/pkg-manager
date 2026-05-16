import * as commands from '$lib/utils/commands';
import { addToast } from '$lib/stores/toast.svelte';
import { createLogger } from '$lib/utils/logger';
import type { PackageManager, OutdatedPackage } from '$lib/types';

const log = createLogger('autoupdate');
const INTERVAL_KEY = 'pkg-manager-check-interval';
const LAST_CHECK_KEY = 'pkg-manager-last-check';

// Check interval in minutes (0 = disabled)
let checkInterval = $state(loadInterval());
let timer: ReturnType<typeof setInterval> | null = null;
let lastCheckTime = $state<Date | null>(loadLastCheck());
let lastOutdatedCount = $state(0);

function loadInterval(): number {
	if (typeof window === 'undefined') return 60;
	try {
		const stored = localStorage.getItem(INTERVAL_KEY);
		if (stored) return parseInt(stored, 10);
	} catch {}
	return 60; // default: check every 60 minutes
}

function loadLastCheck(): Date | null {
	if (typeof window === 'undefined') return null;
	try {
		const stored = localStorage.getItem(LAST_CHECK_KEY);
		if (stored) return new Date(stored);
	} catch {}
	return null;
}

function saveInterval(minutes: number): void {
	try { localStorage.setItem(INTERVAL_KEY, String(minutes)); } catch {}
}

function saveLastCheck(): void {
	const now = new Date();
	lastCheckTime = now;
	try { localStorage.setItem(LAST_CHECK_KEY, now.toISOString()); } catch {}
}

async function checkForUpdates(): Promise<void> {
	log.info('Auto-check: scanning for outdated packages...');
	try {
		const managers = await commands.getPackageManagers();
		const available = managers.filter((m) => m.available);

		let totalOutdated = 0;
		const newlyOutdated: string[] = [];

		for (const m of available) {
			try {
				const outdated = await commands.getOutdated(m.id);
				totalOutdated += outdated.length;
			} catch {
				// Skip failing managers
			}
		}

		saveLastCheck();

		if (totalOutdated > lastOutdatedCount && lastOutdatedCount >= 0) {
			const diff = totalOutdated - lastOutdatedCount;
			addToast(`${diff} new update${diff > 1 ? 's' : ''} available (${totalOutdated} total)`, 'info', 8000);
			log.info(`Auto-check: ${totalOutdated} outdated (${diff} new)`);

			// Send system notification if in Tauri
			if (typeof window !== 'undefined' && '__TAURI__' in window) {
				try {
					const { invoke } = await import('@tauri-apps/api/core');
					await invoke('send_notification', {
						title: 'Package Updates Available',
						body: `${totalOutdated} package${totalOutdated > 1 ? 's' : ''} can be updated`
					});
				} catch {}
			}
		} else {
			log.info(`Auto-check: ${totalOutdated} outdated (no change)`);
		}

		lastOutdatedCount = totalOutdated;
	} catch (e) {
		log.error(`Auto-check failed: ${e}`);
	}
}

export function startAutoCheck(): void {
	stopAutoCheck();
	if (checkInterval <= 0) return;

	const ms = checkInterval * 60 * 1000;
	log.info(`Auto-check: starting, interval ${checkInterval}m`);

	// Check if enough time has passed since last check
	if (lastCheckTime) {
		const elapsed = Date.now() - lastCheckTime.getTime();
		if (elapsed >= ms) {
			// Due for a check
			setTimeout(() => checkForUpdates(), 30000); // Wait 30s after app start
		}
	} else {
		// Never checked — do it after startup settles
		setTimeout(() => checkForUpdates(), 30000);
	}

	timer = setInterval(() => checkForUpdates(), ms);
}

export function stopAutoCheck(): void {
	if (timer) {
		clearInterval(timer);
		timer = null;
	}
}

export function setCheckInterval(minutes: number): void {
	checkInterval = minutes;
	saveInterval(minutes);
	startAutoCheck();
	log.info(`Auto-check interval set to ${minutes}m`);
}

export function getCheckInterval(): number {
	return checkInterval;
}

export function getLastCheckTime(): Date | null {
	return lastCheckTime;
}

export function checkNow(): void {
	checkForUpdates();
}
