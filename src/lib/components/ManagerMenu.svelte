<script lang="ts">
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import type { ManagerInfo, PackageManager } from '$lib/types';
	import {
		getPackages,
		getOutdatedPackages,
		setActiveView,
		setActiveManagerFilter
	} from '$lib/stores/packages.svelte';
	import { runTask } from '$lib/stores/tasks.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let {
		manager,
		anchor,
		onclose
	}: {
		manager: ManagerInfo;
		anchor: { x: number; y: number };
		onclose: () => void;
	} = $props();

	function managerIcon(id: string): IconName {
		const map: Record<string, IconName> = {
			brew: 'brew', npm: 'npm', winget: 'winget', mas: 'mas',
			pip: 'pip', cargo: 'cargo', apt: 'apt', flatpak: 'flatpak',
			snap: 'snap', nix: 'nix', scoop: 'scoop', choco: 'choco', dnf: 'dnf', pacman: 'pacman', conda: 'conda'
		};
		return map[id] ?? 'installed';
	}

	function infoUrl(id: string): string {
		switch (id) {
			case 'brew': return 'https://brew.sh';
			case 'npm': return 'https://www.npmjs.com';
			case 'mas': return 'https://github.com/mas-cli/mas';
			case 'pip': return 'https://pypi.org';
			case 'cargo': return 'https://crates.io';
			case 'winget': return 'https://winget.run';
			case 'apt': return 'https://packages.ubuntu.com';
			case 'flatpak': return 'https://flathub.org';
			case 'snap': return 'https://snapcraft.io';
			case 'nix': return 'https://search.nixos.org/packages';
			case 'scoop': return 'https://scoop.sh';
			default: return '';
		}
	}

	const pkgCount = $derived(getPackages().get(manager.id as PackageManager)?.length ?? 0);
	const outdatedCount = $derived(getOutdatedPackages().get(manager.id as PackageManager)?.length ?? 0);

	function viewInstalled() {
		setActiveManagerFilter(manager.id as PackageManager);
		setActiveView('installed');
		onclose();
	}

	function viewOutdated() {
		setActiveManagerFilter(manager.id as PackageManager);
		setActiveView('outdated');
		onclose();
	}

	function browsePackages() {
		setActiveView('browse');
		onclose();
	}

	function runCleanup() {
		runTask(`${manager.name} cleanup`, () => invoke<string>('run_cleanup', { manager: manager.id }), {
			successMessage: `${manager.name} cleanup complete`
		});
		onclose();
	}

	function runDoctor() {
		runTask(`${manager.name} doctor`, () => invoke<string>('run_doctor', { manager: manager.id }), {
			successMessage: `${manager.name} doctor complete`
		});
		onclose();
	}

	async function openWebsite() {
		const url = infoUrl(manager.id);
		if (url) {
			await invoke('open_url', { url });
		}
		onclose();
	}

	// Position the menu near the clicked item
	const menuStyle = $derived(`top: ${anchor.y}px; left: ${anchor.x}px;`);
</script>

<!-- Backdrop -->
<button
	class="fixed inset-0 z-40"
	style="background: transparent;"
	onclick={onclose}
	aria-label="Close menu"
></button>

<!-- Menu -->
<div
	class="fixed z-50 w-[220px] rounded-lg border py-1 shadow-xl"
	style={`${menuStyle} background-color: var(--surface); border-color: var(--border-subtle);`}
>
	<!-- Header -->
	<div class="flex items-center gap-2.5 border-b px-3 py-2.5" style="border-color: var(--border-subtle);">
		<Icon name={managerIcon(manager.id)} size={16} />
		<div class="flex flex-col">
			<span class="text-[12px] font-semibold" style="color: var(--text-primary);">{manager.name}</span>
			{#if manager.version}
				<span class="font-mono text-[10px]" style="color: var(--text-muted);">{manager.version}</span>
			{/if}
		</div>
	</div>

	<!-- Stats -->
	<div class="flex gap-4 border-b px-3 py-2" style="border-color: var(--border-subtle);">
		<div class="flex flex-col">
			<span class="font-mono text-[13px] font-bold" style="color: var(--text-primary);">{pkgCount}</span>
			<span class="text-[9px]" style="color: var(--text-muted);">installed</span>
		</div>
		{#if outdatedCount > 0}
			<div class="flex flex-col">
				<span class="font-mono text-[13px] font-bold" style="color: var(--warning);">{outdatedCount}</span>
				<span class="text-[9px]" style="color: var(--text-muted);">outdated</span>
			</div>
		{/if}
	</div>

	<!-- Actions -->
	<div class="flex flex-col py-1">
		<button
			class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-primary);"
			onclick={viewInstalled}
		>
			<Icon name="installed" size={13} />
			View installed
		</button>

		{#if outdatedCount > 0}
			<button
				class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
				style="color: var(--text-primary);"
				onclick={viewOutdated}
			>
				<Icon name="outdated" size={13} />
				View outdated ({outdatedCount})
			</button>
		{/if}

		<button
			class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-primary);"
			onclick={browsePackages}
		>
			<Icon name="browse" size={13} />
			Browse packages
		</button>

		<div class="my-1 border-t" style="border-color: var(--border-subtle);"></div>

		<button
			class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-primary);"
			onclick={runCleanup}
		>
			<Icon name="cleanup" size={13} />
			Run cleanup
		</button>

		<button
			class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-primary);"
			onclick={runDoctor}
		>
			<Icon name="search" size={13} />
			Run doctor
		</button>

		<div class="my-1 border-t" style="border-color: var(--border-subtle);"></div>

		<button
			class="flex items-center gap-2.5 px-3 py-1.5 text-left text-[12px] transition-colors duration-75 hover:bg-[var(--bg-hover)]"
			style="color: var(--text-primary);"
			onclick={openWebsite}
		>
			<svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3H3v10h10v-3" /><path d="M9 2h5v5" /><path d="M14 2L7 9" /></svg>
			Open website
		</button>
	</div>
</div>
