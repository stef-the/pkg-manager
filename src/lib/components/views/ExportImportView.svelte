<script lang="ts">
	import { getAllPackages, getAvailableManagers, installPkg } from '$lib/stores/packages.svelte';
	import { addToast } from '$lib/stores/toast.svelte';
	import { runBatchTask } from '$lib/stores/tasks.svelte';
	import { createLogger } from '$lib/utils/logger';
	import { invoke } from '@tauri-apps/api/core';
	import type { PackageManager } from '$lib/types';

	const log = createLogger('export-import');

	let importText = $state('');

	function exportList() {
		const pkgs = getAllPackages();
		const lines = pkgs.map((p) => `${p.manager}\t${p.name}\t${p.version}`);
		const header = `# Pkg Manager Export — ${new Date().toISOString()}\n# manager\tname\tversion\n`;
		const content = header + lines.join('\n');

		const blob = new Blob([content], { type: 'text/plain' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `pkg-manager-export-${Date.now()}.txt`;
		a.click();
		URL.revokeObjectURL(url);

		addToast(`Exported ${pkgs.length} packages`, 'success');
		log.info(`Exported ${pkgs.length} packages`);
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex flex-col gap-4 border-b px-6 py-5" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Export / Import</h1>
		<p class="text-[13px]" style="color: var(--text-muted);">
			Export your installed packages to a file, or import a list to bulk-install.
		</p>
	</div>

	<div class="flex flex-1 flex-col gap-6 overflow-y-auto px-6 py-5">
		<!-- Export -->
		<div class="flex flex-col gap-3">
			<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				Export
			</span>
			<p class="text-[12px]" style="color: var(--text-secondary);">
				Download a list of all installed packages across all managers.
			</p>
			<button
				class="w-fit rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
				style="background-color: var(--accent); color: var(--bg-primary);"
				onclick={exportList}
			>
				Export Package List
			</button>
		</div>

		<!-- Import -->
		<div class="flex flex-col gap-3">
			<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
				Import
			</span>
			<p class="text-[12px]" style="color: var(--text-secondary);">
				Paste a previously exported package list to bulk-install. (Coming soon)
			</p>
			<textarea
				bind:value={importText}
				rows={6}
				placeholder="Paste package list here..."
				class="w-full rounded-lg border p-3 font-mono text-[12px] outline-none transition-colors duration-100"
				style="background-color: var(--surface); border-color: var(--border-subtle); color: var(--text-primary);"
			></textarea>
			<button
				class="w-fit rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
				style="background-color: var(--accent); color: var(--bg-primary);"
				disabled={!importText.trim()}
				onclick={async () => {
					try {
						const parsed = await invoke<[string, string][]>('import_packages', { contents: importText });
						if (parsed.length === 0) {
							addToast('No valid packages found in import text', 'warning');
							return;
						}
						runBatchTask(
							`Importing ${parsed.length} packages`,
							parsed,
							async ([manager, name]) => {
								const { invoke: inv } = await import('@tauri-apps/api/core');
								await inv('install_package', { manager, name });
							},
							{
								itemLabel: ([manager, name]) => `Installing ${name} (${manager})...`,
								successMessage: `Imported ${parsed.length} packages`,
							}
						);
						importText = '';
					} catch (e) {
						addToast(`Import failed: ${e}`, 'error');
					}
				}}
			>
				Import & Install
			</button>
		</div>
	</div>
</div>
