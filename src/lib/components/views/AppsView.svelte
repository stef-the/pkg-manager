<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { addToast } from '$lib/stores/toast.svelte';
	import { runTask } from '$lib/stores/tasks.svelte';
	import { createLogger } from '$lib/utils/logger';

	const log = createLogger('apps');

	interface InstalledApp {
		name: string;
		version: string;
		path: string;
		sizeBytes: number;
		appType: string;
		canUninstall: boolean;
	}

	let apps = $state<InstalledApp[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let confirmOpen = $state(false);
	let uninstallTarget = $state<InstalledApp | null>(null);

	const filteredApps = $derived((() => {
		if (!searchQuery.trim()) return apps;
		const q = searchQuery.toLowerCase();
		return apps.filter((a) => a.name.toLowerCase().includes(q));
	})());

	$effect(() => {
		scanApps();
	});

	async function scanApps() {
		loading = true;
		try {
			apps = await invoke<InstalledApp[]>('scan_installed_apps');
		} catch (e) {
			log.error(`Failed to scan apps: ${e}`);
			apps = [];
		} finally {
			loading = false;
		}
	}

	function formatSize(bytes: number): string {
		if (bytes === 0) return '';
		if (bytes >= 1_000_000_000) return `${(bytes / 1_000_000_000).toFixed(1)} GB`;
		if (bytes >= 1_000_000) return `${(bytes / 1_000_000).toFixed(0)} MB`;
		if (bytes >= 1_000) return `${(bytes / 1_000).toFixed(0)} KB`;
		return `${bytes} B`;
	}

	function requestUninstall(app: InstalledApp) {
		uninstallTarget = app;
		confirmOpen = true;
	}

	function doUninstall() {
		if (!uninstallTarget) return;
		const app = uninstallTarget;
		runTask(`Removing ${app.name}`, () => invoke('uninstall_app', { path: app.path }), {
			successMessage: `${app.name} moved to Trash`,
			onSuccess: () => {
				apps = apps.filter((a) => a.path !== app.path);
			}
		});
		uninstallTarget = null;
	}

	function typeColor(type: string): string {
		switch (type) {
			case 'system': return 'var(--text-muted)';
			case 'app': return 'var(--accent)';
			default: return 'var(--text-secondary)';
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex items-center justify-between px-6 py-4">
		<div class="flex flex-col">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Installed Apps</h1>
			<span class="text-[11px]" style="color: var(--text-muted);">
				{apps.length} apps found on your system
			</span>
		</div>
		<button
			class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
			style="background-color: var(--accent); color: var(--bg-primary);"
			onclick={scanApps}
		>
			Rescan
		</button>
	</div>

	<div class="px-6 pb-2">
		<SearchBar bind:value={searchQuery} placeholder="Filter apps..." />
	</div>

	<div class="flex-1 overflow-y-auto">
		{#if loading}
			<LoadingSkeleton rows={10} />
		{:else if filteredApps.length === 0}
			<EmptyState
				title={searchQuery ? 'No matching apps' : 'No apps found'}
				message={searchQuery ? 'Try a different filter' : 'Could not scan applications on this system'}
			/>
		{:else}
			<!-- Header -->
			<div class="flex items-center gap-4 border-b px-4 py-1.5" style="border-color: var(--border-subtle);">
				<span class="w-4 flex-shrink-0"></span>
				<span class="w-48 text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Name</span>
				<span class="w-24 text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Version</span>
				<span class="w-20 text-right text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Size</span>
				<span class="flex-1 text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Type</span>
				<span class="w-20"></span>
			</div>

			{#each filteredApps as app (app.path || app.name)}
				<div
					class="flex items-center gap-4 border-b px-4 py-2 transition-colors duration-75 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border-subtle);"
				>
					<!-- Icon placeholder -->
					<span class="flex h-4 w-4 flex-shrink-0 items-center justify-center rounded" style="background-color: {typeColor(app.appType)}22; color: {typeColor(app.appType)};">
						<svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor" stroke="none">
							<rect x="2" y="2" width="12" height="12" rx="3" />
						</svg>
					</span>

					<span class="w-48 min-w-0 truncate text-[13px] font-medium" style="color: var(--text-primary);">
						{app.name}
					</span>

					<span class="w-24 min-w-0 truncate font-mono text-[11px]" style="color: var(--text-muted);">
						{app.version || '—'}
					</span>

					<span class="w-20 text-right font-mono text-[11px]" style="color: var(--text-muted);">
						{formatSize(app.sizeBytes)}
					</span>

					<span class="flex-1">
						<span
							class="rounded-full px-2 py-0.5 text-[9px] font-semibold"
							style={`background-color: ${typeColor(app.appType)}22; color: ${typeColor(app.appType)};`}
						>
							{app.appType}
						</span>
					</span>

					<span class="w-20 text-right">
						{#if app.canUninstall}
							<button
								class="rounded-md px-2.5 py-1 text-[10px] font-medium transition-colors duration-100 hover:bg-[var(--error)] hover:text-[var(--bg-primary)]"
								style="color: var(--error);"
								onclick={() => requestUninstall(app)}
							>
								Remove
							</button>
						{:else}
							<span class="text-[10px]" style="color: var(--text-muted);">System</span>
						{/if}
					</span>
				</div>
			{/each}
		{/if}
	</div>
</div>

<ConfirmDialog
	bind:open={confirmOpen}
	title="Remove {uninstallTarget?.name ?? ''}?"
	message="This will move the app to Trash. You can restore it from Trash if needed."
	confirmLabel="Move to Trash"
	variant="danger"
	onconfirm={doUninstall}
/>
