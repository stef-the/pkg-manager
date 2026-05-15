<script lang="ts">
	import { getAllPackages, uninstallPkg } from '$lib/stores/packages.svelte';
	import { getDateFormat } from '$lib/stores/theme.svelte';
	import { addToast } from '$lib/stores/toast.svelte';
	import { runTask } from '$lib/stores/tasks.svelte';
	import { createLogger } from '$lib/utils/logger';
	import type { Package, PackageManager } from '$lib/types';

	const log = createLogger('debloat');

	// Generate a date range: 365 days back from today
	const today = new Date();
	const oneYearAgo = new Date(today);
	oneYearAgo.setFullYear(today.getFullYear() - 1);

	const totalDays = Math.floor((today.getTime() - oneYearAgo.getTime()) / (1000 * 60 * 60 * 24));

	let sliderValue = $state(180); // default: 6 months back
	let selectedPackages = $state<Set<string>>(new Set());
	let calendarOpen = $state(false);
	let calendarMonth = $state(new Date(today.getFullYear(), today.getMonth() - 6, 1));

	const cutoffDate = $derived((() => {
		const d = new Date(today);
		d.setDate(d.getDate() - sliderValue);
		return d;
	})());

	function setDateFromCalendar(date: Date) {
		const diffMs = today.getTime() - date.getTime();
		sliderValue = Math.max(0, Math.min(totalDays, Math.round(diffMs / (1000 * 60 * 60 * 24))));
		calendarOpen = false;
	}

	function prevMonth() {
		calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() - 1, 1);
	}

	function nextMonth() {
		calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() + 1, 1);
	}

	const calendarDays = $derived((() => {
		const year = calendarMonth.getFullYear();
		const month = calendarMonth.getMonth();
		const firstDay = new Date(year, month, 1).getDay();
		const daysInMonth = new Date(year, month + 1, 0).getDate();

		const days: (Date | null)[] = [];
		// Leading empty cells
		for (let i = 0; i < firstDay; i++) days.push(null);
		// Actual days
		for (let d = 1; d <= daysInMonth; d++) days.push(new Date(year, month, d));
		return days;
	})());

	const monthLabel = $derived(
		calendarMonth.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })
	);

	function formatDate(date: Date): string {
		const fmt = getDateFormat();
		const d = date.getDate().toString().padStart(2, '0');
		const m = (date.getMonth() + 1).toString().padStart(2, '0');
		const y = date.getFullYear();
		switch (fmt) {
			case 'dd/mm/yyyy':
				return `${d}/${m}/${y}`;
			case 'mm/dd/yyyy':
				return `${m}/${d}/${y}`;
			case 'yyyy-mm-dd':
				return `${y}-${m}-${d}`;
			default:
				return `${d}/${m}/${y}`;
		}
	}

	// Common bloat packages that are safe to remove
	const recommendedRemovals = new Set([
		// brew: common dev dependencies users often don't need
		'brew/autoconf', 'brew/automake', 'brew/cmake', 'brew/m4', 'brew/pkg-config',
		'brew/libtool', 'brew/bison', 'brew/flex', 'brew/texinfo',
		// npm: common global packages people install and forget
		'npm/create-react-app', 'npm/grunt-cli', 'npm/gulp-cli', 'npm/bower',
		'npm/yo', 'npm/generator-webapp'
	]);

	function isRecommended(pkg: Package): boolean {
		return recommendedRemovals.has(`${pkg.manager}/${pkg.name}`);
	}

	// Sort: selected first, then recommended, then alphabetical
	const candidatePackages = $derived((() => {
		const all = getAllPackages();
		return [...all].sort((a, b) => {
			const aKey = `${a.manager}/${a.name}`;
			const bKey = `${b.manager}/${b.name}`;
			const aSelected = selectedPackages.has(aKey);
			const bSelected = selectedPackages.has(bKey);
			if (aSelected && !bSelected) return -1;
			if (!aSelected && bSelected) return 1;
			const aRec = isRecommended(a);
			const bRec = isRecommended(b);
			if (aRec && !bRec) return -1;
			if (!aRec && bRec) return 1;
			return a.name.localeCompare(b.name);
		});
	})());

	function togglePackage(key: string) {
		const next = new Set(selectedPackages);
		if (next.has(key)) {
			next.delete(key);
		} else {
			next.add(key);
		}
		selectedPackages = next;
	}

	function selectAll() {
		selectedPackages = new Set(candidatePackages.map((p) => `${p.manager}/${p.name}`));
	}

	function selectNone() {
		selectedPackages = new Set();
	}

	let confirmingUninstall = $state(false);

	function requestUninstall() {
		if (selectedPackages.size === 0) return;
		confirmingUninstall = true;
	}

	function confirmUninstall() {
		confirmingUninstall = false;
		const toRemove = [...selectedPackages];
		log.info(`Debloat: uninstalling ${toRemove.length} packages`);

		for (const key of toRemove) {
			const [manager, name] = key.split('/');
			if (manager && name) {
				uninstallPkg(manager as PackageManager, name);
			}
		}
		selectedPackages = new Set();
	}

	function cancelUninstall() {
		confirmingUninstall = false;
	}

	// Tick marks for the slider — show ~8 evenly spaced dates
	const tickCount = 8;
	const ticks = $derived(
		Array.from({ length: tickCount + 1 }, (_, i) => {
			const dayOffset = Math.round((totalDays / tickCount) * i);
			const d = new Date(today);
			d.setDate(d.getDate() - (totalDays - dayOffset));
			return { dayOffset: totalDays - dayOffset, label: formatDate(d) };
		})
	);
