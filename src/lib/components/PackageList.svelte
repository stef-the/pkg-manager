<script lang="ts">
	import type { Package, PackageManager } from '$lib/types';
	import { getSelectedPackage, setSelectedPackage, isPackageOutdated } from '$lib/stores/packages.svelte';
	import EmptyState from './EmptyState.svelte';
	import Icon from './Icons.svelte';

	let {
		packages,
		emptyTitle = 'No packages found',
		emptyMessage = '',
		showViewToggle = true
	}: {
		packages: Package[];
		emptyTitle?: string;
		emptyMessage?: string;
		showViewToggle?: boolean;
	} = $props();

	type SortKey = 'name' | 'version' | 'manager';
	type SortDir = 'asc' | 'desc';
	type ViewMode = 'list' | 'grid';

	let sortKey = $state<SortKey>('name');
	let sortDir = $state<SortDir>('asc');
	let viewMode = $state<ViewMode>('list');

	function toggleSort(key: SortKey) {
		if (sortKey === key) {
			sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		} else {
			sortKey = key;
			sortDir = 'asc';
		}
	}

	const sortedPackages = $derived((() => {
		return [...packages].sort((a, b) => {
			let cmp = 0;
			switch (sortKey) {
				case 'name':
					cmp = a.name.localeCompare(b.name);
					break;
				case 'version':
					cmp = a.version.localeCompare(b.version);
					break;
				case 'manager':
					cmp = a.manager.localeCompare(b.manager);
					break;
			}
			return sortDir === 'asc' ? cmp : -cmp;
		});
	})());

	function managerColor(id: string): string {
		switch (id) {
			case 'brew': return 'var(--color-nord7)';
			case 'npm': return 'var(--color-nord11)';
			case 'winget': return 'var(--color-nord9)';
			default: return 'var(--text-muted)';
		}
	}

	function managerLabel(id: string): string {
		return id === 'brew' ? 'brew' : 'npm';
	}
</script>

