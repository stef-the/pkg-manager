<script lang="ts">
	import { createLogger } from '$lib/utils/logger';
	import { addToast } from '$lib/stores/toast.svelte';

	const log = createLogger('app-updater');

	let updateAvailable = $state(false);
	let updating = $state(false);
	let version = $state('');

	$effect(() => {
		if (typeof window === 'undefined' || !('__TAURI__' in window)) return;

		// Check for app updates on launch (after 10s delay)
		setTimeout(async () => {
			try {
				const { check } = await import('@tauri-apps/plugin-updater');
				const update = await check();
				if (update) {
					version = update.version;
					updateAvailable = true;
					log.info(`App update available: v${update.version}`);
					addToast(`App update v${update.version} available`, 'info', 10000);
				}
			} catch (e) {
				log.debug(`Update check: ${e}`);
			}
		}, 10000);
	});

	async function installUpdate() {
		updating = true;
		try {
			const { check } = await import('@tauri-apps/plugin-updater');
			const update = await check();
			if (update) {
				addToast('Downloading update...', 'info');
				await update.downloadAndInstall();
				addToast('Update installed! Restarting...', 'success');
				// Restart the app
				const { relaunch } = await import('@tauri-apps/plugin-process');
				await relaunch();
			}
		} catch (e) {
			log.error(`Update failed: ${e}`);
			addToast(`Update failed: ${e}`, 'error');
		} finally {
			updating = false;
		}
	}
</script>

{#if updateAvailable}
	<div
		class="flex items-center gap-3 border-t px-4 py-2"
		style="border-color: var(--border-subtle); background-color: var(--accent)10;"
	>
		<span class="flex-1 text-[11px] font-medium" style="color: var(--accent);">
			v{version} available
		</span>
		<button
			class="rounded-md px-2.5 py-1 text-[10px] font-semibold transition-colors duration-100 hover:opacity-90"
			style="background-color: var(--accent); color: var(--bg-primary);"
			onclick={installUpdate}
			disabled={updating}
		>
			{updating ? 'Updating...' : 'Update'}
		</button>
	</div>
{/if}