</script>

<div class="flex h-full flex-col">
	<div class="flex flex-col gap-4 border-b px-6 py-4" style="border-color: var(--border-subtle);">
		<h1 class="text-lg font-semibold" style="color: var(--text-primary);">Debloat</h1>
		<p class="text-[13px]" style="color: var(--text-muted);">
			Select a cutoff date to find packages you may not need anymore.
		</p>
	</div>

	<div class="flex flex-1 flex-col gap-5 overflow-y-auto px-6 py-5">
		<!-- Date Slider -->
		<div class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
					Not used since
				</span>
				<div class="relative">
					<button
						class="rounded-md px-2.5 py-1 font-mono text-[13px] font-semibold transition-colors duration-100 hover:opacity-80"
						style="background-color: var(--surface-raised); color: var(--accent);"
						onclick={() => {
							calendarMonth = new Date(cutoffDate.getFullYear(), cutoffDate.getMonth(), 1);
							calendarOpen = !calendarOpen;
						}}
					>
						{formatDate(cutoffDate)}
					</button>

					<!-- Mini Calendar Dropdown -->
					{#if calendarOpen}
						<div
							class="absolute right-0 top-full z-30 mt-1 w-[260px] rounded-lg border p-3 shadow-xl"
							style="background-color: var(--surface); border-color: var(--border-subtle);"
						>
							<!-- Month nav -->
							<div class="mb-2 flex items-center justify-between">
								<button
									class="flex h-6 w-6 items-center justify-center rounded-md transition-colors duration-100 hover:bg-[var(--bg-hover)]"
									style="color: var(--text-secondary);"
									aria-label="Previous month"
									onclick={prevMonth}
								>
									<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 3L5 8l5 5" /></svg>
								</button>
								<span class="text-[12px] font-semibold" style="color: var(--text-primary);">
									{monthLabel}
								</span>
								<button
									class="flex h-6 w-6 items-center justify-center rounded-md transition-colors duration-100 hover:bg-[var(--bg-hover)]"
									style="color: var(--text-secondary);"
									aria-label="Next month"
									onclick={nextMonth}
								>
									<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3l5 5-5 5" /></svg>
								</button>
							</div>

							<!-- Day headers -->
							<div class="mb-1 grid grid-cols-7 gap-0">
								{#each ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'] as day}
									<span class="text-center text-[10px] font-medium" style="color: var(--text-muted);">
										{day}
									</span>
								{/each}
							</div>

							<!-- Days grid -->
							<div class="grid grid-cols-7 gap-0">
								{#each calendarDays as day}
									{#if day === null}
										<span></span>
									{:else}
										{@const isToday = day.toDateString() === today.toDateString()}
										{@const isCutoff = day.toDateString() === cutoffDate.toDateString()}
										{@const isFuture = day > today}
										{@const isTooOld = day < oneYearAgo}
										<button
											class="flex h-7 w-full items-center justify-center rounded-md text-[11px] transition-colors duration-75"
											style={isCutoff
												? 'background-color: var(--accent); color: var(--bg-primary); font-weight: 700;'
												: isToday
													? 'color: var(--accent); font-weight: 600;'
													: isFuture || isTooOld
														? 'color: var(--text-muted); opacity: 0.3;'
														: 'color: var(--text-secondary);'}
											disabled={isFuture || isTooOld}
											onclick={() => setDateFromCalendar(day)}
										>
											{day.getDate()}
										</button>
									{/if}
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- Slider -->
			<div class="flex flex-col gap-1">
				<input
					type="range"
					min={0}
					max={totalDays}
					bind:value={sliderValue}
					class="debloat-slider w-full"
				/>

				<!-- Tick labels -->
				<div class="flex justify-between px-0.5">
					{#each ticks as tick}
						<span class="text-[9px]" style="color: var(--text-muted);">
							{tick.label}
						</span>
					{/each}
				</div>
			</div>

			<p class="text-[12px]" style="color: var(--text-secondary);">
				Showing packages potentially unused for {sliderValue} days or more.
			</p>
		</div>

		<!-- Package Selection -->
		<div class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<span class="text-[11px] font-medium uppercase tracking-wider" style="color: var(--text-muted);">
					Candidates ({candidatePackages.length})
				</span>
				<div class="flex gap-2">
					<button
						class="text-[11px] font-medium transition-colors duration-100 hover:opacity-80"
						style="color: var(--accent);"
						onclick={selectAll}
					>
						Select all
					</button>
					<button
						class="text-[11px] font-medium transition-colors duration-100 hover:opacity-80"
						style="color: var(--text-muted);"
						onclick={selectNone}
					>
						Clear
					</button>
				</div>
			</div>

			<div class="flex max-h-[40vh] flex-col overflow-y-auto rounded-lg border" style="border-color: var(--border-subtle);">
				{#each candidatePackages as pkg (pkg.manager + '/' + pkg.name)}
					{@const key = `${pkg.manager}/${pkg.name}`}
					{@const isSelected = selectedPackages.has(key)}
					<button
						class="flex items-center gap-3 border-b px-3 py-2 text-left transition-colors duration-75"
						style="border-color: var(--border-subtle);"
						onclick={() => togglePackage(key)}
					>
						<div
							class="flex h-4 w-4 flex-shrink-0 items-center justify-center rounded border text-[10px] transition-colors duration-100"
							style={isSelected
								? 'background-color: var(--accent); border-color: var(--accent); color: var(--bg-primary);'
								: 'border-color: var(--border);'}
						>
							{#if isSelected}<svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 8.5l3.5 3.5L13 5" /></svg>{/if}
						</div>
						<span class="min-w-0 flex-1 truncate text-[13px]" style="color: var(--text-primary);">
							{pkg.name}
							{#if isRecommended(pkg)}
								<span class="ml-1 rounded-full px-1.5 py-0.5 text-[9px] font-semibold" style="background-color: var(--warning); color: var(--color-nord0);">
									Suggested
								</span>
							{/if}
						</span>
						<span class="font-mono text-[11px]" style="color: var(--text-muted);">
							{pkg.version}
						</span>
						<span
							class="rounded-full px-2 py-0.5 text-[10px] font-semibold"
							style={`background-color: ${pkg.manager === 'brew' ? 'var(--color-nord7)' : 'var(--color-nord11)'}22; color: ${pkg.manager === 'brew' ? 'var(--color-nord7)' : 'var(--color-nord11)'};`}
						>
							{pkg.manager}
						</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Action -->
		{#if selectedPackages.size > 0 && !confirmingUninstall}
			<button
				class="w-fit rounded-lg px-5 py-2.5 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
				style="background-color: var(--error); color: var(--color-nord6);"
				onclick={requestUninstall}
			>
				Uninstall {selectedPackages.size} package{selectedPackages.size > 1 ? 's' : ''}
			</button>
		{/if}

		{#if confirmingUninstall}
			<div
				class="flex items-center gap-3 rounded-lg border p-4"
				style="border-color: var(--error); background-color: var(--bg-primary);"
			>
				<span class="flex-1 text-[13px] font-medium" style="color: var(--text-primary);">
					Are you sure? This will uninstall {selectedPackages.size} package{selectedPackages.size > 1 ? 's' : ''}.
				</span>
				<button
					class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
					style="background-color: var(--error); color: var(--color-nord6);"
					onclick={confirmUninstall}
				>
					Yes, uninstall
				</button>
				<button
					class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={cancelUninstall}
				>
					Cancel
				</button>
			</div>
		{/if}
	</div>
</div>

<style>
	.debloat-slider {
		-webkit-appearance: none;
		appearance: none;
		height: 6px;
		border-radius: 3px;
		background: var(--border);
		outline: none;
	}

	.debloat-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: var(--accent);
		cursor: pointer;
		border: 2px solid var(--bg-secondary);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	.debloat-slider::-moz-range-thumb {
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: var(--accent);
		cursor: pointer;
		border: 2px solid var(--bg-secondary);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}
</style>