{#if packages.length === 0}
	<EmptyState title={emptyTitle} message={emptyMessage} />
{:else}
	<!-- Toolbar -->
	{#if showViewToggle}
		<div class="flex items-center justify-between border-b px-4 py-1.5" style="border-color: var(--border-subtle);">
			<span class="text-[11px]" style="color: var(--text-muted);">{packages.length} packages</span>
			<div class="flex items-center gap-1 rounded-md p-0.5" style="background-color: var(--bg-primary);">
				<button
					class="rounded px-2 py-0.5 text-[10px] font-medium transition-colors duration-100"
					style={viewMode === 'list' ? 'background-color: var(--accent); color: var(--bg-primary);' : 'color: var(--text-muted);'}
					onclick={() => (viewMode = 'list')}
				>
					List
				</button>
				<button
					class="rounded px-2 py-0.5 text-[10px] font-medium transition-colors duration-100"
					style={viewMode === 'grid' ? 'background-color: var(--accent); color: var(--bg-primary);' : 'color: var(--text-muted);'}
					onclick={() => (viewMode = 'grid')}
				>
					Grid
				</button>
			</div>
		</div>
	{/if}

	{#if viewMode === 'list'}
		<!-- Column Headers -->
		<div class="flex items-center gap-4 border-b px-4 py-1.5" style="border-color: var(--border-subtle);">
			<button
				class="flex w-40 items-center gap-1 text-left text-[10px] font-medium uppercase tracking-wider transition-colors duration-100"
				style="color: var(--text-muted);"
				onclick={() => toggleSort('name')}
			>
				Name
				{#if sortKey === 'name'}
					<svg width="8" height="8" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
						{#if sortDir === 'asc'}<path d="M4 10l4-5 4 5" />{:else}<path d="M4 6l4 5 4-5" />{/if}
					</svg>
				{/if}
			</button>
			<button
				class="flex w-24 items-center gap-1 text-left text-[10px] font-medium uppercase tracking-wider transition-colors duration-100"
				style="color: var(--text-muted);"
				onclick={() => toggleSort('version')}
			>
				Version
				{#if sortKey === 'version'}
					<svg width="8" height="8" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
						{#if sortDir === 'asc'}<path d="M4 10l4-5 4 5" />{:else}<path d="M4 6l4 5 4-5" />{/if}
					</svg>
				{/if}
			</button>
			<span class="flex-1 text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Description</span>
			<button
				class="flex w-14 items-center justify-end gap-1 text-right text-[10px] font-medium uppercase tracking-wider transition-colors duration-100"
				style="color: var(--text-muted);"
				onclick={() => toggleSort('manager')}
			>
				Mgr
				{#if sortKey === 'manager'}
					<svg width="8" height="8" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
						{#if sortDir === 'asc'}<path d="M4 10l4-5 4 5" />{:else}<path d="M4 6l4 5 4-5" />{/if}
					</svg>
				{/if}
			</button>
		</div>

		<!-- List rows -->
		<div class="flex flex-col overflow-y-auto">
			{#each sortedPackages as pkg (pkg.manager + '/' + pkg.name)}
				{@const isSelected =
					getSelectedPackage()?.name === pkg.name &&
					getSelectedPackage()?.manager === pkg.manager}
				{@const outdated = isPackageOutdated(pkg.name, pkg.manager as PackageManager)}

				<button
					class="flex items-center gap-4 border-b px-4 py-2.5 text-left transition-colors duration-75 hover:bg-[var(--bg-hover)]"
					style={`border-color: var(--border-subtle); ${
						isSelected
							? 'background-color: var(--accent); color: var(--bg-primary);'
							: ''
					}`}
					onclick={() => setSelectedPackage(pkg)}
				>
					<!-- Manager icon as package icon fallback -->
					<span class="flex w-4 items-center justify-center" style={isSelected ? '' : 'opacity: 0.4;'}>
						<Icon name={pkg.manager === 'brew' ? 'brew' : pkg.manager === 'winget' ? 'winget' : 'npm'} size={14} />
					</span>

					<span
						class="w-36 min-w-0 truncate text-[13px] font-semibold"
						style={isSelected ? '' : `color: var(--text-primary);`}
					>
						{pkg.name}
					</span>

					<span
						class="w-24 min-w-0 truncate font-mono text-[12px]"
						style={isSelected ? 'opacity: 0.8;' : `color: var(--text-muted);`}
					>
						{pkg.version || '—'}
					</span>

					<span
						class="min-w-0 flex-1 truncate text-[12px]"
						style={isSelected ? 'opacity: 0.8;' : `color: var(--text-secondary);`}
					>
						{pkg.description || ''}
					</span>

					<div class="flex items-center gap-2">
						{#if outdated}
							<span
								class="rounded-full px-2 py-0.5 text-[10px] font-semibold"
								style={isSelected
									? 'background-color: var(--bg-primary); color: var(--accent);'
									: 'background-color: var(--warning); color: var(--color-nord0);'}
							>
								{outdated.latestVersion}
							</span>
						{/if}
						<span
							class="rounded-full px-2 py-0.5 text-[10px] font-semibold"
							style={isSelected
								? 'background-color: var(--bg-primary); color: var(--accent);'
								: `background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}
						>
							{managerLabel(pkg.manager)}
						</span>
					</div>
				</button>
			{/each}
		</div>
	{:else}
		<!-- Grid view -->
		<div class="grid grid-cols-3 gap-3 overflow-y-auto p-4">
			{#each sortedPackages as pkg (pkg.manager + '/' + pkg.name)}
				{@const isSelected =
					getSelectedPackage()?.name === pkg.name &&
					getSelectedPackage()?.manager === pkg.manager}
				{@const outdated = isPackageOutdated(pkg.name, pkg.manager as PackageManager)}

				<button
					class="flex flex-col gap-2 rounded-lg border p-3 text-left transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style={`border-color: ${isSelected ? 'var(--accent)' : 'var(--border-subtle)'}; ${
						isSelected ? 'background-color: var(--accent); color: var(--bg-primary);' : ''
					}`}
					onclick={() => setSelectedPackage(pkg)}
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-2">
							<span style={isSelected ? '' : 'opacity: 0.4;'}>
								<Icon name={pkg.manager === 'brew' ? 'brew' : pkg.manager === 'winget' ? 'winget' : 'npm'} size={16} />
							</span>
							<span class="text-[13px] font-semibold" style={isSelected ? '' : 'color: var(--text-primary);'}>
								{pkg.name}
							</span>
						</div>
						{#if outdated}
							<span
								class="rounded-full px-1.5 py-0.5 text-[9px] font-semibold"
								style={isSelected
									? 'background-color: var(--bg-primary); color: var(--accent);'
									: 'background-color: var(--warning); color: var(--color-nord0);'}
							>
								update
							</span>
						{/if}
					</div>
					<span
						class="line-clamp-2 text-[11px]"
						style={isSelected ? 'opacity: 0.8;' : 'color: var(--text-muted);'}
					>
						{pkg.description || 'No description'}
					</span>
					<div class="flex items-center justify-between">
						<span class="font-mono text-[10px]" style={isSelected ? 'opacity: 0.7;' : 'color: var(--text-muted);'}>
							{pkg.version || '—'}
						</span>
						<span
							class="rounded-full px-1.5 py-0.5 text-[9px] font-semibold"
							style={isSelected
								? 'background-color: var(--bg-primary); color: var(--accent);'
								: `background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}
						>
							{managerLabel(pkg.manager)}
						</span>
					</div>
				</button>
			{/each}
		</div>
	{/if}
{/if}

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
