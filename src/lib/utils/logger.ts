type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface Logger {
	debug: (message: string, ...args: unknown[]) => void;
	info: (message: string, ...args: unknown[]) => void;
	warn: (message: string, ...args: unknown[]) => void;
	error: (message: string, ...args: unknown[]) => void;
}

type TauriLogModule = typeof import('@tauri-apps/plugin-log');

let tauriLog: TauriLogModule | null = null;
let tauriLogLoading: Promise<TauriLogModule | null> | null = null;

async function getTauriLog(): Promise<TauriLogModule | null> {
	if (tauriLog) return tauriLog;
	if (tauriLogLoading) return tauriLogLoading;

	if (typeof window !== 'undefined' && '__TAURI__' in window) {
		tauriLogLoading = import('@tauri-apps/plugin-log')
			.then((mod) => {
				tauriLog = mod;
				return mod;
			})
			.catch(() => {
				tauriLogLoading = null;
				return null;
			});
		return tauriLogLoading;
	}
	return null;
}

function formatMessage(tag: string, message: string): string {
	return `[${tag}] ${message}`;
}

async function forwardToTauri(level: LogLevel, tag: string, message: string): Promise<void> {
	const mod = await getTauriLog();
	if (!mod) return;

	const formatted = formatMessage(tag, message);
	switch (level) {
		case 'debug':
			await mod.debug(formatted);
			break;
		case 'info':
			await mod.info(formatted);
			break;
		case 'warn':
			await mod.warn(formatted);
			break;
		case 'error':
			await mod.error(formatted);
			break;
	}
}

export function createLogger(tag: string): Logger {
	return {
		debug(message: string, ...args: unknown[]) {
			console.debug(formatMessage(tag, message), ...args);
			forwardToTauri('debug', tag, message);
		},
		info(message: string, ...args: unknown[]) {
			console.info(formatMessage(tag, message), ...args);
			forwardToTauri('info', tag, message);
		},
		warn(message: string, ...args: unknown[]) {
			console.warn(formatMessage(tag, message), ...args);
			forwardToTauri('warn', tag, message);
		},
		error(message: string, ...args: unknown[]) {
			console.error(formatMessage(tag, message), ...args);
			forwardToTauri('error', tag, message);
		}
	};
}
