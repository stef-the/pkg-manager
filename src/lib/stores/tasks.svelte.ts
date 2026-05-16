import { createLogger } from '$lib/utils/logger';
import { addToast } from '$lib/stores/toast.svelte';

const log = createLogger('tasks');

export interface RunningTask {
	id: string;
	label: string;
	status: 'running' | 'paused' | 'done' | 'error' | 'cancelled';
	startedAt: number;
	progress: number; // 0-100, -1 = indeterminate
	total: number; // total items (0 = unknown)
	current: number; // current item index
	currentLabel: string; // e.g. "Upgrading wget..."
	result?: string;
	error?: string;
}

let tasks = $state<RunningTask[]>([]);
let tick = $state(0);

// Tick every second for elapsed time display
if (typeof window !== 'undefined') {
	setInterval(() => { tick++; }, 1000);
}

function generateId(): string {
	return `task-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
}

export function runTask<T>(
	label: string,
	operation: () => Promise<T>,
	options?: {
		onSuccess?: (result: T) => void;
		onError?: (error: unknown) => void;
		successMessage?: string;
		errorMessage?: string;
		suppressErrorToast?: boolean;
	}
): string {
	const id = generateId();
	const task: RunningTask = {
		id,
		label,
		status: 'running',
		startedAt: Date.now(),
		progress: -1,
		total: 0,
		current: 0,
		currentLabel: ''
	};

	tasks = [...tasks, task];
	log.info(`Task started: ${label} (${id})`);

	operation()
		.then((result) => {
			updateTask(id, {
				status: 'done',
				progress: 100,
				result: typeof result === 'string' ? result : undefined
			});
			log.info(`Task completed: ${label} (${id})`);
			if (options?.successMessage) {
				addToast(options.successMessage, 'success');
			}
			options?.onSuccess?.(result);
			setTimeout(() => removeTask(id), 3000);
		})
		.catch((error) => {
			const msg = `${error}`;
			updateTask(id, { status: 'error', error: msg });
			log.error(`Task failed: ${label} (${id}): ${msg}`);
			if (!options?.suppressErrorToast) {
				addToast(options?.errorMessage ?? `${label} failed: ${msg}`, 'error');
			}
			options?.onError?.(error);
			setTimeout(() => removeTask(id), 5000);
		});

	return id;
}

/**
 * Run a batch of operations with progress tracking.
 * Each item runs sequentially. Supports pause and cancel.
 */
export function runBatchTask<T>(
	label: string,
	items: T[],
	operation: (item: T, index: number) => Promise<void>,
	options?: {
		itemLabel?: (item: T, index: number) => string;
		onSuccess?: () => void;
		onError?: (error: unknown) => void;
		successMessage?: string;
	}
): string {
	const id = generateId();
	const task: RunningTask = {
		id,
		label,
		status: 'running',
		startedAt: Date.now(),
		progress: 0,
		total: items.length,
		current: 0,
		currentLabel: ''
	};

	tasks = [...tasks, task];
	log.info(`Batch task started: ${label} (${id}), ${items.length} items`);

	(async () => {
		for (let i = 0; i < items.length; i++) {
			// Yield to event loop so UI can repaint between items
			await new Promise((r) => setTimeout(r, 0));

			// Check for cancel/pause
			const current = getTask(id);
			if (!current || current.status === 'cancelled') {
				log.info(`Batch task cancelled: ${label} (${id}) at item ${i}/${items.length}`);
				updateTask(id, { status: 'cancelled', currentLabel: 'Cancelled' });
				setTimeout(() => removeTask(id), 3000);
				return;
			}

			// Wait while paused
			while (getTask(id)?.status === 'paused') {
				await new Promise((r) => setTimeout(r, 500));
			}

			// Re-check after unpause
			const afterPause = getTask(id);
			if (!afterPause || afterPause.status === 'cancelled') {
				updateTask(id, { status: 'cancelled', currentLabel: 'Cancelled' });
				setTimeout(() => removeTask(id), 3000);
				return;
			}

			const itemLabel = options?.itemLabel?.(items[i], i) ?? `Item ${i + 1}/${items.length}`;
			updateTask(id, {
				current: i + 1,
				progress: Math.round(((i) / items.length) * 100),
				currentLabel: itemLabel
			});

			try {
				await operation(items[i], i);
			} catch (e) {
				log.error(`Batch item failed: ${itemLabel}: ${e}`);
				// Continue with next item
			}
		}

		updateTask(id, {
			status: 'done',
			progress: 100,
			currentLabel: 'Complete'
		});
		log.info(`Batch task completed: ${label} (${id})`);
		if (options?.successMessage) {
			addToast(options.successMessage, 'success');
		}
		options?.onSuccess?.();
		setTimeout(() => removeTask(id), 3000);
	})().catch((error) => {
		updateTask(id, { status: 'error', error: `${error}` });
		log.error(`Batch task failed: ${label} (${id}): ${error}`);
		options?.onError?.(error);
		setTimeout(() => removeTask(id), 5000);
	});

	return id;
}

function getTask(id: string): RunningTask | undefined {
	return tasks.find((t) => t.id === id);
}

function updateTask(id: string, updates: Partial<RunningTask>): void {
	tasks = tasks.map((t) => (t.id === id ? { ...t, ...updates } : t));
}

function removeTask(id: string): void {
	tasks = tasks.filter((t) => t.id !== id);
}

export function getTasks(): RunningTask[] {
	// Reference tick to trigger re-render every second
	void tick;
	return tasks;
}

export function getRunningTasks(): RunningTask[] {
	void tick;
	return tasks.filter((t) => t.status === 'running');
}

export function hasRunningTasks(): boolean {
	return tasks.some((t) => t.status === 'running');
}

export function pauseTask(id: string): void {
	const task = getTask(id);
	if (task?.status === 'running') {
		updateTask(id, { status: 'paused' });
		log.info(`Task paused: ${task.label} (${id})`);
	}
}

export function resumeTask(id: string): void {
	const task = getTask(id);
	if (task?.status === 'paused') {
		updateTask(id, { status: 'running' });
		log.info(`Task resumed: ${task.label} (${id})`);
	}
}

export function cancelTask(id: string): void {
	const task = getTask(id);
	if (task && (task.status === 'running' || task.status === 'paused')) {
		updateTask(id, { status: 'cancelled' });
		log.info(`Task cancelled: ${task.label} (${id})`);
	}
}

export function dismissTask(id: string): void {
	removeTask(id);
}

export function elapsed(task: RunningTask): string {
	void tick; // reactive dependency — forces recalc every second
	const ms = Date.now() - task.startedAt;
	const s = Math.floor(ms / 1000);
	if (s < 60) return `${s}s`;
	return `${Math.floor(s / 60)}m ${s % 60}s`;
}

export function estimateRemaining(task: RunningTask): string {
	if (task.total <= 0 || task.current <= 0) return '';
	const elapsedMs = Date.now() - task.startedAt;
	const msPerItem = elapsedMs / task.current;
	const remaining = (task.total - task.current) * msPerItem;
	const s = Math.floor(remaining / 1000);
	if (s < 60) return `~${s}s left`;
	return `~${Math.floor(s / 60)}m ${s % 60}s left`;
}
