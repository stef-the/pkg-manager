<script lang="ts">
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import {
		getAllPackages,
		setActiveView,
		setSelectedPackage,
		refreshPackages,
		setSearchQuery
	} from '$lib/stores/packages.svelte';
	import type { ViewId, Package } from '$lib/types';

	let { open = $bindable(false) }: { open: boolean } = $props();
	let query = $state('');
	let selectedIndex = $state(0);
	let inputEl: HTMLInputElement;

	interface PaletteItem {
		id: string;
		label: string;
		sublabel?: string;
		icon: IconName;
		action: () => void;
		category: 'action' | 'view' | 'package';
	}

	const viewItems: PaletteItem[] = [
		{ id: 'v-dashboard', label: 'Dashboard', icon: 'dashboard', category: 'view', action: () => go('dashboard') },
		{ id: 'v-installed', label: 'Installed', icon: 'installed', category: 'view', action: () => go('installed') },
		{ id: 'v-outdated', label: 'Outdated', icon: 'outdated', category: 'view', action: () => go('outdated') },
		{ id: 'v-browse', label: 'Browse', icon: 'browse', category: 'view', action: () => go('browse') },
		{ id: 'v-search', label: 'Search', icon: 'search', category: 'view', action: () => go('search') },
		{ id: 'v-managers', label: 'Package Managers', icon: 'managers', category: 'view', action: () => go('managers') },
		{ id: 'v-cleanup', label: 'Cleanup & Doctor', icon: 'cleanup', category: 'view', action: () => go('cleanup') },
		{ id: 'v-debloat', label: 'Debloat', icon: 'debloat', category: 'view', action: () => go('debloat') },
		{ id: 'v-terminal', label: 'Terminal', icon: 'terminal', category: 'view', action: () => go('terminal') },
		{ id: 'v-history', label: 'History', icon: 'logs', category: 'view', action: () => go('history') },
		{ id: 'v-logs', label: 'View Logs', icon: 'logs', category: 'view', action: () => go('logs') },
		{ id: 'v-export', label: 'Export / Import', icon: 'export', category: 'view', action: () => go('export-import') },
	];

	const actionItems: PaletteItem[] = [
		{ id: 'a-refresh', label: 'Refresh packages', icon: 'refresh', category: 'action', action: () => { close(); refreshPackages(); } },
		{ id: 'a-prefs', label: 'Open preferences', icon: 'settings', category: 'action', action: () => { close(); window.dispatchEvent(new CustomEvent('open-preferences')); } },
	];

	function go(view: ViewId) {
		close();
		setActiveView(view);
	}

	function close() {
		open = false;
		query = '';
		selectedIndex = 0;
	}

	const results = $derived((() => {
		const q = query.toLowerCase().trim();
		let items: PaletteItem[] = [];

		// Always show actions and views
		items.push(...actionItems);
		items.push(...viewItems);

		// Search packages if query is long enough
		if (q.length >= 2) {
			const pkgs = getAllPackages()
				.filter((p) => p.name.toLowerCase().includes(q))
				.slice(0, 10)
				.map((p): PaletteItem => ({
					id: `p-${p.manager}/${p.name}`,
					label: p.name,
					sublabel: `${p.manager} ${p.version}`,
					icon: p.manager === 'brew' ? 'brew' : p.manager === 'npm' ? 'npm' : 'installed',
					category: 'package',
					action: () => { close(); setSelectedPackage(p); setActiveView('installed'); }
				}));
			items.push(...pkgs);
		}

		if (!q) return items;
		return items.filter((i) => i.label.toLowerCase().includes(q));
	})());

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			close();
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (results[selectedIndex]) {
				results[selectedIndex].action();
			}
		}
	}

	$effect(() => {
		if (open) {
			selectedIndex = 0;
			requestAnimationFrame(() => inputEl?.focus());
		}
	});

	// Reset selection when results change
	$effect(() => {
		if (results.length > 0 && selectedIndex >= results.length) {
			selectedIndex = 0;
		}
	});

	function categoryLabel(cat: string): string {
		switch (cat) {
			case 'action': return 'Actions';
			case 'view': return 'Views';
			case 'package': return 'Packages';
			default: return '';
		}
	}
</script>

{#if open}
	<button
		class="fixed inset-0 z-50"
		style="background-color: rgba(0, 0, 0, 0.5);"
		onclick={close}
		aria-label="Close"
	></button>

	<div
		class="fixed left-1/2 top-[15%] z-50 flex w-[500px] -translate-x-1/2 flex-col overflow-hidden rounded-xl shadow-2xl"
		style="background-color: var(--surface); border: 1px solid var(--border-subtle); max-height: 60vh;"
	>
		<!-- Input -->
		<div class="flex items-center gap-3 border-b px-4 py-3" style="border-color: var(--border-subtle);">
			<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="var(--text-muted)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
				<circle cx="7" cy="7" r="4" /><path d="M10 10l3.5 3.5" />
			</svg>
			<input
				bind:this={inputEl}
				bind:value={query}
				onkeydown={handleKeydown}
				placeholder="Search packages, views, actions..."
				class="flex-1 bg-transparent text-[14px] outline-none"
				style="color: var(--text-primary);"
			/>
			<kbd class="rounded border px-1.5 py-0.5 text-[10px]" style="border-color: var(--border); color: var(--text-muted);">esc</kbd>
		</div>

		<!-- Results -->
		<div class="overflow-y-auto">
			{#each ['action', 'view', 'package'] as category}
				{@const catItems = results.filter((r) => r.category === category)}
				{#if catItems.length > 0}
					<div class="px-3 pt-2 pb-1">
						<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
							{categoryLabel(category)}
						</span>
					</div>
					{#each catItems as item, i}
						{@const globalIdx = results.indexOf(item)}
						<button
							class="flex w-full items-center gap-3 px-4 py-2 text-left text-[13px] transition-colors duration-75"
							style={globalIdx === selectedIndex
								? 'background-color: var(--accent); color: var(--bg-primary);'
								: 'color: var(--text-primary);'}
							onmouseenter={() => (selectedIndex = globalIdx)}
							onclick={() => item.action()}
						>
							<span style={globalIdx === selectedIndex ? '' : 'opacity: 0.5;'}>
								<Icon name={item.icon} size={14} />
							</span>
							<span class="flex-1">{item.label}</span>
							{#if item.sublabel}
								<span class="text-[11px]" style={globalIdx === selectedIndex ? 'opacity: 0.7;' : 'color: var(--text-muted);'}>
									{item.sublabel}
								</span>
							{/if}
						</button>
					{/each}
				{/if}
			{/each}
			{#if results.length === 0}
				<div class="px-4 py-6 text-center text-[13px]" style="color: var(--text-muted);">
					No results for "{query}"
				</div>
			{/if}
		</div>
	</div>
{/if}
