import { invoke } from '@tauri-apps/api/core';
import { createLogger } from '$lib/utils/logger';

const log = createLogger('pinned');

let pinnedSet = $state<Set<string>>(new Set());
let loaded = false;

export async function loadPinned(): Promise<void> {
	if (loaded) return;
	loaded = true;
	try {
		const list = await invoke<string[]>('get_pinned_packages');
		pinnedSet = new Set(list);
		log.info(`Loaded ${list.length} pinned packages`);
	} catch {
		pinnedSet = new Set();
	}
}

async function savePinned(): Promise<void> {
	try {
		await invoke('set_pinned_packages', { packages: [...pinnedSet] });
	} catch (e) {
		log.error(`Failed to save pinned packages: ${e}`);
	}
}

export function isPinned(manager: string, name: string): boolean {
	return pinnedSet.has(`${manager}/${name}`);
}

export function togglePin(manager: string, name: string): void {
	const key = `${manager}/${name}`;
	const next = new Set(pinnedSet);
	if (next.has(key)) {
		next.delete(key);
		log.info(`Unpinned ${key}`);
	} else {
		next.add(key);
		log.info(`Pinned ${key}`);
	}
	pinnedSet = next;
	savePinned();
}

export function getPinnedCount(): number {
	return pinnedSet.size;
}
