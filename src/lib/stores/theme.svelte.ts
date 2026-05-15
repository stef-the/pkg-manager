import type { ThemeMode, DateFormat } from '$lib/types';
import { createLogger } from '$lib/utils/logger';

const log = createLogger('theme');
const THEME_KEY = 'pkg-manager-theme';
const DATE_KEY = 'pkg-manager-date-format';

let preference = $state<ThemeMode>(loadPreference());
let dateFormat = $state<DateFormat>(loadDateFormat());
let systemDark = $state(false);

const effectiveTheme = $derived<'dark' | 'light'>(
	preference === 'system' ? (systemDark ? 'dark' : 'light') : preference
);

function loadPreference(): ThemeMode {
	if (typeof window === 'undefined') return 'system';
	try {
		const stored = localStorage.getItem(THEME_KEY);
		if (stored === 'dark' || stored === 'light' || stored === 'system') {
			return stored;
		}
	} catch {
		log.warn('Failed to read theme preference from localStorage');
	}
	return 'system';
}

function loadDateFormat(): DateFormat {
	if (typeof window === 'undefined') return 'dd/mm/yyyy';
	try {
		const stored = localStorage.getItem(DATE_KEY);
		if (stored === 'dd/mm/yyyy' || stored === 'mm/dd/yyyy' || stored === 'yyyy-mm-dd') {
			return stored;
		}
	} catch {
		log.warn('Failed to read date format from localStorage');
	}
	return 'dd/mm/yyyy';
}

function savePreference(mode: ThemeMode): void {
	try {
		localStorage.setItem(THEME_KEY, mode);
	} catch {
		log.warn('Failed to save theme preference to localStorage');
	}
}

function saveDateFormat(fmt: DateFormat): void {
	try {
		localStorage.setItem(DATE_KEY, fmt);
	} catch {
		log.warn('Failed to save date format to localStorage');
	}
}

export function initTheme(): void {
	if (typeof window === 'undefined') return;

	const mq = window.matchMedia('(prefers-color-scheme: dark)');
	systemDark = mq.matches;

	mq.addEventListener('change', (e) => {
		systemDark = e.matches;
		log.info(`System theme changed to ${e.matches ? 'dark' : 'light'}`);
	});

	log.info(`Theme initialized: preference=${preference}, effective=${effectiveTheme}`);
}

export function setThemePreference(mode: ThemeMode): void {
	preference = mode;
	savePreference(mode);
	log.info(`Theme preference set to ${mode} (effective: ${effectiveTheme})`);
}

export function getThemePreference(): ThemeMode {
	return preference;
}

export function getEffectiveTheme(): 'dark' | 'light' {
	return effectiveTheme;
}

export function setDateFormat(fmt: DateFormat): void {
	dateFormat = fmt;
	saveDateFormat(fmt);
	log.info(`Date format set to ${fmt}`);
}

export function getDateFormat(): DateFormat {
	return dateFormat;
}

export function applyThemeToDocument(): void {
	if (typeof document === 'undefined') return;
	const html = document.documentElement;
	if (effectiveTheme === 'dark') {
		html.classList.add('dark');
	} else {
		html.classList.remove('dark');
	}
}
