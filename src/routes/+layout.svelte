<script lang="ts">
	import '../app.css';
	import appIcon from '$lib/assets/app-icon.svg';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import DetailPanel from '$lib/components/DetailPanel.svelte';
	import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
	import Toast from '$lib/components/Toast.svelte';
	import KeyboardHandler from '$lib/components/KeyboardHandler.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import AppUpdater from '$lib/components/AppUpdater.svelte';
	import { startAutoCheck } from '$lib/stores/autoupdate.svelte';

	let paletteOpen = $state(false);

	$effect(() => {
		function handleKey(e: KeyboardEvent) {
			if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
				e.preventDefault();
				paletteOpen = !paletteOpen;
			}
		}
		window.addEventListener('keydown', handleKey);
		return () => window.removeEventListener('keydown', handleKey);
	});
	import { initTheme, getEffectiveTheme, applyThemeToDocument } from '$lib/stores/theme.svelte';
	import { setActiveView, refreshPackages } from '$lib/stores/packages.svelte';

	let { children } = $props();

	$effect(() => {
		initTheme();

		// Start auto-update checker
		startAutoCheck();

		// Listen for tray menu actions
		if (typeof window !== 'undefined' && '__TAURI__' in window) {
			import('@tauri-apps/api/event').then(({ listen }) => {
				listen<string>('tray-action', (event) => {
					if (event.payload === 'refresh') refreshPackages();
					if (event.payload === 'outdated') setActiveView('outdated');
				});
			});
		}
	});

	$effect(() => {
		const _theme = getEffectiveTheme();
		applyThemeToDocument();
	});
</script>

<svelte:head>
	<link rel="icon" href={appIcon} />
	<title>Pkg Manager</title>
</svelte:head>

<ErrorBoundary>
	<div class="flex h-screen overflow-hidden" style="background-color: var(--bg-primary);">
		<div class="flex flex-col">
			<Sidebar />
			<AppUpdater />
		</div>
		<main class="flex min-w-0 flex-1 flex-col overflow-hidden">
			{@render children()}
		</main>
		<DetailPanel />
	</div>
	<Toast />
	<KeyboardHandler />
	<CommandPalette bind:open={paletteOpen} />
</ErrorBoundary>
