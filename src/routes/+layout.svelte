<script lang="ts">
	import '../app.css';
	import appIcon from '$lib/assets/app-icon.svg';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import DetailPanel from '$lib/components/DetailPanel.svelte';
	import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
	import Toast from '$lib/components/Toast.svelte';
	import KeyboardHandler from '$lib/components/KeyboardHandler.svelte';
	import { initTheme, getEffectiveTheme, applyThemeToDocument } from '$lib/stores/theme.svelte';

	let { children } = $props();

	$effect(() => {
		initTheme();
	});

	$effect(() => {
		// Re-run whenever effective theme changes
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
		<Sidebar />
		<main class="flex min-w-0 flex-1 flex-col overflow-hidden">
			{@render children()}
		</main>
		<DetailPanel />
	</div>
	<Toast />
	<KeyboardHandler />
</ErrorBoundary>
