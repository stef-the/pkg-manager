<script lang="ts">
	import { getThemePreference, setThemePreference, getDateFormat, setDateFormat } from '$lib/stores/theme.svelte';
	import { createLogger } from '$lib/utils/logger';
	import type { ThemeMode, DateFormat } from '$lib/types';

	const log = createLogger('preferences');

	let { open = $bindable(false) }: { open: boolean } = $props();

	let autostart = $state(false);
	let autostartLoading = $state(false);

	async function loadAutostart() {
		if (typeof window === 'undefined' || !('__TAURI__' in window)) return;
		try {
			const mod = await import('@tauri-apps/plugin-autostart');
			autostart = await mod.isEnabled();
		} catch (e) {
			log.warn(`Failed to check autostart status: ${e}`);
		}
	}

	async function toggleAutostart() {
		if (typeof window === 'undefined' || !('__TAURI__' in window)) return;
		autostartLoading = true;
		try {
			const mod = await import('@tauri-apps/plugin-autostart');
			if (autostart) {
				await mod.disable();
				autostart = false;
				log.info('Autostart disabled');
			} else {
				await mod.enable();
				autostart = true;
				log.info('Autostart enabled');
			}
		} catch (e) {
			log.error(`Failed to toggle autostart: ${e}`);
		} finally {
			autostartLoading = false;
		}
	}

	$effect(() => {
		if (open) {
			loadAutostart();
		}
	});

	const themeOptions: { value: ThemeMode; label: string; description: string }[] = [
		{ value: 'system', label: 'System', description: 'Follow your operating system setting' },
		{ value: 'light', label: 'Light', description: 'Always use light mode' },
		{ value: 'dark', label: 'Dark', description: 'Always use dark mode' }
	];

	const dateFormatOptions: { value: DateFormat; label: string; example: string }[] = [
		{ value: 'dd/mm/yyyy', label: 'DD/MM/YYYY', example: '15/05/2026' },
		{ value: 'mm/dd/yyyy', label: 'MM/DD/YYYY', example: '05/15/2026' },
		{ value: 'yyyy-mm-dd', label: 'YYYY-MM-DD', example: '2026-05-15' }
	];
</script>

{#if open}
	<!-- Backdrop -->
	<button
		class="fixed inset-0 z-40"
		style="background-color: rgba(0, 0, 0, 0.4);"
		onclick={() => (open = false)}
		aria-label="Close preferences"
	></button>

	<!-- Modal -->
	<div
		class="fixed left-1/2 top-1/2 z-50 w-[440px] -translate-x-1/2 -translate-y-1/2 rounded-xl shadow-2xl"
		style="background-color: var(--bg-secondary); border: 1px solid var(--border-subtle);"
	>
		<!-- Header -->
		<div
			class="flex items-center justify-between border-b px-5 py-4"
			style="border-color: var(--border-subtle);"
		>
			<h2 class="text-[13px] font-semibold" style="color: var(--text-primary);">Preferences</h2>
			<button
				class="flex h-6 w-6 items-center justify-center rounded-md text-sm transition-colors duration-100 hover:bg-[var(--bg-hover)]"
				style="color: var(--text-muted);"
				aria-label="Close preferences"
				onclick={() => (open = false)}
			>
				<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 4l8 8M12 4l-8 8" /></svg>
			</button>
		</div>

		<!-- Content -->
		<div class="flex flex-col gap-6 px-5 py-5">
			<!-- Theme -->
			<div class="flex flex-col gap-3">
				<span
					class="text-[10px] font-medium uppercase tracking-wider"
					style="color: var(--text-muted);"
				>
					Appearance
				</span>
				<div class="flex flex-col gap-1.5">
					{#each themeOptions as option}
						{@const isActive = getThemePreference() === option.value}
						<button
							class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-left transition-colors duration-100 hover:bg-[var(--bg-hover)]"
							style={isActive
								? 'background-color: var(--accent); color: var(--bg-primary);'
								: ''}
							onclick={() => setThemePreference(option.value)}
						>
							<div class="flex flex-col">
								<span class="text-[13px] font-medium">{option.label}</span>
								<span
									class="text-[11px]"
									style={isActive ? 'opacity: 0.7;' : 'color: var(--text-muted);'}
								>
									{option.description}
								</span>
							</div>
						</button>
					{/each}
				</div>
			</div>

			<!-- Date Format -->
			<div class="flex flex-col gap-3">
				<span
					class="text-[10px] font-medium uppercase tracking-wider"
					style="color: var(--text-muted);"
				>
					Date Format
				</span>
				<div class="flex gap-2">
					{#each dateFormatOptions as option}
						{@const isActive = getDateFormat() === option.value}
						<button
							class="flex flex-1 flex-col items-center gap-1 rounded-lg px-3 py-2 transition-colors duration-100"
							style={isActive
								? 'background-color: var(--accent); color: var(--bg-primary);'
								: 'background-color: var(--bg-primary);'}
							onclick={() => setDateFormat(option.value)}
						>
							<span class="text-[12px] font-medium">{option.label}</span>
							<span
								class="font-mono text-[10px]"
								style={isActive ? 'opacity: 0.7;' : 'color: var(--text-muted);'}
							>
								{option.example}
							</span>
						</button>
					{/each}
				</div>
			</div>

			<!-- Startup -->
			<div class="flex flex-col gap-3">
				<span
					class="text-[10px] font-medium uppercase tracking-wider"
					style="color: var(--text-muted);"
				>
					Startup
				</span>
				<button
					class="flex items-center justify-between rounded-lg px-3 py-2.5 transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					onclick={toggleAutostart}
					disabled={autostartLoading}
				>
					<div class="flex flex-col">
						<span class="text-[13px] font-medium" style="color: var(--text-primary);">
							Launch on startup
						</span>
						<span class="text-[11px]" style="color: var(--text-muted);">
							Start Pkg Manager when you log in
						</span>
					</div>
					<div
						class="flex h-5 w-9 items-center rounded-full px-0.5 transition-colors duration-200"
						style={autostart
							? 'background-color: var(--accent);'
							: 'background-color: var(--border);'}
					>
						<div
							class="h-4 w-4 rounded-full shadow-sm transition-transform duration-200"
							style={`background-color: var(--bg-primary); transform: translateX(${autostart ? '14px' : '0'});`}
						></div>
					</div>
				</button>
			</div>
		</div>
	</div>
{/if}
