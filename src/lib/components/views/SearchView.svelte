<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import {
		searchRemotePackages,
		getSearchResults,
		getSearchError,
		isLoadingSearch,
		installPkg,
		getAvailableManagers,
		getAllPackages,
		setActiveView
	} from '$lib/stores/packages.svelte';
	import type { PackageManager, Package } from '$lib/types';

	let query = $state('');
	let selectedManager = $state<PackageManager>('brew');
	let hasSearched = $state(false);

	function handleSearch(value: string) {
		query = value;
		if (value.trim()) {
			hasSearched = true;
			searchRemotePackages(selectedManager, value);
		}
	}

	function retrySearch() {
		if (query.trim()) {
			searchRemotePackages(selectedManager, query);
		}
	}

	function handleManagerChange(manager: PackageManager) {
		selectedManager = manager;
		if (query.trim()) {
			searchRemotePackages(manager, query);
		}
	}

	function isInstalled(pkg: Package): boolean {
		return getAllPackages().some((p) => p.name === pkg.name && p.manager === pkg.manager);
	}

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew':
				return 'Homebrew';
			case 'npm':
				return 'npm';
			default:
				return id;
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex flex-col gap-4 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Search Packages</h1>

		<!-- Manager Tabs -->
		<div class="flex items-center gap-1 rounded-lg p-1" style="background-color: var(--bg-secondary);">
			{#each getAvailableManagers() as manager}
				{@const isActive = selectedManager === manager.id}
				<button
					class="flex-1 rounded-md px-3 py-1.5 text-center text-[13px] font-medium transition-colors duration-100"
					style={isActive
						? 'background-color: var(--accent); color: var(--bg-primary);'
						: 'color: var(--text-secondary);'}
					onclick={() => handleManagerChange(manager.id as PackageManager)}
				>
					{managerDisplayName(manager.id)}
				</button>
			{/each}
		</div>

		<SearchBar
			bind:value={query}
			placeholder={`Search ${managerDisplayName(selectedManager)} packages...`}
			oninput={handleSearch}
		/>
	</div>

	<!-- Results -->
	<div class="flex-1 overflow-y-auto">
		{#if getAvailableManagers().length === 0}
			<EmptyState
				variant="warning"
				title="No package managers available"
				message="Install a package manager to search for packages."
				actionLabel="Go to Managers"
				onaction={() => setActiveView('managers')}
			/>
		{:else if isLoadingSearch()}
			<LoadingSkeleton />
		{:else if getSearchError()}
			<EmptyState
				variant="error"
				title="Search failed"
				message={getSearchError() ?? 'A network or system error occurred. Check your connection and try again.'}
				actionLabel="Retry"
				onaction={retrySearch}
			/>
		{:else if getSearchResults().length > 0}
			<div class="flex flex-col">
				{#each getSearchResults() as pkg (pkg.manager + '/' + pkg.name)}
					{@const installed = isInstalled(pkg)}
					<div
						class="flex items-center gap-4 border-b px-4 py-2.5"
						style="border-color: var(--border-subtle);"
					>
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2">
								<span class="text-[13px] font-semibold" style="color: var(--text-primary);">
									{pkg.name}
								</span>
								<span class="font-mono text-[11px]" style="color: var(--text-muted);">
									{pkg.version}
								</span>
							</div>
							{#if pkg.description}
								<p class="mt-0.5 truncate text-[12px]" style="color: var(--text-secondary);">
									{pkg.description}
								</p>
							{/if}
						</div>
						{#if installed}
							<span
								class="rounded-full px-3 py-1 text-[11px] font-semibold"
								style="background-color: var(--success); color: var(--color-nord0);"
							>
								Installed
							</span>
						{:else}
							<button
								class="rounded-full px-3 py-1 text-[11px] font-semibold transition-colors duration-100 hover:opacity-90"
								style="background-color: var(--accent); color: var(--bg-primary);"
								onclick={() => installPkg(pkg.manager as PackageManager, pkg.name)}
								>
								Install
							</button>
						{/if}
					</div>
				{/each}
			</div>
		{:else if hasSearched && !query.trim()}
			<EmptyState title="Search for packages" message="Enter a search term above" />
		{:else if hasSearched}
			<EmptyState
				title="No results found"
				message={`No packages matching "${query}" in ${managerDisplayName(selectedManager)}`}
			/>
		{:else}
			<EmptyState
				title="Search for packages"
				message={`Find and install packages from ${managerDisplayName(selectedManager)}`}
			/>
		{/if}
	</div>
</div>
