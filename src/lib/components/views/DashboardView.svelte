<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import PackageList from '$lib/components/PackageList.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import {
		getFilteredPackages,
		getPackages,
		getTotalOutdatedCount,
		getOutdatedPackages,
		getAvailableManagers,
		isLoading,
		getError,
		getSearchQuery,
		setSearchQuery,
		setActiveView,
		getLastRefreshed,
		refreshPackages
	} from '$lib/stores/packages.svelte';
	import { getDateFormat } from '$lib/stores/theme.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PackageManager } from '$lib/types';

	let localSearch = $state(getSearchQuery());
	let hoveredSegment = $state<string | null>(null);
	let storageData = $state<[string, string][]>([]);

	function handleSearch(value: string) {
		setSearchQuery(value);
	}

	function managerDisplayName(id: string): string {
		const names: Record<string, string> = {
			brew: 'Homebrew', npm: 'npm', mas: 'Mac App Store', pip: 'pip',
			cargo: 'Cargo', winget: 'winget', apt: 'apt', flatpak: 'Flatpak', snap: 'Snap'
		};
		return names[id] ?? id;
	}

	function managerIcon(id: string): IconName {
		const map: Record<string, IconName> = {
			brew: 'brew', npm: 'npm', winget: 'winget', mas: 'mas',
			pip: 'pip', cargo: 'cargo', apt: 'apt', flatpak: 'flatpak', snap: 'snap'
		};
		return map[id] ?? 'installed';
	}

	function managerColor(id: string): string {
		const colors: Record<string, string> = {
			brew: '#8fbcbb', npm: '#bf616a', mas: '#5e81ac', pip: '#ebcb8b',
			cargo: '#d08770', winget: '#81a1c1', apt: '#b48ead', flatpak: '#a3be8c', snap: '#88c0d0'
		};
		return colors[id] ?? '#4c566a';
	}

	function getManagerPackageCount(id: string): number {
		return getPackages().get(id as PackageManager)?.length ?? 0;
	}

	function getTotalPackageCount(): number {
		let total = 0;
		for (const pkgs of getPackages().values()) total += pkgs.length;
		return total;
	}

	// Donut chart segments
	const donutSegments = $derived((() => {
		const total = getTotalPackageCount();
		if (total === 0) return [];
		let cumulative = 0;
		return getAvailableManagers()
			.map((m) => {
				const count = getManagerPackageCount(m.id);
				const pct = count / total;
				const start = cumulative;
				cumulative += pct;
				return { id: m.id, count, pct, start, end: cumulative, color: managerColor(m.id) };
			})
			.filter((s) => s.count > 0);
	})());

	function donutArc(start: number, end: number, r: number, cx: number, cy: number): string {
		if (end - start >= 0.9999) {
			return `M ${cx} ${cy - r} A ${r} ${r} 0 1 1 ${cx - 0.001} ${cy - r} A ${r} ${r} 0 1 1 ${cx} ${cy - r}`;
		}
		const sa = start * 2 * Math.PI - Math.PI / 2;
		const ea = end * 2 * Math.PI - Math.PI / 2;
		const x1 = cx + r * Math.cos(sa), y1 = cy + r * Math.sin(sa);
		const x2 = cx + r * Math.cos(ea), y2 = cy + r * Math.sin(ea);
		return `M ${x1} ${y1} A ${r} ${r} 0 ${end - start > 0.5 ? 1 : 0} 1 ${x2} ${y2}`;
	}

	function formatDate(date: Date | null): string {
		if (!date) return '';
		const fmt = getDateFormat();
		const d = date.getDate().toString().padStart(2, '0');
		const m = (date.getMonth() + 1).toString().padStart(2, '0');
		const y = date.getFullYear();
		switch (fmt) {
			case 'dd/mm/yyyy': return `${d}/${m}/${y}`;
			case 'mm/dd/yyyy': return `${m}/${d}/${y}`;
			case 'yyyy-mm-dd': return `${y}-${m}-${d}`;
			default: return `${d}/${m}/${y}`;
		}
	}

	// Load storage info (once — now instant, no du)
	let storageLoaded = false;
	$effect(() => {
		if (storageLoaded) return;
		storageLoaded = true;
		if (typeof window !== 'undefined' && '__TAURI__' in window) {
			invoke<[string, string][]>('get_storage_info')
				.then((data) => { storageData = data; })
				.catch(() => { storageData = []; });
		}
	});

	function storageFor(id: string): string | null {
		const entry = storageData.find(([mgr]) => mgr === id);
		return entry ? entry[1] : null;
	}
</script>

