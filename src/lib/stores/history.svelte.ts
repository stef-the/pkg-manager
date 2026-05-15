import { invoke } from '@tauri-apps/api/core';
import { createLogger } from '$lib/utils/logger';

const log = createLogger('history');

export interface HistoryEntry {
	action: 'install' | 'uninstall' | 'update' | 'cleanup' | 'other';
	manager: string;
	package: string;
	timestamp: string;
}

let entries = $state<HistoryEntry[]>([]);

export async function loadHistory(): Promise<void> {
	try {
		const lines = await invoke<string[]>('read_history', { limit: 200 });
		entries = lines
			.map((line) => {
				try { return JSON.parse(line) as HistoryEntry; }
				catch { return null; }
			})
			.filter((e): e is HistoryEntry => e !== null)
			.reverse(); // newest first
	} catch {
		entries = [];
	}
}

export async function recordAction(
	action: HistoryEntry['action'],
	manager: string,
	pkg: string
): Promise<void> {
	const entry: HistoryEntry = {
		action,
		manager,
		package: pkg,
		timestamp: new Date().toISOString()
	};
	try {
		await invoke('append_history', { entry: JSON.stringify(entry) });
		entries = [entry, ...entries];
		log.info(`History: ${action} ${manager}/${pkg}`);
	} catch (e) {
		log.error(`Failed to record history: ${e}`);
	}
}

export function getHistory(): HistoryEntry[] {
	return entries;
}
