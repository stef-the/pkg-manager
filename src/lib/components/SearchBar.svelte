<script lang="ts">
	let { value = $bindable(''), placeholder = 'Search packages...', oninput }: {
		value: string;
		placeholder?: string;
		oninput?: (value: string) => void;
	} = $props();

	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	function handleInput(e: Event) {
		const target = e.target as HTMLInputElement;
		value = target.value;

		if (debounceTimer) clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			oninput?.(value);
		}, 300);
	}

	function clear() {
		value = '';
		oninput?.('');
	}
</script>

<div class="relative">
	<svg
		class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2"
		width="14" height="14" viewBox="0 0 16 16" fill="none"
		stroke="var(--text-muted)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
	>
		<circle cx="7" cy="7" r="4" />
		<path d="M10 10l3.5 3.5" />
	</svg>
	<input
		type="text"
		{placeholder}
		{value}
		oninput={handleInput}
		class="w-full rounded-lg border py-2 pl-9 pr-8 text-sm outline-none transition-colors duration-100"
		style={`
			background-color: var(--surface);
			border-color: var(--border-subtle);
			color: var(--text-primary);
		`}
		onfocus={(e) => (e.currentTarget.style.borderColor = 'var(--accent)')}
		onblur={(e) => (e.currentTarget.style.borderColor = 'var(--border-subtle)')}
	/>
	{#if value}
		<button
			class="absolute right-2.5 top-1/2 -translate-y-1/2 transition-colors duration-100"
			style="color: var(--text-muted);"
			aria-label="Clear search"
			onclick={clear}
		>
			<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
		</button>
	{/if}
</div>
