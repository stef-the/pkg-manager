<script lang="ts">
	import PreferencesModal from '$lib/components/PreferencesModal.svelte';
	import ManagerMenu from '$lib/components/ManagerMenu.svelte';
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import type { ViewId, PackageManager, ManagerInfo } from '$lib/types';
	import {
		getActiveView,
		setActiveView,
		getActiveManagerFilter,
		setActiveManagerFilter,
		getTotalOutdatedCount,
		getAvailableManagers,
		refreshPackages,
		isLoading
	} from '$lib/stores/packages.svelte';
	const navItems: { id: ViewId; label: string; icon: IconName }[] = [
		{ id: 'dashboard', label: 'Dashboard', icon: 'dashboard' },
		{ id: 'installed', label: 'Installed', icon: 'installed' },
		{ id: 'outdated', label: 'Outdated', icon: 'outdated' },
		{ id: 'browse', label: 'Browse', icon: 'browse' },
		{ id: 'search', label: 'Search', icon: 'search' },
		{ id: 'apps', label: 'Apps', icon: 'installed' },
		{ id: 'managers', label: 'Managers', icon: 'managers' }
	];

	const toolItems: { id: ViewId; label: string; icon: IconName }[] = [
		{ id: 'cleanup', label: 'Cleanup & Doctor', icon: 'cleanup' },
		{ id: 'debloat', label: 'Debloat', icon: 'debloat' },
		{ id: 'export-import', label: 'Export / Import', icon: 'export' },
		{ id: 'logs', label: 'View Logs', icon: 'logs' },
		{ id: 'terminal', label: 'Terminal', icon: 'terminal' },
		{ id: 'history', label: 'History', icon: 'logs' }
	];

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew': return 'Homebrew';
			case 'npm': return 'npm';
			case 'winget': return 'winget';
			default: return id;
		}
	}

	function managerIconName(id: string): IconName {
		const map: Record<string, IconName> = {
			brew: 'brew', npm: 'npm', winget: 'winget', mas: 'mas',
			pip: 'pip', cargo: 'cargo', apt: 'apt', flatpak: 'flatpak',
			snap: 'snap', nix: 'nix', scoop: 'scoop', choco: 'choco', dnf: 'dnf', pacman: 'pacman', conda: 'conda'
		};
		return map[id] ?? 'installed';
	}

	let menuManager = $state<ManagerInfo | null>(null);
	let menuAnchor = $state({ x: 0, y: 0 });

	function openManagerMenu(manager: ManagerInfo, event: MouseEvent) {
		const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
		menuAnchor = { x: rect.right + 4, y: rect.top };
		menuManager = manager;
	}

	let prefsOpen = $state(false);
</script>

<aside
	class="flex h-full w-[240px] min-w-[240px] flex-col border-r"
	style="background-color: var(--bg-secondary); border-color: var(--border-subtle);"
