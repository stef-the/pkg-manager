<script lang="ts">
	import { getManagers, loadAllPackages, isLoading } from '$lib/stores/packages.svelte';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import { addToast } from '$lib/stores/toast.svelte';
	import { runTask } from '$lib/stores/tasks.svelte';
	import Icon from '$lib/components/Icons.svelte';
	import type { IconName } from '$lib/components/Icons.svelte';
	import { invoke } from '@tauri-apps/api/core';

	function autoInstall(id: string, name: string) {
		runTask(`Installing ${name}`, () => invoke<string>('install_manager', { manager: id }), {
			successMessage: `${name} installed successfully`,
			onSuccess: () => loadAllPackages(true),
			onError: (e) => {
				const msg = `${e}`.toLowerCase();
				if (msg.includes('permission') || msg.includes('sudo') || msg.includes('eacces')) {
					addToast(`Installing ${name} failed: permission denied. Try running with elevated privileges.`, 'error', 6000);
				}
			}
		});
	}

	type Platform = 'macos' | 'windows' | 'linux';

	interface ManagerMeta {
		name: string;
		icon: IconName;
		description: string;
		installUrl: string;
		platforms: Platform[];
	}

	function managerMeta(id: string): ManagerMeta {
		switch (id) {
			case 'brew':
				return { name: 'Homebrew', icon: 'brew', description: 'The Missing Package Manager for macOS (or Linux)', installUrl: 'https://brew.sh', platforms: ['macos', 'linux'] };
			case 'npm':
				return { name: 'npm', icon: 'npm', description: 'Node.js package manager for JavaScript', installUrl: 'https://nodejs.org', platforms: ['macos', 'windows', 'linux'] };
			case 'mas':
				return { name: 'Mac App Store', icon: 'mas', description: 'Install and update Mac App Store apps from the command line', installUrl: 'https://github.com/mas-cli/mas', platforms: ['macos'] };
			case 'pip':
				return { name: 'pip (Python)', icon: 'pip', description: 'Package installer for Python', installUrl: 'https://pip.pypa.io', platforms: ['macos', 'windows', 'linux'] };
			case 'cargo':
				return { name: 'Cargo (Rust)', icon: 'cargo', description: 'Rust package manager and build tool', installUrl: 'https://rustup.rs', platforms: ['macos', 'windows', 'linux'] };
			case 'winget':
				return { name: 'winget', icon: 'winget', description: 'Windows Package Manager from Microsoft', installUrl: 'https://learn.microsoft.com/en-us/windows/package-manager/', platforms: ['windows'] };
			case 'apt':
				return { name: 'apt', icon: 'apt', description: 'Debian/Ubuntu package manager (native or via WSL)', installUrl: 'https://wiki.debian.org/Apt', platforms: ['linux', 'windows'] };
			case 'nix':
				return { name: 'Nix', icon: 'nix', description: 'Reproducible package manager with 100,000+ packages', installUrl: 'https://nixos.org/download', platforms: ['macos', 'linux'] };
			case 'scoop':
				return { name: 'Scoop', icon: 'scoop', description: 'Windows command-line installer (no admin needed)', installUrl: 'https://scoop.sh', platforms: ['windows'] };
			case 'flatpak':
				return { name: 'Flatpak', icon: 'flatpak', description: 'Linux app sandboxing and distribution framework', installUrl: 'https://flatpak.org/setup/', platforms: ['linux'] };
			case 'snap':
				return { name: 'Snap', icon: 'snap', description: 'Universal Linux package format by Canonical', installUrl: 'https://snapcraft.io/docs/installing-snapd', platforms: ['linux'] };
			case 'choco':
				return { name: 'Chocolatey', icon: 'choco', description: 'The package manager for Windows', installUrl: 'https://chocolatey.org/install', platforms: ['windows'] };
			case 'dnf':
				return { name: 'DNF', icon: 'dnf', description: 'Package manager for Fedora, RHEL, and CentOS', installUrl: 'https://fedoraproject.org', platforms: ['linux'] };
			case 'pacman':
				return { name: 'Pacman', icon: 'pacman', description: 'Package manager for Arch Linux and derivatives', installUrl: 'https://archlinux.org', platforms: ['linux'] };
			case 'conda':
				return { name: 'Conda', icon: 'conda', description: 'Package and environment manager for data science', installUrl: 'https://docs.conda.io/en/latest/miniconda.html', platforms: ['macos', 'windows', 'linux'] };
			default:
				return { name: id, icon: 'installed', description: '', installUrl: '', platforms: ['macos', 'windows', 'linux'] };
		}
	}

	function getCurrentPlatform(): Platform {
		const ua = navigator.userAgent.toLowerCase();
		if (ua.includes('mac')) return 'macos';
		if (ua.includes('win')) return 'windows';
		return 'linux';
	}

	function isCompatible(meta: ManagerMeta): boolean {
		return meta.platforms.includes(getCurrentPlatform());
	}

	function platformLabel(p: Platform): string {
		switch (p) { case 'macos': return 'macOS'; case 'windows': return 'Windows'; case 'linux': return 'Linux'; }
	}

	const knownManagers = ['brew', 'npm', 'mas', 'pip', 'cargo', 'conda', 'nix', 'scoop', 'choco', 'winget', 'apt', 'dnf', 'pacman', 'flatpak', 'snap'];
	const detectedManagers = $derived(getManagers());
	const detectedIds = $derived(new Set(detectedManagers.map((m) => m.id)));