<div class="flex h-full flex-col overflow-y-auto">
	<!-- Header -->
	<div class="flex items-center justify-between px-6 py-4">
		<div class="flex flex-col">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Dashboard</h1>
			{#if getLastRefreshed()}
				<span class="text-[11px]" style="color: var(--text-muted);">
					Last updated {formatDate(getLastRefreshed())}
				</span>
			{/if}
		</div>
	</div>

	{#if isLoading() && getTotalPackageCount() === 0}
		<div class="px-6 pt-4">
			<LoadingSkeleton rows={3} />
		</div>
	{:else if getError()}
		<div class="px-6">
			<EmptyState
				variant="error"
				title="Failed to load packages"
				message={getError() ?? 'An unknown error occurred. Check your connection and try again.'}
				actionLabel="Retry"
				onaction={refreshPackages}
			/>
		</div>
	{:else if !isLoading() && getAvailableManagers().length === 0}
		<div class="px-6">
			<EmptyState
				variant="warning"
				title="No package managers found"
				message="Install a package manager like Homebrew or npm to get started."
				actionLabel="Go to Managers"
				onaction={() => setActiveView('managers')}
			/>
		</div>
	{/if}

	<!-- Top stats + chart row -->
	<div class="flex gap-3 px-6">
		<!-- Left column: stats cards -->
		<div class="flex flex-col gap-3" style="min-width: 140px;">
			<div
				class="flex flex-col gap-1 rounded-lg border px-4 py-3"
				style="border-color: var(--border-subtle); background-color: var(--surface);"
			>
				<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Total</span>
				<span class="text-2xl font-bold" style="color: var(--text-primary);">{getTotalPackageCount()}</span>
				<span class="text-[10px]" style="color: var(--text-muted);">{getAvailableManagers().length} managers</span>
			</div>
			<button
				class="flex flex-col gap-1 rounded-lg border px-4 py-3 text-left transition-colors duration-100 hover:border-[var(--warning)]"
				style="border-color: var(--border-subtle); background-color: var(--surface);"
				onclick={() => setActiveView('outdated')}
			>
				<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Outdated</span>
				<span class="text-2xl font-bold" style={getTotalOutdatedCount() > 0 ? 'color: var(--warning);' : 'color: var(--text-primary);'}>
					{getTotalOutdatedCount()}
				</span>
				<span class="text-[10px]" style="color: var(--text-muted);">
					{getTotalOutdatedCount() > 0 ? 'updates available' : 'all up to date'}
				</span>
			</button>
		</div>

		<!-- Donut chart -->
		<div
			class="flex flex-col items-center gap-2 rounded-lg border px-4 py-3"
			style="border-color: var(--border-subtle); background-color: var(--surface); min-width: 180px;"
		>
			<svg width="140" height="140" viewBox="0 0 120 120">
				{#each donutSegments as seg}
					<path
						d={donutArc(seg.start, seg.end, 45, 60, 60)}
						fill="none"
						stroke={seg.color}
						stroke-width={hoveredSegment === seg.id ? 18 : 14}
						stroke-linecap="butt"
						role="img"
						aria-label="{managerDisplayName(seg.id)}: {seg.count} packages"
						style="transition: stroke-width 0.15s ease; cursor: pointer;"
						onmouseenter={() => (hoveredSegment = seg.id)}
						onmouseleave={() => (hoveredSegment = null)}
					/>
				{/each}
				{#if hoveredSegment}
					{@const seg = donutSegments.find((s) => s.id === hoveredSegment)}
					{#if seg}
						<text x="60" y="54" text-anchor="middle" font-size="18" font-weight="700" fill="var(--text-primary)">
							{seg.count}
						</text>
						<text x="60" y="70" text-anchor="middle" font-size="8" fill={seg.color}>
							{managerDisplayName(seg.id)}
						</text>
					{/if}
				{:else}
					<text x="60" y="56" text-anchor="middle" font-size="20" font-weight="700" fill="var(--text-primary)">
						{getTotalPackageCount()}
					</text>
					<text x="60" y="72" text-anchor="middle" font-size="9" fill="var(--text-muted)">
						packages
					</text>
				{/if}
			</svg>
			<div class="flex flex-wrap justify-center gap-x-3 gap-y-0.5">
				{#each donutSegments as seg}
					<button
						class="flex items-center gap-1 rounded px-1 py-0.5 transition-colors duration-75"
						style={hoveredSegment === seg.id ? `background-color: ${seg.color}22;` : ''}
						onmouseenter={() => (hoveredSegment = seg.id)}
						onmouseleave={() => (hoveredSegment = null)}
					>
						<div class="h-1.5 w-1.5 rounded-full" style={`background-color: ${seg.color};`}></div>
						<span class="text-[9px]" style="color: var(--text-muted);">{managerDisplayName(seg.id)}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Storage info -->
		<div
			class="flex flex-1 flex-col gap-2 rounded-lg border px-4 py-3"
			style="border-color: var(--border-subtle); background-color: var(--surface);"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Disk Usage</span>
			{#if storageData.length > 0}
				<div class="flex flex-col gap-1.5">
					{#each storageData as [mgr, size]}
						<div class="flex items-center gap-2">
							<span class="flex w-4 items-center justify-center" style="color: var(--text-muted);">
								<Icon name={managerIcon(mgr)} size={12} />
							</span>
							<span class="flex-1 text-[11px]" style="color: var(--text-secondary);">{managerDisplayName(mgr)}</span>
							<span class="font-mono text-[12px] font-semibold" style="color: var(--text-primary);">{size}</span>
						</div>
					{/each}
				</div>
			{:else}
				<div class="flex flex-col gap-1.5">
					{#each getAvailableManagers() as m}
						{@const count = getManagerPackageCount(m.id)}
						{#if count > 0}
							<div class="flex items-center gap-2">
								<span class="flex w-4 items-center justify-center" style="color: var(--text-muted);">
									<Icon name={managerIcon(m.id)} size={12} />
								</span>
								<span class="flex-1 text-[11px]" style="color: var(--text-secondary);">{managerDisplayName(m.id)}</span>
								<span class="font-mono text-[12px] font-semibold" style="color: var(--text-primary);">{count} pkgs</span>
							</div>
						{/if}
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<!-- Search -->
	<div class="px-6 pt-4 pb-2">
		<SearchBar bind:value={localSearch} oninput={handleSearch} />
	</div>

	<!-- Package List -->
	<div class="min-h-0 flex-1">
		{#if isLoading()}
			<LoadingSkeleton />
		{:else}
			<PackageList
				packages={getFilteredPackages()}
				emptyTitle={getSearchQuery() ? 'No matching packages' : 'No packages loaded'}
				emptyMessage={getSearchQuery() ? 'Try a different search term' : 'Packages will appear here once loaded'}
			/>
		{/if}
	</div>
</div>