>
	<!-- App Header -->
	<div class="flex items-center gap-2.5 px-5 py-4">
		<svg width="24" height="24" viewBox="0 0 512 512" fill="none">
			<rect width="512" height="512" rx="96" fill="var(--accent)"/>
			<path d="M256 136l100 52v126l-100 52-100-52V188z" fill="none" stroke="var(--bg-primary)" stroke-width="20"/>
			<path d="M156 188l100 52 100-52" fill="none" stroke="var(--bg-primary)" stroke-width="20" stroke-linejoin="round"/>
			<path d="M256 240v126" stroke="var(--bg-primary)" stroke-width="20"/>
		</svg>
		<span class="text-sm font-semibold" style="color: var(--text-primary);">Pkg Manager</span>
	</div>

	<!-- Scrollable middle -->
	<div class="flex-1 overflow-y-auto">
	<nav class="flex flex-col gap-0.5 px-3">
		<span
			class="mb-1 px-2 text-[10px] font-medium uppercase tracking-wider"
			style="color: var(--text-muted);"
		>
			Views
		</span>
		{#each navItems as item}
			{@const isActive = getActiveView() === item.id}
			{@const outdatedCount = item.id === 'outdated' ? getTotalOutdatedCount() : 0}
			<button
				class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style={isActive
					? `background-color: var(--accent); color: var(--bg-primary);`
					: `color: var(--text-secondary);`}
				onclick={() => setActiveView(item.id)}
			>
				<span class="flex w-4 items-center justify-center"><Icon name={item.icon} size={14} /></span>
				<span class="flex-1">{item.label}</span>
				{#if outdatedCount > 0}
					<span
						class="flex h-5 min-w-5 items-center justify-center rounded-full px-1.5 text-[11px] font-semibold"
						style={isActive
							? `background-color: var(--bg-primary); color: var(--accent);`
							: `background-color: var(--error); color: var(--bg-primary);`}
					>
						{outdatedCount}
					</span>
				{/if}
			</button>
		{/each}
	</nav>

	<!-- Package Managers Filter -->
	<div class="mt-5 flex flex-col gap-0.5 px-3">
		<span
			class="mb-1 px-2 text-[10px] font-medium uppercase tracking-wider"
			style="color: var(--text-muted);"
		>
			Managers
		</span>
		{#if true}
		{@const activeFilter = getActiveManagerFilter()}
		<button
			class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
			style={activeFilter === 'all'
				? `background-color: var(--accent); color: var(--bg-primary);`
				: `color: var(--text-secondary);`}
			onclick={() => setActiveManagerFilter('all')}
		>
			<span class="flex w-4 items-center justify-center"><Icon name="all" size={14} /></span>
			<span>All</span>
		</button>
		{#each getAvailableManagers() as manager}
			{@const isActive = activeFilter === manager.id}
			<button
				class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style={isActive
					? `background-color: var(--accent); color: var(--bg-primary);`
					: `color: var(--text-secondary);`}
				onclick={(e) => openManagerMenu(manager, e)}
				oncontextmenu={(e) => {
					e.preventDefault();
					openManagerMenu(manager, e);
				}}
			>
				<span class="flex w-4 items-center justify-center"><Icon name={managerIconName(manager.id)} size={14} /></span>
				<span class="flex-1">{managerDisplayName(manager.id)}</span>
				<svg width="8" height="8" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" style="opacity: 0.4;"><path d="M6 3l5 5-5 5" /></svg>
			</button>
		{/each}
		{/if}
	</div>

	<!-- Tools -->
	<div class="mt-5 flex flex-col gap-0.5 px-3">
		<span
			class="mb-1 px-2 text-[10px] font-medium uppercase tracking-wider"
			style="color: var(--text-muted);"
		>
			Tools
		</span>
		{#each toolItems as item}
			{@const isActive = getActiveView() === item.id}
			<button
				class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style={isActive
					? `background-color: var(--accent); color: var(--bg-primary);`
					: `color: var(--text-secondary);`}
				onclick={() => setActiveView(item.id)}
			>
				<span class="flex w-4 items-center justify-center"><Icon name={item.icon} size={14} /></span>
				<span>{item.label}</span>
			</button>
		{/each}
	</div>

	</div><!-- end scrollable middle -->

	<!-- Bottom Section: Refresh + Preferences (pinned) -->
	<div class="flex flex-col gap-2 border-t px-3 py-3" style="border-color: var(--border-subtle);">
		<button
			class="flex items-center gap-2 rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-secondary);"
			onclick={() => refreshPackages()}
			disabled={isLoading()}
		>
			<span class="flex w-4 items-center justify-center" class:animate-spin={isLoading()}><Icon name="refresh" size={14} /></span>
			<span>{isLoading() ? 'Refreshing...' : 'Refresh'}</span>
		</button>

		<!-- Preferences -->
		<button
			class="flex items-center gap-2 rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-secondary);"
			onclick={() => (prefsOpen = true)}
		>
			<span class="flex w-4 items-center justify-center"><Icon name="settings" size={14} /></span>
			<span>Preferences</span>
		</button>
	</div>
</aside>

<PreferencesModal bind:open={prefsOpen} />

{#if menuManager}
	<ManagerMenu
		manager={menuManager}
		anchor={menuAnchor}
		onclose={() => (menuManager = null)}
	/>
{/if}
