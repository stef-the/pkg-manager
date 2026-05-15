import { vi } from 'vitest';

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

// Mock @tauri-apps/plugin-log
vi.mock('@tauri-apps/plugin-log', () => ({
	debug: vi.fn(),
	info: vi.fn(),
	warn: vi.fn(),
	error: vi.fn()
}));

// Mock @tauri-apps/plugin-autostart
vi.mock('@tauri-apps/plugin-autostart', () => ({
	enable: vi.fn(),
	disable: vi.fn(),
	isEnabled: vi.fn().mockResolvedValue(false)
}));

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: vi.fn().mockImplementation((query: string) => ({
		matches: query === '(prefers-color-scheme: dark)',
		media: query,
		onchange: null,
		addListener: vi.fn(),
		removeListener: vi.fn(),
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
});
