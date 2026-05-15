import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { addToast, removeToast, getToasts, clearAllToasts } from './toast.svelte';

describe('toast store', () => {
	beforeEach(() => {
		clearAllToasts();
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('adds a toast', () => {
		addToast('Hello', 'info');
		const toasts = getToasts();
		expect(toasts).toHaveLength(1);
		expect(toasts[0].message).toBe('Hello');
		expect(toasts[0].type).toBe('info');
	});

	it('removes a toast by id', () => {
		const id = addToast('Test', 'success');
		expect(getToasts()).toHaveLength(1);
		removeToast(id);
		expect(getToasts()).toHaveLength(0);
	});

	it('auto-dismisses after timeout', () => {
		addToast('Temporary', 'warning', 2000);
		expect(getToasts()).toHaveLength(1);
		vi.advanceTimersByTime(2000);
		expect(getToasts()).toHaveLength(0);
	});

	it('clears all toasts', () => {
		addToast('One', 'info');
		addToast('Two', 'error');
		addToast('Three', 'success');
		expect(getToasts()).toHaveLength(3);
		clearAllToasts();
		expect(getToasts()).toHaveLength(0);
	});

	it('generates unique ids', () => {
		const id1 = addToast('A', 'info');
		const id2 = addToast('B', 'info');
		expect(id1).not.toBe(id2);
	});
});
