<script lang="ts">
	let {
		title = 'No packages found',
		message = ''
	}: {
		title?: string;
		message?: string;
	} = $props();
</script>

<div class="flex flex-col items-center justify-center gap-3 py-16">
	<svg width="32" height="32" viewBox="0 0 16 16" fill="none" stroke="var(--text-muted)" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.4">
		<path d="M2 5l6-3 6 3v6l-6 3-6-3V5z" />
		<path d="M2 5l6 3 6-3" />
		<path d="M8 8v6" />
	</svg>
	<h3 class="text-sm font-semibold" style="color: var(--text-primary);">{title}</h3>
	{#if message}
		<p class="max-w-xs text-center text-xs" style="color: var(--text-muted);">{message}</p>
	{/if}
</div>
