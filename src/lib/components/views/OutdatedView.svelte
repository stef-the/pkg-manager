<script lang="ts">
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import {
		getAllOutdated,
		isLoadingOutdated,
		updatePkg,
		updateAllOutdated,
		isPackageOutdated
	} from '$lib/stores/packages.svelte';
	import type { OutdatedPackage, PackageManager } from '$lib/types';

	let selectedForUpdate = $state<Set<string>>(new Set());

	function toggleSelect(pkg: OutdatedPackage) {
		const key = `${pkg.manager}/${pkg.name}`;
		const next = new Set(selectedForUpdate);
		if (next.has(key)) next.delete(key);
		else next.add(key);
		selectedForUpdate = next;
	}

	function selectAllForUpdate() {
		selectedForUpdate = new Set(getAllOutdated().map((p) => `${p.manager}/${p.name}`));
	}

	function selectNoneForUpdate() {
		selectedForUpdate = new Set();
	}

	function updateSelected() {
		for (const key of selectedForUpdate) {
			const [manager, name] = key.split('/');
			if (manager && name) {
				updatePkg(manager as PackageManager, name);
			}
		}
		selectedForUpdate = new Set();
	}

	function managerColor(id: string): string {
		return id === 'brew' ? 'var(--color-nord7)' : 'var(--color-nord11)';
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex items-center justify-between border-b px-6 py-5" style="border-color: var(--border-subtle);">
		<div class="flex flex-col gap-0.5">
			<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Outdated Packages</h1>
			<span class="text-xs" style="color: var(--text-muted);">
				{getAllOutdated().length} package{getAllOutdated().length !== 1 ? 's have' : ' has'} updates available
			</span>
		</div>
		<div class="flex gap-2">
			{#if selectedForUpdate.size > 0}
				<button
					class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={selectNoneForUpdate}
				>
					Clear
				</button>
				<button
					class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
					style="background-color: var(--accent); color: var(--bg-primary);"
					onclick={updateSelected}
				>
					Update {selectedForUpdate.size} selected
				</button>
			{:else if getAllOutdated().length > 0}
				<button
					class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={selectAllForUpdate}
				>
					Select all
				</button>
				<button
					class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
					style="background-color: var(--accent); color: var(--bg-primary);"
					onclick={() => updateAllOutdated()}
				>
					Update All
				</button>
			{/if}
		</div>
	</div>

	<!-- Package List -->
	<div class="flex-1 overflow-y-auto">
		{#if isLoadingOutdated()}
			<LoadingSkeleton rows={4} />
		{:else if getAllOutdated().length === 0}
			<EmptyState
				title="All up to date"
				message="All your packages are on the latest version"
			/>
		{:else}
			<div class="flex flex-col">
				{#each getAllOutdated() as pkg (pkg.manager + '/' + pkg.name)}
					{@const key = `${pkg.manager}/${pkg.name}`}
					{@const isSelected = selectedForUpdate.has(key)}
					<div
						class="flex items-center gap-3 border-b px-4 py-2.5"
						style="border-color: var(--border-subtle);"
					>
						<!-- Checkbox -->
						<button
							class="flex h-4 w-4 flex-shrink-0 items-center justify-center rounded border text-[10px] transition-colors duration-100"
							style={isSelected
								? 'background-color: var(--accent); border-color: var(--accent); color: var(--bg-primary);'
								: 'border-color: var(--border);'}
							onclick={() => toggleSelect(pkg)}
						>
							{#if isSelected}
								<svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 8.5l3.5 3.5L13 5" /></svg>
							{/if}
						</button>

						<!-- Name -->
						<span class="w-40 min-w-0 truncate text-[13px] font-semibold" style="color: var(--text-primary);">
							{pkg.name}
						</span>

						<!-- Version change -->
						<span class="font-mono text-[12px]" style="color: var(--text-muted);">
							{pkg.currentVersion}
						</span>
						<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="var(--text-muted)" stroke-width="1.5" stroke-linecap="round"><path d="M4 8h8M9 5l3 3-3 3" /></svg>
						<span class="font-mono text-[12px]" style="color: var(--success);">
							{pkg.latestVersion}
						</span>

						<!-- Spacer -->
						<div class="flex-1"></div>

						<!-- Manager badge -->
						<span
							class="rounded-full px-2 py-0.5 text-[10px] font-semibold"
							style={`background-color: ${managerColor(pkg.manager)}22; color: ${managerColor(pkg.manager)};`}
						>
							{pkg.manager}
						</span>

						<!-- Individual update button -->
						<button
							class="rounded-md px-3 py-1 text-[11px] font-medium transition-colors duration-100 hover:opacity-90"
							style="background-color: var(--accent); color: var(--bg-primary);"
							onclick={() => updatePkg(pkg.manager as PackageManager, pkg.name)}
						>
							Update
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
