<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import PackageList from '$lib/components/PackageList.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import {
		getFilteredPackages,
		isLoading,
		getSearchQuery,
		setSearchQuery
	} from '$lib/stores/packages.svelte';

	let localSearch = $state(getSearchQuery());

	function handleSearch(value: string) {
		setSearchQuery(value);
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex flex-col gap-4 border-b px-6 py-5" style="border-color: var(--border-subtle);">
		<div class="flex items-center justify-between">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Installed Packages</h1>
			<span class="text-sm" style="color: var(--text-muted);">
				{getFilteredPackages().length} packages
			</span>
		</div>
		<SearchBar bind:value={localSearch} oninput={handleSearch} />
	</div>

	<!-- Package List -->
	<div class="flex-1 overflow-y-auto">
		{#if isLoading()}
			<LoadingSkeleton />
		{:else}
			<PackageList
				packages={getFilteredPackages()}
				emptyTitle={getSearchQuery() ? 'No matching packages' : 'No installed packages'}
				emptyMessage={getSearchQuery() ? 'Try a different search term' : ''}
			/>
		{/if}
	</div>
</div>
