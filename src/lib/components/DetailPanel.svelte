<script lang="ts">
	import {
		getSelectedPackage,
		setSelectedPackage,
		isPackageOutdated,
		updatePkg,
		uninstallPkg,
		getSystemStats,
		getAvailableManagers,
		getPackages,
		getTotalOutdatedCount,
		getLastRefreshed
	} from '$lib/stores/packages.svelte';
	import {
		getTasks, dismissTask, hasRunningTasks, pauseTask, resumeTask, cancelTask,
		elapsed, estimateRemaining, type RunningTask
	} from '$lib/stores/tasks.svelte';
	import { getDateFormat } from '$lib/stores/theme.svelte';
	import PackageInfoModal from '$lib/components/PackageInfoModal.svelte';
	import { isPinned, togglePin } from '$lib/stores/pinned.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PackageManager } from '$lib/types';

	let infoModalOpen = $state(false);
	let infoModalManager = $state('');
	let infoModalName = $state('');

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew':
				return 'Homebrew';
			case 'npm':
				return 'npm';
			default:
				return id;
		}
	}

	function managerColor(id: string): string {
		return id === 'brew' ? 'var(--color-nord7)' : 'var(--color-nord11)';
	}

	function getManagerCount(id: string): number {
		return getPackages().get(id as PackageManager)?.length ?? 0;
	}

	function formatTime(date: Date | null): string {
		if (!date) return 'Never';
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	function formatDate(date: Date | null): string {
		if (!date) return '';
		const fmt = getDateFormat();
		const d = date.getDate().toString().padStart(2, '0');
		const m = (date.getMonth() + 1).toString().padStart(2, '0');
		const y = date.getFullYear();
		switch (fmt) {
			case 'dd/mm/yyyy':
				return `${d}/${m}/${y}`;
			case 'mm/dd/yyyy':
				return `${m}/${d}/${y}`;
			case 'yyyy-mm-dd':
				return `${y}-${m}-${d}`;
			default:
				return `${d}/${m}/${y}`;
		}
	}
</script>

<aside
	class="flex h-full w-[300px] min-w-[300px] flex-col border-l"
	style="background-color: var(--bg-secondary); border-color: var(--border-subtle);"
>
	{#if getSelectedPackage()}
		{@const pkg = getSelectedPackage()!}
		{@const outdated = isPackageOutdated(pkg.name, pkg.manager as PackageManager)}
		{@const busy = hasRunningTasks()}

		<!-- Header -->
		<div
			class="flex items-center justify-between border-b px-4 py-3"
			style="border-color: var(--border-subtle);"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				Package Details
			</span>
			<div class="flex items-center gap-1">
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md text-xs transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style={isPinned(pkg.manager, pkg.name) ? 'color: var(--warning);' : 'color: var(--text-muted);'}
					aria-label={isPinned(pkg.manager, pkg.name) ? 'Unpin package' : 'Pin package'}
					onclick={() => togglePin(pkg.manager, pkg.name)}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill={isPinned(pkg.manager, pkg.name) ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M5 2l6 0 0 4 1.5 2H3.5L5 6z" /><path d="M8 8v6" />
					</svg>
				</button>
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md text-xs transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="color: var(--text-muted);"
					aria-label="Close details"
					onclick={() => setSelectedPackage(null)}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
				</button>
			</div>
		</div>

		<!-- Content -->
		<div class="flex flex-1 flex-col gap-4 overflow-y-auto px-4 py-4">
			<div class="flex flex-col gap-1.5">
				<h2 class="text-base font-semibold" style="color: var(--text-primary);">{pkg.name}</h2>
				<div class="flex items-center gap-2">
					<span
						class="inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-semibold"
						style={`background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}
					>
						{managerDisplayName(pkg.manager)}
					</span>
					{#if isPinned(pkg.manager, pkg.name)}
						<span
							class="inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-semibold"
							style="background-color: var(--warning); color: var(--color-nord0);"
						>
							Pinned
						</span>
					{:else if outdated}
						<span
							class="inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-semibold"
							style="background-color: var(--warning); color: var(--color-nord0);"
						>
							Update available
						</span>
					{/if}
				</div>
			</div>

			<div class="flex flex-col gap-3">
				<div class="flex flex-col gap-0.5">
					<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
						Version
					</span>
					<span class="font-mono text-[13px]" style="color: var(--text-primary);">{pkg.version || 'Unknown'}</span>
				</div>

				{#if outdated}
					<div class="flex flex-col gap-0.5">
						<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
							Latest
						</span>
						<span class="font-mono text-[13px]" style="color: var(--success);">
							{outdated.latestVersion}
						</span>
					</div>
				{/if}

				{#if pkg.description}
					<div class="flex flex-col gap-0.5">
						<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
							Description
						</span>
						<p class="text-[12px] leading-relaxed" style="color: var(--text-secondary);">
							{pkg.description}
						</p>
					</div>
				{/if}
			</div>
		</div>

		<!-- Actions -->
		<div class="flex flex-col gap-2 border-t px-4 py-3" style="border-color: var(--border-subtle);">
			<!-- Link buttons -->
			<div class="flex gap-2">
				<button
					class="flex flex-1 items-center justify-center gap-1.5 rounded-lg border px-3 py-2 text-[12px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={async () => {
						const url = await invoke<string>('get_package_info_url', { manager: pkg.manager, name: pkg.name });
						await invoke('open_url', { url });
					}}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3H3v10h10v-3" /><path d="M9 2h5v5" /><path d="M14 2L7 9" /></svg>
					Website
				</button>
				<button
					class="flex flex-1 items-center justify-center gap-1.5 rounded-lg border px-3 py-2 text-[12px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={() => {
						infoModalManager = pkg.manager;
						infoModalName = pkg.name;
						infoModalOpen = true;
					}}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="6" /><path d="M8 5v0.5" /><path d="M8 7.5v4" /></svg>
					View More
				</button>
			</div>

			{#if outdated}
				<button
					class="w-full rounded-lg px-3 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
					style="background-color: var(--accent); color: var(--bg-primary);"
					onclick={() => updatePkg(pkg.manager as PackageManager, pkg.name)}
					disabled={busy}
				>
					Update to {outdated.latestVersion}
				</button>
			{/if}
			<button
				class="w-full rounded-lg border px-3 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--error)] hover:text-[var(--bg-primary)]"
				style="border-color: var(--error); color: var(--error);"
				onclick={() => uninstallPkg(pkg.manager as PackageManager, pkg.name)}
				disabled={busy}
			>
				Uninstall
			</button>
		</div>
	{:else}
		<!-- System Stats (idle state) -->
		<div
			class="flex items-center border-b px-4 py-3"
			style="border-color: var(--border-subtle);"
		>
			<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				System
			</span>
		</div>

		<div class="flex flex-1 flex-col gap-5 overflow-y-auto px-4 py-4">
			<!-- System Info -->
			{#if getSystemStats()}
				{@const stats = getSystemStats()!}
				<div class="flex flex-col gap-2.5">
					<div class="flex flex-col gap-0.5">
						<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
							Host
						</span>
						<span class="text-[13px]" style="color: var(--text-primary);">{stats.hostname}</span>
					</div>
					<div class="flex flex-col gap-0.5">
						<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
							Platform
						</span>
						<span class="text-[13px]" style="color: var(--text-primary);">{stats.os}</span>
					</div>
				</div>
			{/if}

			<!-- Manager Versions -->
			<div class="flex flex-col gap-2.5">
				<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
					Package Managers
				</span>
				{#each getAvailableManagers() as manager}
					<div
						class="flex items-center justify-between rounded-lg border px-3 py-2"
						style="border-color: var(--border-subtle);"
					>
						<div class="flex items-center gap-2">
							<span
								class="h-2 w-2 rounded-full"
								style={`background-color: ${managerColor(manager.id)};`}
							></span>
							<span class="text-[13px] font-medium" style="color: var(--text-primary);">
								{managerDisplayName(manager.id)}
							</span>
						</div>
						<span class="font-mono text-[11px]" style="color: var(--text-muted);">
							{manager.version || '—'}
						</span>
					</div>
				{/each}
			</div>

			<!-- Quick Stats -->
			<div class="flex flex-col gap-2.5">
				<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
					Overview
				</span>
				{#each getAvailableManagers() as manager}
					<div class="flex items-center justify-between">
						<span class="text-[12px]" style="color: var(--text-secondary);">
							{managerDisplayName(manager.id)} packages
						</span>
						<span class="font-mono text-[12px] font-medium" style="color: var(--text-primary);">
							{getManagerCount(manager.id)}
						</span>
					</div>
				{/each}
				<div class="flex items-center justify-between">
					<span class="text-[12px]" style="color: var(--text-secondary);">
						Outdated
					</span>
					<span
						class="font-mono text-[12px] font-medium"
						style={getTotalOutdatedCount() > 0
							? 'color: var(--warning);'
							: 'color: var(--text-primary);'}
					>
						{getTotalOutdatedCount()}
					</span>
				</div>
			</div>

			<!-- Last Refresh -->
			{#if getLastRefreshed()}
				<div class="flex flex-col gap-0.5">
					<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
						Last Refreshed
					</span>
					<span class="text-[12px]" style="color: var(--text-secondary);">
						{formatDate(getLastRefreshed())} {formatTime(getLastRefreshed())}
					</span>
				</div>
			{/if}

		</div>
	{/if}

	<!-- Activity — always visible at bottom -->
	{#if getTasks().length > 0}
		<div class="flex max-h-[40%] flex-col gap-2 overflow-y-auto border-t px-4 py-3" style="border-color: var(--border-subtle);">
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				Activity
			</span>
			{#each getTasks() as task (task.id)}
				<div
					class="relative flex flex-col gap-1.5 overflow-hidden rounded-lg border px-3 py-2"
					style="border-color: var(--border-subtle);"
				>
					<div class="flex items-center gap-2">
						{#if task.status === 'running'}
							<svg class="flex-shrink-0 animate-spin" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M8 2a6 6 0 110 12" /></svg>
						{:else if task.status === 'paused'}
							<svg class="flex-shrink-0" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--warning)" stroke-width="2"><rect x="4" y="3" width="3" height="10" rx="0.5" /><rect x="9" y="3" width="3" height="10" rx="0.5" /></svg>
						{:else if task.status === 'done'}
							<svg class="flex-shrink-0" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--success)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 8.5l3.5 3.5L13 5" /></svg>
						{:else if task.status === 'cancelled'}
							<svg class="flex-shrink-0" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--text-muted)" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
						{:else}
							<svg class="flex-shrink-0" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--error)" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
						{/if}
						<span class="min-w-0 flex-1 truncate text-[11px] font-medium" style="color: var(--text-primary);">
							{task.label}
						</span>
						<span class="flex-shrink-0 font-mono text-[10px]" style="color: var(--text-muted);">
							{elapsed(task)}
						</span>
					</div>

					{#if task.currentLabel && (task.status === 'running' || task.status === 'paused')}
						<span class="truncate text-[10px]" style="color: var(--text-muted);">
							{task.currentLabel}
						</span>
					{/if}

					{#if task.total > 0 && (task.status === 'running' || task.status === 'paused')}
						<div class="flex items-center gap-2">
							<div class="h-1.5 flex-1 overflow-hidden rounded-full" style="background-color: var(--bg-primary);">
								<div
									class="h-full rounded-full transition-all duration-300"
									style={`background-color: ${task.status === 'paused' ? 'var(--warning)' : 'var(--accent)'}; width: ${task.progress}%;`}
								></div>
							</div>
							<span class="flex-shrink-0 font-mono text-[9px]" style="color: var(--text-muted);">
								{task.current}/{task.total}
							</span>
						</div>
						{#if estimateRemaining(task)}
							<span class="text-[9px]" style="color: var(--text-muted);">
								{estimateRemaining(task)}
							</span>
						{/if}
					{:else if task.status === 'running'}
						<div class="h-1 overflow-hidden rounded-full" style="background-color: var(--bg-primary);">
							<div class="h-full animate-pulse rounded-full" style="background-color: var(--accent); width: 100%;"></div>
						</div>
					{/if}

					{#if task.status === 'running' || task.status === 'paused'}
						<div class="flex gap-1.5">
							{#if task.status === 'running'}
								<button class="text-[10px] font-medium hover:opacity-80" style="color: var(--warning);" onclick={() => pauseTask(task.id)}>Pause</button>
							{:else}
								<button class="text-[10px] font-medium hover:opacity-80" style="color: var(--accent);" onclick={() => resumeTask(task.id)}>Resume</button>
							{/if}
							<button class="text-[10px] font-medium hover:opacity-80" style="color: var(--error);" onclick={() => cancelTask(task.id)}>Cancel</button>
						</div>
					{:else}
						<button class="w-fit text-[10px] font-medium hover:opacity-80" style="color: var(--text-muted);" onclick={() => dismissTask(task.id)}>Dismiss</button>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</aside>

<PackageInfoModal
	bind:open={infoModalOpen}
	manager={infoModalManager}
	packageName={infoModalName}
/>
