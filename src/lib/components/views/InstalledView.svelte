<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import PackageList from '$lib/components/PackageList.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import {
		getFilteredPackages,
		isLoading,
		getError,
		getAvailableManagers,
		getSearchQuery,
		setSearchQuery,
		setActiveView,
		refreshPackages
	} from '$lib/stores/packages.svelte';

	let localSearch = $state(getSearchQuery());

	function handleSearch(value: string) {
		setSearchQuery(value);
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex flex-col gap-4 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<div class="flex items-center justify-between">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Installed Packages</h1>
			<span class="text-[12px]" style="color: var(--text-muted);">
				{getFilteredPackages().length} packages
			</span>
		</div>
		<SearchBar bind:value={localSearch} oninput={handleSearch} />
	</div>

	<!-- Package List -->
	<div class="flex-1 overflow-y-auto">
		{#if isLoading() && getFilteredPackages().length === 0}
			<LoadingSkeleton />
		{:else if getError() && getFilteredPackages().length === 0}
			<EmptyState
				variant="error"
				title="Failed to load packages"
				message={getError() ?? 'An unknown error occurred.'}
				actionLabel="Retry"
				onaction={refreshPackages}
			/>
		{:else}
			<PackageList
				packages={getFilteredPackages()}
				emptyTitle={getSearchQuery() ? 'No matching packages' : 'No installed packages'}
				emptyMessage={getSearchQuery() ? 'Try a different search term' : 'Your installed packages will appear here once loaded.'}
			/>
		{/if}
	</div>
</div>
