<script lang="ts">
	import { createLogger } from '$lib/utils/logger';

	const log = createLogger('ErrorBoundary');

	let { children } = $props();
	let error = $state<Error | null>(null);

	function handleError(e: ErrorEvent) {
		log.error(`Uncaught error: ${e.message}`);
		error = e.error instanceof Error ? e.error : new Error(e.message);
	}

	function handleRejection(e: PromiseRejectionEvent) {
		log.error(`Unhandled promise rejection: ${e.reason}`);
	}

	function reload() {
		error = null;
		window.location.reload();
	}

	$effect(() => {
		window.addEventListener('error', handleError);
		window.addEventListener('unhandledrejection', handleRejection);
		return () => {
			window.removeEventListener('error', handleError);
			window.removeEventListener('unhandledrejection', handleRejection);
		};
	});
</script>

{#if error}
	<div
		class="flex h-screen w-full flex-col items-center justify-center gap-4 p-8"
		style="background-color: var(--bg-primary); color: var(--text-primary);"
	>
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full text-xl"
			style="background-color: var(--error); color: var(--bg-primary);"
		>
			!
		</div>
		<h1 class="text-lg font-semibold">Something went wrong</h1>
		<p class="max-w-md text-center text-sm" style="color: var(--text-secondary);">
			{error.message}
		</p>
		<button
			class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100"
			style="background-color: var(--accent); color: var(--bg-primary);"
			onclick={reload}
		>
			Reload
		</button>
	</div>
{:else}
	{@render children()}
{/if}
