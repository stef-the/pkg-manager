<script lang="ts">
	import DashboardView from '$lib/components/views/DashboardView.svelte';
	import InstalledView from '$lib/components/views/InstalledView.svelte';
	import OutdatedView from '$lib/components/views/OutdatedView.svelte';
	import SearchView from '$lib/components/views/SearchView.svelte';
	import BrowseView from '$lib/components/views/BrowseView.svelte';
	import ManagersView from '$lib/components/views/ManagersView.svelte';
	import CleanupView from '$lib/components/views/CleanupView.svelte';
	import DebloatView from '$lib/components/views/DebloatView.svelte';
	import ExportImportView from '$lib/components/views/ExportImportView.svelte';
	import LogsView from '$lib/components/views/LogsView.svelte';
	import TerminalView from '$lib/components/views/TerminalView.svelte';
	import HistoryView from '$lib/components/views/HistoryView.svelte';
	import {
		getActiveView, setActiveView, setActiveManagerFilter,
		loadAllPackages, getTotalOutdatedCount, getAllPackages
	} from '$lib/stores/packages.svelte';
	import { loadPinned } from '$lib/stores/pinned.svelte';
	import { loadState, saveState, updateWindowTitle, updateTrayTooltip } from '$lib/stores/state.svelte';
	import { createLogger } from '$lib/utils/logger';
	import type { ViewId, PackageManager } from '$lib/types';

	const log = createLogger('page');

	// Load saved state on mount
	$effect(() => {
		log.info('App mounted, loading...');
		loadAllPackages();
		loadPinned();
		loadState().then((state) => {
			setActiveView(state.activeView);
			setActiveManagerFilter(state.activeManagerFilter);
		});
	});

	// Persist view changes
	$effect(() => {
		const view = getActiveView();
		saveState({ activeView: view });
	});

	// Update window title and tray when outdated count changes
	$effect(() => {
		const outdated = getTotalOutdatedCount();
		const total = getAllPackages().length;
		updateWindowTitle(outdated);
		updateTrayTooltip(total, outdated);
	});
</script>

{#if getActiveView() === 'dashboard'}
	<DashboardView />
{:else if getActiveView() === 'installed'}
	<InstalledView />
{:else if getActiveView() === 'outdated'}
	<OutdatedView />
{:else if getActiveView() === 'search'}
	<SearchView />
{:else if getActiveView() === 'browse'}
	<BrowseView />
{:else if getActiveView() === 'managers'}
	<ManagersView />
{:else if getActiveView() === 'cleanup'}
	<CleanupView />
{:else if getActiveView() === 'debloat'}
	<DebloatView />
{:else if getActiveView() === 'export-import'}
	<ExportImportView />
{:else if getActiveView() === 'logs'}
	<LogsView />
{:else if getActiveView() === 'terminal'}
	<TerminalView />
{:else if getActiveView() === 'history'}
	<HistoryView />
{/if}
