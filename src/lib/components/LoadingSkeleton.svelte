<script lang="ts">
	let { rows = 8 }: { rows?: number } = $props();
</script>

<div class="flex flex-col gap-1">
	{#each Array(rows) as _, i}
		<div
			class="flex items-center gap-4 rounded-lg px-4 py-3"
			style="animation: pulse 1.5s ease-in-out infinite; animation-delay: {i * 75}ms;"
		>
			<div class="h-4 w-32 rounded" style="background-color: var(--bg-tertiary);"></div>
			<div class="h-4 w-16 rounded" style="background-color: var(--bg-tertiary);"></div>
			<div class="h-4 flex-1 rounded" style="background-color: var(--bg-tertiary);"></div>
			<div class="h-5 w-14 rounded-full" style="background-color: var(--bg-tertiary);"></div>
		</div>
	{/each}
</div>

<style>
	@keyframes pulse {
		0%,
		100% {
			opacity: 0.4;
		}
		50% {
			opacity: 0.7;
		}
	}
</style>
