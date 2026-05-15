<script lang="ts">
	import { getAvailableManagers } from '$lib/stores/packages.svelte';
	import { runTask, hasRunningTasks } from '$lib/stores/tasks.svelte';
	import { createLogger } from '$lib/utils/logger';
	import type { PackageManager } from '$lib/types';
	import { invoke } from '@tauri-apps/api/core';

	const log = createLogger('cleanup');

	let output = $state('');

	function runCleanup(manager: PackageManager, name: string) {
		runTask(`${name} cleanup`, () => invoke<string>('run_cleanup', { manager }), {
			successMessage: `${name} cleanup complete`,
			onSuccess: (result) => {
				output = result;
			},
			onError: (e) => {
				output = `Error: ${e}`;
			}
		});
	}

	function runDoctor(manager: PackageManager, name: string) {
		runTask(`${name} doctor`, () => invoke<string>('run_doctor', { manager }), {
			successMessage: `${name} doctor complete`,
			onSuccess: (result) => {
				output = result;
			},
			onError: (e) => {
				output = `Error: ${e}`;
			}
		});
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
	<div class="flex flex-col gap-4 border-b px-6 py-5" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Cleanup & Doctor</h1>
		<p class="text-[13px]" style="color: var(--text-muted);">
			Run maintenance commands to clean caches and diagnose issues.
		</p>
	</div>

	<div class="flex flex-1 flex-col gap-4 overflow-y-auto px-6 py-5">
		{#each getAvailableManagers() as manager}
			<div
				class="flex flex-col gap-3 rounded-lg border p-4"
				style="border-color: var(--border-subtle);"
			>
				<span class="text-[13px] font-semibold" style="color: var(--text-primary);">
					{managerDisplayName(manager.id)}
				</span>
				<div class="flex gap-2">
					<button
						class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
						style="background-color: var(--accent); color: var(--bg-primary);"
						onclick={() => runCleanup(manager.id as PackageManager, managerDisplayName(manager.id))}
					>
						Cleanup
					</button>
					<button
						class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
						style="border-color: var(--border); color: var(--text-primary);"
						onclick={() => runDoctor(manager.id as PackageManager, managerDisplayName(manager.id))}
					>
						Doctor
					</button>
				</div>
			</div>
		{/each}

		{#if output}
			<div class="flex flex-col gap-2">
				<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
					Output
				</span>
				<pre
					class="max-h-80 overflow-auto rounded-lg p-4 font-mono text-[12px] leading-relaxed"
					style="background-color: var(--bg-primary); color: var(--text-secondary);"
				>{output}</pre>
			</div>
		{/if}
	</div>
</div>
