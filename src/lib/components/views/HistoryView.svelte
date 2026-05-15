<script lang="ts">
	import { getHistory, loadHistory, type HistoryEntry } from '$lib/stores/history.svelte';
	import { getDateFormat } from '$lib/stores/theme.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';

	$effect(() => {
		loadHistory();
	});

	function actionIcon(action: string): IconName {
		switch (action) {
			case 'install': return 'installed';
			case 'uninstall': return 'close';
			case 'update': return 'outdated';
			case 'cleanup': return 'cleanup';
			default: return 'logs';
		}
	}

	function actionColor(action: string): string {
		switch (action) {
			case 'install': return 'var(--success)';
			case 'uninstall': return 'var(--error)';
			case 'update': return 'var(--accent)';
			case 'cleanup': return 'var(--warning)';
			default: return 'var(--text-muted)';
		}
	}

	function formatTimestamp(iso: string): string {
		const d = new Date(iso);
		const fmt = getDateFormat();
		const day = d.getDate().toString().padStart(2, '0');
		const month = (d.getMonth() + 1).toString().padStart(2, '0');
		const year = d.getFullYear();
		const time = d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

		let date: string;
		switch (fmt) {
			case 'dd/mm/yyyy': date = `${day}/${month}/${year}`; break;
			case 'mm/dd/yyyy': date = `${month}/${day}/${year}`; break;
			case 'yyyy-mm-dd': date = `${year}-${month}-${day}`; break;
			default: date = `${day}/${month}/${year}`;
		}
		return `${date} ${time}`;
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex items-center justify-between px-6 py-4">
		<div class="flex flex-col">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">History</h1>
			<span class="text-[11px]" style="color: var(--text-muted);">
				{getHistory().length} actions recorded
			</span>
		</div>
	</div>

	<div class="flex-1 overflow-y-auto">
		{#if getHistory().length === 0}
			<EmptyState title="No history yet" message="Actions will be recorded as you install, update, and remove packages" />
		{:else}
			<div class="flex flex-col">
				{#each getHistory() as entry}
					<div class="flex items-center gap-3 border-b px-5 py-2.5" style="border-color: var(--border-subtle);">
						<div
							class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full"
							style={`background-color: ${actionColor(entry.action)}22;`}
						>
							<span style={`color: ${actionColor(entry.action)};`}>
								<Icon name={actionIcon(entry.action)} size={12} />
							</span>
						</div>
						<div class="min-w-0 flex-1">
							<span class="text-[13px] font-medium" style="color: var(--text-primary);">
								{entry.action}
							</span>
							<span class="text-[13px]" style="color: var(--text-secondary);">
								{entry.package}
							</span>
							<span class="text-[11px]" style="color: var(--text-muted);">
								via {entry.manager}
							</span>
						</div>
						<span class="flex-shrink-0 text-[10px]" style="color: var(--text-muted);">
							{formatTimestamp(entry.timestamp)}
						</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
