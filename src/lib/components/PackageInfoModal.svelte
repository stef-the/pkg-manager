<script lang="ts">
	import type { PackageDetail } from '$lib/types';
	import { invoke } from '@tauri-apps/api/core';
	import { createLogger } from '$lib/utils/logger';

	const log = createLogger('package-info');

	let {
		open = $bindable(false),
		manager = '',
		packageName = ''
	}: {
		open: boolean;
		manager: string;
		packageName: string;
	} = $props();

	let detail = $state<PackageDetail | null>(null);
	let loading = $state(false);
	let error = $state<string | null>(null);

	$effect(() => {
		if (open && manager && packageName) {
			loadDetail();
		} else {
			detail = null;
			error = null;
		}
	});

	async function loadDetail() {
		loading = true;
		error = null;
		try {
			detail = await invoke<PackageDetail>('get_package_detail', {
				manager,
				name: packageName
			});
		} catch (e) {
			error = `${e}`;
			log.error(`Failed to load package detail: ${e}`);
		} finally {
			loading = false;
		}
	}

	async function openHomepage() {
		if (!detail?.homepage) return;
		try {
			await invoke('open_url', { url: detail.homepage });
		} catch (e) {
			log.error(`Failed to open homepage: ${e}`);
		}
	}

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew': return 'Homebrew';
			case 'npm': return 'npm';
			default: return id;
		}
	}
</script>

{#if open}
	<button
		class="fixed inset-0 z-40"
		style="background-color: rgba(0, 0, 0, 0.4);"
		onclick={() => (open = false)}
		aria-label="Close"
	></button>

	<div
		class="fixed left-1/2 top-1/2 z-50 flex max-h-[80vh] w-[520px] -translate-x-1/2 -translate-y-1/2 flex-col rounded-xl shadow-2xl"
		style="background-color: var(--bg-secondary); border: 1px solid var(--border-subtle);"
	>
		<!-- Header -->
		<div
			class="flex items-center justify-between border-b px-5 py-4"
			style="border-color: var(--border-subtle);"
		>
			<div class="flex items-center gap-2">
				<h2 class="text-sm font-semibold" style="color: var(--text-primary);">
					{packageName}
				</h2>
				<span
					class="rounded-full px-2 py-0.5 text-[10px] font-semibold"
					style="background-color: var(--bg-tertiary); color: var(--text-muted);"
				>
					{managerDisplayName(manager)}
				</span>
			</div>
			<button
				class="flex h-6 w-6 items-center justify-center rounded-md transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style="color: var(--text-muted);"
				aria-label="Close"
				onclick={() => (open = false)}
			>
				<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
			</button>
		</div>

		<!-- Content -->
		<div class="flex-1 overflow-y-auto px-5 py-4">
			{#if loading}
				<div class="flex items-center gap-2 py-8">
					<svg class="animate-spin" width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M8 2a6 6 0 110 12" /></svg>
					<span class="text-[13px]" style="color: var(--text-muted);">Loading package info...</span>
				</div>
			{:else if error}
				<div class="py-8 text-center">
					<p class="text-[13px]" style="color: var(--error);">{error}</p>
				</div>
			{:else if detail}
				<div class="flex flex-col gap-4">
					<!-- Description -->
					{#if detail.description}
						<p class="text-[13px] leading-relaxed" style="color: var(--text-secondary);">
							{detail.description}
						</p>
					{/if}

					<!-- Info grid -->
					<div class="grid grid-cols-2 gap-3">
						<div class="flex flex-col gap-0.5">
							<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Version</span>
							<span class="font-mono text-[13px]" style="color: var(--text-primary);">{detail.version || 'N/A'}</span>
						</div>
						<div class="flex flex-col gap-0.5">
							<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">License</span>
							<span class="text-[13px]" style="color: var(--text-primary);">{detail.license || 'Unknown'}</span>
						</div>
						{#if detail.installedOn}
							<div class="flex flex-col gap-0.5">
								<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Installed</span>
								<span class="text-[13px]" style="color: var(--text-primary);">{detail.installedOn}</span>
							</div>
						{/if}
						{#if detail.installSize}
							<div class="flex flex-col gap-0.5">
								<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Size</span>
								<span class="text-[13px]" style="color: var(--text-primary);">{detail.installSize}</span>
							</div>
						{/if}
					</div>

					<!-- Homepage -->
					{#if detail.homepage}
						<div class="flex flex-col gap-1">
							<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">Homepage</span>
							<button
								class="w-fit text-left text-[13px] underline transition-colors duration-100 hover:opacity-80"
								style="color: var(--accent);"
								onclick={openHomepage}
							>
								{detail.homepage}
							</button>
						</div>
					{/if}

					<!-- Dependencies -->
					{#if detail.dependencies.length > 0}
						<div class="flex flex-col gap-1.5">
							<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
								Dependencies ({detail.dependencies.length})
							</span>
							<div class="flex flex-wrap gap-1.5">
								{#each detail.dependencies as dep}
									<span
										class="rounded-md px-2 py-0.5 text-[11px] font-medium"
										style="background-color: var(--bg-primary); color: var(--text-secondary);"
									>
										{dep}
									</span>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer -->
		{#if detail?.homepage}
			<div class="flex justify-end border-t px-5 py-3" style="border-color: var(--border-subtle);">
				<button
					class="flex items-center gap-1.5 rounded-lg px-4 py-2 text-[12px] font-medium transition-colors duration-100 hover:opacity-90"
					style="background-color: var(--accent); color: var(--bg-primary);"
					onclick={openHomepage}
				>
					<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3H3v10h10v-3" /><path d="M9 2h5v5" /><path d="M14 2L7 9" /></svg>
					Open Website
				</button>
			</div>
		{/if}
	</div>
{/if}
