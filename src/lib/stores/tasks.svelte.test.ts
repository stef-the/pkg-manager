import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { runTask, getTasks, dismissTask, hasRunningTasks, elapsed } from './tasks.svelte';

describe('tasks store', () => {
	beforeEach(() => {
		// Clear all tasks
		for (const task of getTasks()) {
			dismissTask(task.id);
		}
	});

	it('runTask adds a task and resolves', async () => {
		runTask('Test task', () => Promise.resolve('done'));
		const tasks = getTasks();
		expect(tasks.length).toBeGreaterThanOrEqual(1);
		const task = tasks.find((t) => t.label === 'Test task');
		expect(task).toBeDefined();
		expect(task!.status).toBe('running');
	});

	it('hasRunningTasks reflects running state', () => {
		runTask('Long task', () => new Promise(() => {})); // never resolves
		expect(hasRunningTasks()).toBe(true);
	});

	it('elapsed returns formatted time', () => {
		const task = {
			id: 'test',
			label: 'test',
			status: 'running' as const,
			startedAt: Date.now() - 5000,
			progress: -1,
			total: 0,
			current: 0,
			currentLabel: ''
		};
		const result = elapsed(task);
		expect(result).toMatch(/\d+s/);
	});

	it('dismissTask removes a task', () => {
		const id = runTask('Dismiss me', () => Promise.resolve());
		expect(getTasks().some((t) => t.id === id)).toBe(true);
		dismissTask(id);
		expect(getTasks().some((t) => t.id === id)).toBe(false);
	});
});