</script>

<div class="flex h-full flex-col">
	<div class="flex flex-col gap-1 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Package Managers</h1>
		<p class="text-[13px]" style="color: var(--text-muted);">
			Manage and discover package managers on your system.
		</p>
	</div>

	<div class="flex flex-1 flex-col gap-3 overflow-y-auto px-6 py-5">
		{#if isLoading() && getManagers().length === 0}
			<LoadingSkeleton rows={5} />
		{/if}
		{#each knownManagers as id}
			{@const meta = managerMeta(id)}
			{@const detected = detectedManagers.find((m) => m.id === id)}
			{@const available = detected?.available ?? false}

			<div
				class="flex items-center gap-4 rounded-lg border p-4"
				style="border-color: var(--border-subtle);"
			>
				<div class="flex h-10 w-10 items-center justify-center rounded-lg" style="background-color: var(--bg-primary);">
					<Icon name={meta.icon} size={20} />
				</div>

				<div class="min-w-0 flex-1">
					<div class="flex items-center gap-2">
						<span class="text-[13px] font-semibold" style="color: var(--text-primary);">{meta.name}</span>
						{#if available}
							<span class="rounded-full px-2 py-0.5 text-[10px] font-semibold" style="background-color: var(--success); color: var(--color-nord0);">
								Installed
							</span>
						{:else}
							<span class="rounded-full px-2 py-0.5 text-[10px] font-semibold" style="background-color: var(--bg-tertiary); color: var(--text-muted);">
								Not found
							</span>
						{/if}
					</div>
					<p class="mt-0.5 text-[12px]" style="color: var(--text-muted);">{meta.description}</p>
					<div class="mt-1 flex items-center gap-2">
						{#if detected?.version}
							<span class="font-mono text-[10px]" style="color: var(--text-secondary);">{detected.version}</span>
						{/if}
						<div class="flex gap-1">
							{#each meta.platforms as p}
								<span class="rounded px-1 py-0.5 text-[8px] font-medium" style="background-color: var(--bg-tertiary); color: var(--text-muted);">{platformLabel(p)}</span>
							{/each}
						</div>
					</div>
				</div>

				{#if !available && isCompatible(meta)}
					<div class="flex flex-shrink-0 gap-2">
						<button
							class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
							style="background-color: var(--accent); color: var(--bg-primary);"
							onclick={() => autoInstall(id, meta.name)}
						>
							Install
						</button>
						{#if meta.installUrl}
							<button
								class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
								style="border-color: var(--border); color: var(--text-secondary);"
								onclick={() => invoke('open_url', { url: meta.installUrl })}
							>
								Guide
							</button>
						{/if}
					</div>
				{:else if !available && !isCompatible(meta)}
					<span class="flex-shrink-0 rounded-lg px-3 py-1.5 text-[11px]" style="color: var(--text-muted);">
						Not available on {platformLabel(getCurrentPlatform())}
					</span>
				{/if}
			</div>
		{/each}
	</div>
</div>
