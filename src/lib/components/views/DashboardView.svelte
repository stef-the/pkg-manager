<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import PackageList from '$lib/components/PackageList.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import {
		getFilteredPackages,
		getPackages,
		getTotalOutdatedCount,
		getOutdatedPackages,
		getAvailableManagers,
		isLoading,
		getSearchQuery,
		setSearchQuery,
		setActiveView,
		getLastRefreshed
	} from '$lib/stores/packages.svelte';
	import { getDateFormat } from '$lib/stores/theme.svelte';
	import type { PackageManager } from '$lib/types';

	let localSearch = $state(getSearchQuery());

	function handleSearch(value: string) {
		setSearchQuery(value);
	}

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew': return 'Homebrew';
			case 'npm': return 'npm';
			case 'mas': return 'Mac App Store';
			case 'pip': return 'pip';
			case 'cargo': return 'Cargo';
			case 'winget': return 'winget';
			case 'apt': return 'apt';
			case 'flatpak': return 'Flatpak';
			case 'snap': return 'Snap';
			default: return id;
		}
	}

	function managerIcon(id: string): IconName {
		const map: Record<string, IconName> = {
			brew: 'brew', npm: 'npm', winget: 'winget', mas: 'mas',
			pip: 'pip', cargo: 'cargo', apt: 'apt', flatpak: 'flatpak', snap: 'snap'
		};
		return map[id] ?? 'installed';
	}

	function managerColor(id: string): string {
		switch (id) {
			case 'brew': return '#8fbcbb';
			case 'npm': return '#bf616a';
			case 'mas': return '#5e81ac';
			case 'pip': return '#ebcb8b';
			case 'cargo': return '#d08770';
			case 'winget': return '#81a1c1';
			case 'apt': return '#b48ead';
			case 'flatpak': return '#a3be8c';
			case 'snap': return '#88c0d0';
			default: return '#4c566a';
		}
	}

	function getManagerPackageCount(managerId: string): number {
		return getPackages().get(managerId as PackageManager)?.length ?? 0;
	}

	function getManagerOutdatedCount(managerId: string): number {
		return getOutdatedPackages().get(managerId as PackageManager)?.length ?? 0;
	}

	function getTotalPackageCount(): number {
		let total = 0;
		for (const pkgs of getPackages().values()) total += pkgs.length;
		return total;
	}

	// Donut chart data
	const donutSegments = $derived((() => {
		const mgrs = getAvailableManagers();
		const total = getTotalPackageCount();
		if (total === 0) return [];

		let cumulative = 0;
		return mgrs
			.map((m) => {
				const count = getManagerPackageCount(m.id);
				const pct = count / total;
				const start = cumulative;
				cumulative += pct;
				return { id: m.id, count, pct, start, end: cumulative, color: managerColor(m.id) };
			})
			.filter((s) => s.count > 0);
	})());

	// Convert percentage to SVG arc path for donut chart
	function donutArc(start: number, end: number, r: number, cx: number, cy: number): string {
		if (end - start >= 0.9999) {
			// Full circle — use two arcs
			return `M ${cx} ${cy - r} A ${r} ${r} 0 1 1 ${cx - 0.001} ${cy - r} A ${r} ${r} 0 1 1 ${cx} ${cy - r}`;
		}
		const startAngle = start * 2 * Math.PI - Math.PI / 2;
		const endAngle = end * 2 * Math.PI - Math.PI / 2;
		const x1 = cx + r * Math.cos(startAngle);
		const y1 = cy + r * Math.sin(startAngle);
		const x2 = cx + r * Math.cos(endAngle);
		const y2 = cy + r * Math.sin(endAngle);
		const large = end - start > 0.5 ? 1 : 0;
		return `M ${x1} ${y1} A ${r} ${r} 0 ${large} 1 ${x2} ${y2}`;
	}

	// Bar chart: top managers by count
	const barData = $derived((() => {
		return getAvailableManagers()
			.map((m) => ({
				id: m.id,
				name: managerDisplayName(m.id),
				installed: getManagerPackageCount(m.id),
				outdated: getManagerOutdatedCount(m.id),
				color: managerColor(m.id)
			}))
			.filter((d) => d.installed > 0)
			.sort((a, b) => b.installed - a.installed);
	})());

	const maxBarValue = $derived(Math.max(...barData.map((d) => d.installed), 1));

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
</script>

