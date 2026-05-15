<script lang="ts">
	let {
		title = 'No packages found',
		message = '',
		actionLabel = '',
		onaction = undefined as (() => void) | undefined,
		variant = 'default' as 'default' | 'error' | 'warning'
	}: {
		title?: string;
		message?: string;
		actionLabel?: string;
		onaction?: (() => void) | undefined;
		variant?: 'default' | 'error' | 'warning';
	} = $props();

	function iconColor(): string {
		switch (variant) {
			case 'error': return 'var(--error)';
			case 'warning': return 'var(--warning)';
			default: return 'var(--text-muted)';
		}
	}
</script>

<div class="flex flex-col items-center justify-center gap-3 py-16">
	{#if variant === 'error'}
		<svg width="32" height="32" viewBox="0 0 16 16" fill="none" stroke={iconColor()} stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.6">
			<circle cx="8" cy="8" r="6" />
			<path d="M8 5v3.5" />
			<circle cx="8" cy="11" r="0.5" fill={iconColor()} />
		</svg>
	{:else if variant === 'warning'}
		<svg width="32" height="32" viewBox="0 0 16 16" fill="none" stroke={iconColor()} stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.6">
			<path d="M8 2L1 14h14L8 2z" />
			<path d="M8 6.5v3" />
			<circle cx="8" cy="11.5" r="0.5" fill={iconColor()} />
		</svg>
	{:else}
		<svg width="32" height="32" viewBox="0 0 16 16" fill="none" stroke={iconColor()} stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.4">
			<path d="M2 5l6-3 6 3v6l-6 3-6-3V5z" />
			<path d="M2 5l6 3 6-3" />
			<path d="M8 8v6" />
		</svg>
	{/if}
	<h3 class="text-[13px] font-semibold" style="color: var(--text-primary);">{title}</h3>
	{#if message}
		<p class="max-w-xs text-center text-[12px]" style="color: var(--text-muted);">{message}</p>
	{/if}
	{#if actionLabel && onaction}
		<button
			class="mt-1 rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
			style="background-color: var(--accent); color: var(--bg-primary);"
			onclick={onaction}
		>
			{actionLabel}
		</button>
	{/if}
</div>
