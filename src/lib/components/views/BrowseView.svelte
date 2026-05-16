<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import PackageIcon from '$lib/components/PackageIcon.svelte';
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
	let selectedManager = $state<PackageManager | 'all'>('all');
	let hasSearched = $state(false);

	interface FeaturedCategory {
		title: string;
		packages: { name: string; manager: PackageManager; description: string }[];
	}

	const featured: FeaturedCategory[] = [
		{
			title: 'Developer Essentials',
			packages: [
				{ name: 'git', manager: 'brew', description: 'Distributed version control system' },
				{ name: 'node', manager: 'brew', description: 'JavaScript runtime' },
				{ name: 'python3', manager: 'brew', description: 'Python programming language' },
				{ name: 'rust', manager: 'brew', description: 'Systems programming language' },
				{ name: 'go', manager: 'brew', description: 'Go programming language' },
				{ name: 'docker', manager: 'brew', description: 'Container platform' },
				{ name: 'gh', manager: 'brew', description: 'GitHub CLI' },
				{ name: 'neovim', manager: 'brew', description: 'Hyperextensible text editor' },
			]
		},
		{
			title: 'CLI Power Tools',
			packages: [
				{ name: 'ripgrep', manager: 'brew', description: 'Fast regex search tool' },
				{ name: 'fd', manager: 'brew', description: 'Fast file finder' },
				{ name: 'bat', manager: 'brew', description: 'Cat clone with syntax highlighting' },
				{ name: 'fzf', manager: 'brew', description: 'Fuzzy finder' },
				{ name: 'jq', manager: 'brew', description: 'JSON processor' },
				{ name: 'htop', manager: 'brew', description: 'Interactive process viewer' },
				{ name: 'tmux', manager: 'brew', description: 'Terminal multiplexer' },
				{ name: 'wget', manager: 'brew', description: 'Internet file retriever' },
			]
		},
		{
			title: 'Popular npm Packages',
			packages: [
				{ name: 'typescript', manager: 'npm', description: 'TypeScript compiler' },
				{ name: 'eslint', manager: 'npm', description: 'JavaScript linter' },
				{ name: 'prettier', manager: 'npm', description: 'Code formatter' },
				{ name: 'vite', manager: 'npm', description: 'Next-gen frontend build tool' },
				{ name: 'tailwindcss', manager: 'npm', description: 'Utility-first CSS framework' },
				{ name: 'prisma', manager: 'npm', description: 'Next-gen ORM for Node.js' },
				{ name: 'zod', manager: 'npm', description: 'TypeScript-first schema validation' },
				{ name: 'vitest', manager: 'npm', description: 'Fast unit test framework' },
			]
		},
		{
			title: 'Python Data Science',
			packages: [
				{ name: 'numpy', manager: 'pip', description: 'Numerical computing' },
				{ name: 'pandas', manager: 'pip', description: 'Data analysis library' },
				{ name: 'scipy', manager: 'pip', description: 'Scientific computing' },
				{ name: 'jupyter', manager: 'pip', description: 'Interactive notebooks' },
				{ name: 'flask', manager: 'pip', description: 'Lightweight web framework' },
				{ name: 'fastapi', manager: 'pip', description: 'Modern async web framework' },
				{ name: 'black', manager: 'pip', description: 'Python code formatter' },
				{ name: 'ruff', manager: 'pip', description: 'Fast Python linter' },
			]
		}
	];

	function handleSearch(value: string) {
		query = value;
		if (!value.trim()) return;
		hasSearched = true;
		if (selectedManager === 'all') {
			for (const m of getAvailableManagers()) {
				searchRemotePackages(m.id as PackageManager, value);
			}
		} else {
			searchRemotePackages(selectedManager, value);
		}
	}

	function retrySearch() {
		if (query.trim()) {
			handleSearch(query);
		}
	}

	function isInstalled(name: string, manager: string): boolean {
		return getAllPackages().some((p) => p.name === name && p.manager === manager);
	}

	function managedElsewhere(name: string, manager: string): string | null {
		const other = getAllPackages().find((p) => p.name === name && p.manager !== manager);
		return other ? other.manager : null;
	}

	function managerColor(id: string): string {
		switch (id) {
			case 'brew': return 'var(--color-nord7)';
			case 'npm': return 'var(--color-nord11)';
			case 'pip': return 'var(--color-nord13)';
			case 'cargo': return 'var(--color-nord12)';
			case 'winget': return 'var(--color-nord9)';
			case 'mas': return 'var(--color-nord10)';
			case 'apt': return 'var(--color-nord15)';
			default: return 'var(--text-muted)';
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex flex-col gap-4 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Browse Packages</h1>

		<!-- Manager filter tabs -->
		<div class="flex items-center gap-1 overflow-x-auto rounded-lg p-1" style="background-color: var(--bg-secondary);">
			<button
				class="flex-shrink-0 rounded-md px-3 py-1.5 text-center text-[12px] font-medium transition-colors duration-100"
				style={selectedManager === 'all'
					? 'background-color: var(--accent); color: var(--bg-primary);'
					: 'color: var(--text-secondary);'}
				onclick={() => (selectedManager = 'all')}
			>
				All
			</button>
			{#each getAvailableManagers() as manager}
				{@const isActive = selectedManager === manager.id}
				<button
					class="flex-shrink-0 rounded-md px-3 py-1.5 text-center text-[12px] font-medium transition-colors duration-100"
					style={isActive
						? 'background-color: var(--accent); color: var(--bg-primary);'
						: 'color: var(--text-secondary);'}
					onclick={() => (selectedManager = manager.id as PackageManager)}
				>
					{manager.name}
				</button>
			{/each}
		</div>

		<SearchBar
			bind:value={query}
			placeholder="Search packages across all managers..."
			oninput={handleSearch}
		/>
	</div>

	<div class="flex-1 overflow-y-auto">
		{#if getAvailableManagers().length === 0}
			<EmptyState
				variant="warning"
				title="No package managers available"
				message="Install a package manager to browse and install packages."
				actionLabel="Go to Managers"
				onaction={() => setActiveView('managers')}
			/>
		{:else if isLoadingSearch()}
			<LoadingSkeleton />
		{:else if getSearchError() && hasSearched}
			<EmptyState
				variant="error"
				title="Search failed"
				message={getSearchError() ?? 'A network or system error occurred. Check your connection and try again.'}
				actionLabel="Retry"
				onaction={retrySearch}
			/>
		{:else if hasSearched && getSearchResults().length > 0}
			<!-- Search results -->
			<div class="flex flex-col">
				{#each getSearchResults() as pkg (pkg.manager + '/' + pkg.name)}
					{@const installed = isInstalled(pkg.name, pkg.manager)}
					<div class="flex items-center gap-3 border-b px-4 py-2.5" style="border-color: var(--border-subtle);">
						<PackageIcon manager={pkg.manager as PackageManager} name={pkg.name} size={20} />
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2">
								<span class="text-[13px] font-semibold" style="color: var(--text-primary);">{pkg.name}</span>
								<span class="font-mono text-[10px]" style="color: var(--text-muted);">{pkg.version}</span>
								<span class="rounded-full px-1.5 py-0.5 text-[9px] font-semibold" style={`background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}>{pkg.manager}</span>
							</div>
							{#if pkg.description}
								<p class="mt-0.5 truncate text-[11px]" style="color: var(--text-muted);">{pkg.description}</p>
							{/if}
						</div>
						{#if installed}
							<span class="rounded-full px-3 py-1 text-[10px] font-semibold" style="background-color: var(--success); color: var(--color-nord0);">Installed</span>
						{:else}
							{@const otherMgr = managedElsewhere(pkg.name, pkg.manager)}
							{#if otherMgr}
								<span class="rounded-full px-3 py-1 text-[10px] font-semibold" style="background-color: var(--bg-tertiary); color: var(--text-muted);">via {otherMgr}</span>
							{:else}
								<button
									class="rounded-full px-3 py-1 text-[10px] font-semibold transition-colors duration-100 hover:opacity-90"
									style="background-color: var(--accent); color: var(--bg-primary);"
									onclick={() => installPkg(pkg.manager as PackageManager, pkg.name)}
								>
									Install
								</button>
							{/if}
						{/if}
					</div>
				{/each}
			</div>
		{:else if hasSearched}
			<EmptyState title="No results found" message={`No packages matching "${query}"`} />
		{:else}
			<!-- Featured / Recommended -->
			<div class="flex flex-col gap-6 px-6 py-5">
				{#each featured as category}
					<div class="flex flex-col gap-2.5">
						<span class="text-[12px] font-semibold" style="color: var(--text-primary);">
							{category.title}
						</span>
						<div class="flex gap-2.5 overflow-x-auto pb-1">
							{#each category.packages as pkg}
								{@const installed = isInstalled(pkg.name, pkg.manager)}
								<div
									class="flex w-[180px] flex-shrink-0 flex-col gap-2 rounded-lg border p-3 transition-colors duration-100 hover:bg-[var(--bg-hover)]"
									style="border-color: var(--border-subtle);"
								>
									<div class="flex items-center gap-2">
										<PackageIcon manager={pkg.manager} name={pkg.name} size={18} />
										<span class="min-w-0 flex-1 truncate text-[12px] font-semibold" style="color: var(--text-primary);">
											{pkg.name}
										</span>
									</div>
									<p class="line-clamp-2 text-[10px]" style="color: var(--text-muted);">
										{pkg.description}
									</p>
									<div class="flex items-center justify-between">
										<span
											class="rounded-full px-1.5 py-0.5 text-[8px] font-semibold"
											style={`background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}
										>
											{pkg.manager}
										</span>
										{#if installed}
											<span class="text-[10px] font-medium" style="color: var(--success);">Installed</span>
										{:else if managedElsewhere(pkg.name, pkg.manager)}
											<span class="text-[9px]" style="color: var(--text-muted);">via {managedElsewhere(pkg.name, pkg.manager)}</span>
										{:else}
											<button
												class="rounded-md px-2 py-0.5 text-[10px] font-medium transition-colors duration-100 hover:opacity-90"
												style="background-color: var(--accent); color: var(--bg-primary);"
												onclick={() => installPkg(pkg.manager, pkg.name)}
											>
												Install
											</button>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
