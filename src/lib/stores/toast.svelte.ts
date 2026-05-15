import type { ToastItem } from '$lib/types';

let toasts = $state<ToastItem[]>([]);
const timers = new Map<string, ReturnType<typeof setTimeout>>();

function generateId(): string {
	return `toast-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

export function addToast(
	message: string,
	type: ToastItem['type'] = 'info',
	timeout = 4000
): string {
	const id = generateId();
	const toast: ToastItem = { id, message, type, timeout };
	toasts = [...toasts, toast];

	if (timeout > 0) {
		const timer = setTimeout(() => {
			removeToast(id);
		}, timeout);
		timers.set(id, timer);
	}

	return id;
}

export function removeToast(id: string): void {
	const timer = timers.get(id);
	if (timer) {
		clearTimeout(timer);
		timers.delete(id);
	}
	toasts = toasts.filter((t) => t.id !== id);
}

export function clearAllToasts(): void {
	for (const timer of timers.values()) {
		clearTimeout(timer);
	}
	timers.clear();
	toasts = [];
}

export function getToasts(): ToastItem[] {
	return toasts;
}
