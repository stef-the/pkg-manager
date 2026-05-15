<script lang="ts">
	import { createLogger } from '$lib/utils/logger';
	import { addToast } from '$lib/stores/toast.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import { invoke } from '@tauri-apps/api/core';

	const log = createLogger('logs');

	let logContent = $state('');
	let loading = $state(false);
	let loadError = $state<string | null>(null);

	async function loadLogs() {
		loading = true;
		loadError = null;
		try {
			const content = await invoke<string>('read_log_file');
			logContent = content;
		} catch (e) {
			loadError = `${e}`;
			logContent = '';
			log.error(`Failed to load logs: ${e}`);
		} finally {
			loading = false;
		}
	}

	async function openLogDir() {
		try {
			await invoke<void>('open_log_directory');
		} catch (e) {
			addToast(`Failed to open log directory: ${e}`, 'error');
			log.error(`Failed to open log directory: ${e}`);
		}
	}

	$effect(() => {
		loadLogs();
	});
</script>

<div class="flex h-full flex-col">
	<div class="flex items-center justify-between border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<div class="flex flex-col gap-1">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">View Logs</h1>
			<p class="text-[13px]" style="color: var(--text-muted);">
				Application logs for troubleshooting.
			</p>
		</div>
		<div class="flex gap-2">
			<button
				class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style="border-color: var(--border); color: var(--text-primary);"
				onclick={openLogDir}
			>
				Open Folder
			</button>
			<button
				class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
				style="background-color: var(--accent); color: var(--bg-primary);"
				onclick={loadLogs}
			>
				Refresh
			</button>
		</div>
	</div>

	<div class="flex-1 overflow-y-auto p-6">
		{#if loading}
			<LoadingSkeleton rows={12} />
		{:else if loadError}
			<EmptyState
				variant="error"
				title="Failed to load logs"
				message={loadError}
				actionLabel="Retry"
				onaction={loadLogs}
			/>
		{:else if !logContent}
			<EmptyState
				title="No logs available"
				message="Application logs will appear here once there is activity."
			/>
		{:else}
			<pre
				class="h-full overflow-auto rounded-lg p-4 font-mono text-[11px] leading-relaxed"
				style="background-color: var(--bg-primary); color: var(--text-secondary);"
			>{logContent}</pre>
		{/if}
	</div>
</div>
