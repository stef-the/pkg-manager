<script lang="ts">
	import { page } from '$app/state';
</script>

<div
	class="flex h-screen w-full flex-col items-center justify-center gap-4 p-8"
	style="background-color: var(--bg-primary); color: var(--text-primary);"
>
	<span class="font-mono text-[48px] font-bold" style="color: var(--error);">
		{page.status}
	</span>
	<h1 class="text-lg font-semibold">
		{#if page.status === 404}
			Page not found
		{:else}
			Something went wrong
		{/if}
	</h1>
	<p class="max-w-md text-center text-sm" style="color: var(--text-secondary);">
		{page.error?.message ?? 'An unexpected error occurred.'}
	</p>
	<a
		href="/"
		class="rounded-lg px-6 py-2 text-sm font-medium transition-colors duration-100 hover:opacity-90"
		style="background-color: var(--accent); color: var(--bg-primary);"
	>
		Back to Dashboard
	</a>
</div>
