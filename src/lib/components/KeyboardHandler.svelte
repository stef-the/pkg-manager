<script lang="ts">
	import { setActiveView, setSearchQuery } from '$lib/stores/packages.svelte';
	import type { ViewId } from '$lib/types';

	let searchFocused = $state(false);

	function handleKeydown(e: KeyboardEvent) {
		const meta = e.metaKey || e.ctrlKey;

		// Cmd+K handled by CommandPalette in layout

		// Cmd/Ctrl+number — switch views
		if (meta && e.key >= '1' && e.key <= '9') {
			e.preventDefault();
			const views: ViewId[] = ['dashboard', 'installed', 'outdated', 'browse', 'search', 'managers'];
			const idx = parseInt(e.key) - 1;
			if (idx < views.length) {
				setActiveView(views[idx]);
			}
			return;
		}

		// Cmd/Ctrl+R — refresh
		if (meta && e.key === 'r') {
			e.preventDefault();
			import('$lib/stores/packages.svelte').then(({ refreshPackages }) => {
				refreshPackages();
			});
			return;
		}

		// Cmd/Ctrl+, — preferences
		if (meta && e.key === ',') {
			e.preventDefault();
			// Emit a custom event that the sidebar can listen to
			window.dispatchEvent(new CustomEvent('open-preferences'));
			return;
		}

		// Escape — clear search or close modals
		if (e.key === 'Escape') {
			setSearchQuery('');
			const focused = document.activeElement as HTMLElement;
			focused?.blur();
		}
	}

	$effect(() => {
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});
</script>
