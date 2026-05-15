<script lang="ts">
	import { getAvailableManagers, setActiveView } from '$lib/stores/packages.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import { runTask } from '$lib/stores/tasks.svelte';
	import { createLogger } from '$lib/utils/logger';
	import { invoke } from '@tauri-apps/api/core';
	import type { PackageManager } from '$lib/types';

	const log = createLogger('terminal');

	let command = $state('');
	let selectedManager = $state<PackageManager>('brew');
	let history = $state<{ cmd: string; output: string; isError: boolean }[]>([]);
	let running = $state(false);

	function runCommand() {
		if (!command.trim() || running) return;
		const cmd = command.trim();
		const fullCmd = `${selectedManager} ${cmd}`;
		command = '';
		running = true;

		runTask(fullCmd, () => invoke<string>('run_terminal_command', {
			manager: selectedManager,
			args: cmd
		}), {
			onSuccess: (result) => {
				history = [...history, { cmd: fullCmd, output: result, isError: false }];
				running = false;
			},
			onError: (e) => {
				history = [...history, { cmd: fullCmd, output: `${e}`, isError: true }];
				running = false;
			}
		});
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			runCommand();
		}
	}

	function managerDisplayName(id: string): string {
		switch (id) {
			case 'brew':
				return 'brew';
			case 'npm':
				return 'npm';
			default:
				return id;
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex flex-col gap-4 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Terminal</h1>
		<p class="text-[13px]" style="color: var(--text-muted);">
			Run package manager commands directly.
		</p>
	</div>

	<!-- Output -->
	<div class="flex-1 overflow-y-auto px-6 py-4">
		{#if getAvailableManagers().length === 0}
			<EmptyState
				variant="warning"
				title="No package managers available"
				message="Install a package manager to use the terminal."
				actionLabel="Go to Managers"
				onaction={() => setActiveView('managers')}
			/>
		{:else if history.length === 0}
			<div class="flex h-full items-center justify-center">
				<span class="text-[13px]" style="color: var(--text-muted);">
					Run a command below to get started.
				</span>
			</div>
		{:else}
			<div class="flex flex-col gap-3">
				{#each history as entry}
					<div class="flex flex-col gap-1">
						<span class="font-mono text-[12px] font-semibold" style="color: var(--accent);">
							$ {entry.cmd}
						</span>
						<pre
							class="overflow-x-auto rounded-lg p-3 font-mono text-[11px] leading-relaxed"
							style={`background-color: var(--bg-primary); color: ${entry.isError ? 'var(--error)' : 'var(--text-secondary)'};`}
						>{entry.output}</pre>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Input -->
	<div class="border-t px-6 py-3" style="border-color: var(--border-subtle);">
		<div class="flex items-center gap-2">
			<!-- Manager selector -->
			<div class="flex items-center gap-1 rounded-lg p-0.5" style="background-color: var(--bg-primary);">
				{#each getAvailableManagers() as manager}
					{@const isActive = selectedManager === manager.id}
					<button
						class="rounded-md px-2.5 py-1 text-[11px] font-medium transition-colors duration-100"
						style={isActive
							? 'background-color: var(--accent); color: var(--bg-primary);'
							: 'color: var(--text-muted);'}
						onclick={() => (selectedManager = manager.id as PackageManager)}
					>
						{managerDisplayName(manager.id)}
					</button>
				{/each}
			</div>

			<!-- Command input -->
			<div class="flex flex-1 items-center gap-2">
				<span class="font-mono text-[13px] font-medium" style="color: var(--accent);">$</span>
				<input
					type="text"
					bind:value={command}
					onkeydown={handleKeydown}
					placeholder={`${selectedManager} ...`}
					class="flex-1 bg-transparent py-1.5 font-mono text-[13px] outline-none"
					style="color: var(--text-primary);"
					disabled={running}
				/>
			</div>

			<button
				class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
				style="background-color: var(--accent); color: var(--bg-primary);"
				onclick={runCommand}
				disabled={running || !command.trim()}
			>
				{running ? '...' : 'Run'}
			</button>
		</div>
	</div>
</div>
