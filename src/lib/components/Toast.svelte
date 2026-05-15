<script lang="ts">
	import { getToasts, removeToast } from '$lib/stores/toast.svelte';

	function typeStyles(type: string): string {
		switch (type) {
			case 'success':
				return 'background-color: var(--success); color: var(--color-nord0);';
			case 'error':
				return 'background-color: var(--error); color: var(--color-nord6);';
			case 'warning':
				return 'background-color: var(--warning); color: var(--color-nord0);';
			case 'info':
			default:
				return 'background-color: var(--info); color: var(--color-nord0);';
		}
	}
</script>

{#if getToasts().length > 0}
	<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
		{#each getToasts() as toast (toast.id)}
			<div
				class="flex min-w-[280px] max-w-[400px] items-center gap-3 rounded-lg px-4 py-3 shadow-lg"
				style={typeStyles(toast.type)}
			>
				<span class="flex-1 text-sm font-medium">{toast.message}</span>
				<button
					class="flex-shrink-0 opacity-70 transition-opacity hover:opacity-100"
					aria-label="Dismiss"
					onclick={() => removeToast(toast.id)}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
				</button>
			</div>
		{/each}
	</div>
{/if}