<div class="flex h-full flex-col overflow-y-auto">
	<!-- Header -->
	<div class="flex items-center justify-between border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<div class="flex flex-col">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Dashboard</h1>
			{#if getLastRefreshed()}
				<span class="text-[11px]" style="color: var(--text-muted);">
					Last updated {formatDate(getLastRefreshed())}
				</span>
			{/if}
		</div>
	</div>

	<!-- Stats Row -->
	<div class="flex gap-3 px-6 pt-5">
		<div
			class="flex flex-1 flex-col gap-1 rounded-lg border px-4 py-3"
			style="border-color: var(--border-subtle); background-color: var(--surface);"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Total</span>
			<span class="text-2xl font-bold" style="color: var(--text-primary);">{getTotalPackageCount()}</span>
			<span class="text-[10px]" style="color: var(--text-muted);">{getAvailableManagers().length} managers</span>
		</div>
		<button
			class="flex flex-1 flex-col gap-1 rounded-lg border px-4 py-3 text-left transition-colors duration-100 hover:border-[var(--warning)]"
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

	<!-- Charts Row -->
	<div class="flex gap-4 px-6 pt-4">
		<!-- Donut Chart -->
		<div
			class="flex flex-col items-center gap-3 rounded-lg border px-5 py-4"
			style="border-color: var(--border-subtle); background-color: var(--surface); min-width: 200px;"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Distribution</span>
			<svg width="120" height="120" viewBox="0 0 120 120">
				{#each donutSegments as seg}
					<path
						d={donutArc(seg.start, seg.end, 45, 60, 60)}
						fill="none"
						stroke={seg.color}
						stroke-width="14"
						stroke-linecap="butt"
					/>
				{/each}
				<!-- Center text -->
				<text x="60" y="56" text-anchor="middle" font-size="20" font-weight="700" fill="var(--text-primary)">
					{getTotalPackageCount()}
				</text>
				<text x="60" y="72" text-anchor="middle" font-size="9" fill="var(--text-muted)">
					packages
				</text>
			</svg>
			<!-- Legend -->
			<div class="flex flex-wrap justify-center gap-x-3 gap-y-1">
				{#each donutSegments as seg}
					<div class="flex items-center gap-1">
						<div class="h-2 w-2 rounded-full" style={`background-color: ${seg.color};`}></div>
						<span class="text-[9px]" style="color: var(--text-muted);">{managerDisplayName(seg.id)} ({seg.count})</span>
					</div>
				{/each}
			</div>
		</div>

		<!-- Bar Chart -->
		<div
			class="flex flex-1 flex-col gap-3 rounded-lg border px-5 py-4"
			style="border-color: var(--border-subtle); background-color: var(--surface);"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				Packages by Manager
			</span>
			<div class="flex flex-1 flex-col justify-center gap-2">
				{#each barData as bar}
					<div class="flex items-center gap-2">
						<span class="w-16 text-right text-[10px] font-medium" style="color: var(--text-secondary);">
							{bar.name}
						</span>
						<div class="flex flex-1 items-center gap-1">
							<!-- Installed bar -->
							<div class="h-4 rounded-sm transition-all duration-500" style={`width: ${(bar.installed / maxBarValue) * 100}%; background-color: ${bar.color}; min-width: 2px;`}></div>
							<!-- Outdated overlay -->
							{#if bar.outdated > 0}
								<div class="h-4 rounded-sm" style={`width: ${(bar.outdated / maxBarValue) * 100}%; background-color: var(--warning); min-width: 2px;`}></div>
							{/if}
						</div>
						<span class="w-10 text-right font-mono text-[10px]" style="color: var(--text-muted);">
							{bar.installed}
						</span>
					</div>
				{/each}
			</div>
			<div class="flex items-center gap-4">
				<div class="flex items-center gap-1">
					<div class="h-2 w-4 rounded-sm" style="background-color: var(--accent);"></div>
					<span class="text-[9px]" style="color: var(--text-muted);">Installed</span>
				</div>
				<div class="flex items-center gap-1">
					<div class="h-2 w-4 rounded-sm" style="background-color: var(--warning);"></div>
					<span class="text-[9px]" style="color: var(--text-muted);">Outdated</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Search + Package List -->
	<div class="flex flex-col gap-0 border-t px-6 pt-4 pb-0" style="border-color: var(--border-subtle);">
		<SearchBar bind:value={localSearch} oninput={handleSearch} />
	</div>
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
