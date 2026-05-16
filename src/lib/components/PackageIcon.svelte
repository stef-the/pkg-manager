<script lang="ts">
	import Icon from './Icons.svelte';
	import type { IconName } from './Icons.svelte';
	import type { PackageManager } from '$lib/types';
	import { getSimpleIconUrl } from '$lib/utils/icons';

	let {
		manager,
		name,
		size = 16
	}: {
		manager: PackageManager;
		name: string;
		size?: number;
	} = $props();

	let failed = $state(false);

	function managerIcon(id: string): IconName {
		const map: Record<string, IconName> = {
			brew: 'brew', npm: 'npm', winget: 'winget', mas: 'mas',
			pip: 'pip', cargo: 'cargo', apt: 'apt', flatpak: 'flatpak',
			snap: 'snap', nix: 'nix', scoop: 'scoop'
		};
		return map[id] ?? 'installed';
	}

	// Try Simple Icons CDN first (client-side, no Rust needed)
	const iconUrl = $derived(getSimpleIconUrl(name));
</script>

{#if iconUrl && !failed}
	<img
		src={iconUrl}
		alt=""
		width={size}
		height={size}
		class="rounded-sm"
		style="min-width: {size}px; min-height: {size}px;"
		onerror={() => { failed = true; }}
	/>
{:else}
	<span style="opacity: 0.35;">
		<Icon name={managerIcon(manager)} {size} />
	</span>
{/if}
