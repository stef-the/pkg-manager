<script lang="ts">
	let { rows = 8 }: { rows?: number } = $props();
</script>

<div class="flex flex-col">
	{#each Array(rows) as _, i}
		<div
			class="flex items-center gap-4 border-b px-4 py-2.5"
			style="border-color: var(--border-subtle); animation: skeleton-pulse 1.5s ease-in-out infinite; animation-delay: {i * 75}ms;"
		>
			<!-- Icon placeholder -->
			<div class="h-3.5 w-4 rounded" style="background-color: var(--bg-tertiary);"></div>
			<!-- Name -->
			<div class="h-3.5 w-36 rounded" style="background-color: var(--bg-tertiary);"></div>
			<!-- Version -->
			<div class="h-3 w-16 rounded" style="background-color: var(--bg-tertiary);"></div>
			<!-- Description -->
			<div class="h-3 flex-1 rounded" style="background-color: var(--bg-tertiary);"></div>
			<!-- Badge -->
			<div class="h-4 w-10 rounded-full" style="background-color: var(--bg-tertiary);"></div>
		</div>
	{/each}
</div>

<style>
	@keyframes skeleton-pulse {
		0%, 100% { opacity: 0.3; }
		50% { opacity: 0.6; }
	}
</style>
