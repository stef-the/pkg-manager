import type { Package, OutdatedPackage, ManagerInfo, PackageManager, ViewId, SystemStats } from '$lib/types';
import * as commands from '$lib/utils/commands';
import { addToast } from '$lib/stores/toast.svelte';
import { runTask, runBatchTask } from '$lib/stores/tasks.svelte';
import { recordAction } from '$lib/stores/history.svelte';
import { createLogger } from '$lib/utils/logger';

const log = createLogger('packages');

// --- State ---
let packages = $state<Map<PackageManager, Package[]>>(new Map());
let outdatedPackages = $state<Map<PackageManager, OutdatedPackage[]>>(new Map());
let managers = $state<ManagerInfo[]>([]);
let selectedPackage = $state<Package | null>(null);
let loading = $state(false);
let loadingOutdated = $state(false);
let loadingAction = $state<string | null>(null);
let hasLoaded = false;
let error = $state<string | null>(null);
let searchError = $state<string | null>(null);
let loadingSearch = $state(false);
let searchQuery = $state('');
let searchResults = $state<Package[]>([]);
let activeView = $state<ViewId>('dashboard');
let activeManagerFilter = $state<PackageManager | 'all'>('all');
let systemStats = $state<SystemStats | null>(null);
let lastRefreshed = $state<Date | null>(null);

// --- Derived (optimized: sort once, filter is cheap) ---
const allPackages: Package[] = $derived.by(() => {
	const result: Package[] = [];
	for (const pkgs of packages.values()) {
		result.push(...pkgs);
	}
	// Sort once when package list changes (not on every filter/search)
	return result.sort((a, b) => a.name.localeCompare(b.name));
});

// Pre-compute lowercase name+description for faster search matching
const searchIndex = $derived(
	allPackages.map((p) => ({
		pkg: p,
		lower: `${p.name}\0${p.description}`.toLowerCase()
	}))
);

const filteredPackages: Package[] = $derived.by(() => {
	let items = searchIndex;
	if (activeManagerFilter !== 'all') {
		items = items.filter((i) => i.pkg.manager === activeManagerFilter);
	}
	if (searchQuery.trim()) {
		const q = searchQuery.toLowerCase().trim();
		items = items.filter((i) => i.lower.includes(q));
	}
	return items.map((i) => i.pkg);
});

const allOutdated: OutdatedPackage[] = $derived.by(() => {
	const result: OutdatedPackage[] = [];
	for (const pkgs of outdatedPackages.values()) {
		result.push(...pkgs);
	}
	return result.sort((a, b) => a.name.localeCompare(b.name));
});

const totalOutdatedCount = $derived(allOutdated.length);

const availableManagers = $derived<ManagerInfo[]>(
	managers.filter((m) => m.available)
);

// --- Actions ---
async function loadManagers(): Promise<void> {
	log.info('Loading package managers...');
	try {
		managers = await commands.getPackageManagers();
		log.info(`Found managers: ${managers.map((m) => `${m.id}(${m.available ? 'available' : 'unavailable'})`).join(', ')}`);
	} catch (e) {
		const msg = `Failed to detect package managers: ${e}`;
		log.error(msg);
		addToast(msg, 'error');
	}
}

async function loadPackagesForManager(manager: PackageManager): Promise<void> {
	log.info(`Loading installed packages for ${manager}...`);
	try {
		const pkgs = await commands.listPackages(manager);
		const updated = new Map(packages);
		updated.set(manager, pkgs);
		packages = updated;
		log.info(`Loaded ${pkgs.length} packages for ${manager}`);
	} catch (e) {
		const msg = `Failed to load ${manager} packages: ${e}`;
		log.error(msg);
		addToast(msg, 'error');
	}
}

async function loadOutdatedForManager(manager: PackageManager): Promise<void> {
	log.info(`Checking outdated packages for ${manager}...`);
	try {
		const pkgs = await commands.getOutdated(manager);
		const updated = new Map(outdatedPackages);
		updated.set(manager, pkgs);
		outdatedPackages = updated;
		log.info(`Found ${pkgs.length} outdated packages for ${manager}`);
	} catch (e) {
		const msg = `Failed to check outdated ${manager} packages: ${e}`;
		log.error(msg);
		addToast(msg, 'error');
	}
}

async function loadSystemStats(): Promise<void> {
	try {
		systemStats = await commands.getSystemStats();
	} catch (e) {
		log.warn(`Failed to load system stats: ${e}`);
	}
}

export function loadAllPackages(force = false): void {
	if (hasLoaded && !force) return;
	hasLoaded = true;
	loading = true;
	loadingOutdated = true;
	error = null;

	// Detect managers (fast)
	runTask('Detecting package managers', async () => {
		await Promise.all([loadManagers(), loadSystemStats()]);
	}, {
		onSuccess: () => {
			const available = managers.filter((m) => m.available);

			if (available.length === 0) {
				loading = false;
				loadingOutdated = false;
				lastRefreshed = new Date();
				return;
			}

			// Track how many installed/outdated loads are pending
			let installedPending = available.length;
			let outdatedPending = available.length;

			// Load installed packages per manager (each as its own task)
			for (const m of available) {
				const name = m.id === 'brew' ? 'Homebrew' : 'npm';
				runTask(`Loading ${name} packages`, async () => {
					await loadPackagesForManager(m.id as PackageManager);
				}, {
					onSuccess: () => {
						installedPending--;
						if (installedPending <= 0) {
							loading = false;
							lastRefreshed = new Date();
						}
					},
					onError: () => {
						installedPending--;
						if (installedPending <= 0) {
							loading = false;
							lastRefreshed = new Date();
						}
					}
				});
			}

			// Load outdated per manager (each as its own task, independent)
			for (const m of available) {
				const name = m.id === 'brew' ? 'Homebrew' : 'npm';
				runTask(`Checking ${name} outdated`, async () => {
					await loadOutdatedForManager(m.id as PackageManager);
				}, {
					onSuccess: () => {
						outdatedPending--;
						if (outdatedPending <= 0) loadingOutdated = false;
					},
					onError: () => {
						outdatedPending--;
						if (outdatedPending <= 0) loadingOutdated = false;
					}
				});
			}
		},
		onError: (e) => {
			error = `${e}`;
			loading = false;
			loadingOutdated = false;
		}
	});
}

export function refreshPackages(): void {
	log.info('Refreshing all packages...');
	loadAllPackages(true);
}

export async function searchRemotePackages(manager: PackageManager, query: string): Promise<void> {
	if (!query.trim()) {
		searchResults = [];
		searchError = null;
		return;
	}
	loadingSearch = true;
	searchError = null;
	try {
		searchResults = await commands.searchPackages(manager, query);
	} catch (e) {
		const msg = `${e}`;
		log.error(`Search failed: ${msg}`);
		searchError = msg;
		searchResults = [];
	} finally {
		loadingSearch = false;
	}
}

function isPermissionError(err: unknown): boolean {
	const msg = `${err}`.toLowerCase();
	return msg.includes('permission denied') ||
		msg.includes('eacces') ||
		msg.includes('requires root') ||
		msg.includes('sudo') ||
		msg.includes('access denied') ||
		msg.includes('operation not permitted');
}

function formatActionError(action: string, name: string, err: unknown): string {
	if (isPermissionError(err)) {
		return `${action} ${name} failed: permission denied. Try running with elevated privileges (e.g. sudo).`;
	}
	return `${action} ${name} failed: ${err}`;
}

export function installPkg(manager: PackageManager, name: string): void {
	runTask(`Installing ${name}`, async () => {
		await commands.installPackage(manager, name);
		await recordAction('install', manager, name);
		await loadPackagesForManager(manager);
	}, {
		successMessage: `Installed ${name}`,
		onError: (e) => {
			addToast(formatActionError('Install', name, e), 'error', 6000);
		}
	});
}

export function uninstallPkg(manager: PackageManager, name: string): void {
	runTask(`Uninstalling ${name}`, async () => {
		await commands.uninstallPackage(manager, name);
		await recordAction('uninstall', manager, name);
		if (selectedPackage?.name === name && selectedPackage?.manager === manager) {
			selectedPackage = null;
		}
		await loadPackagesForManager(manager);
	}, {
		successMessage: `Uninstalled ${name}`,
		onError: (e) => {
			addToast(formatActionError('Uninstall', name, e), 'error', 6000);
		}
	});
}

export function updatePkg(manager: PackageManager, name: string): void {
	runTask(`Updating ${name}`, async () => {
		await commands.updatePackage(manager, name);
		await recordAction('update', manager, name);
		await Promise.all([
			loadPackagesForManager(manager),
			loadOutdatedForManager(manager)
		]);
	}, {
		successMessage: `Updated ${name}`,
		onError: (e) => {
			addToast(formatActionError('Update', name, e), 'error', 6000);
		}
	});
}

export function updateAllOutdated(): void {
	const items = [...allOutdated];
	runBatchTask(
		`Updating ${items.length} packages`,
		items,
		async (pkg) => {
			await commands.updatePackage(pkg.manager, pkg.name);
		},
		{
			itemLabel: (pkg) => `Upgrading ${pkg.name}...`,
			successMessage: 'All packages updated',
			onSuccess: () => {
				loadAllPackages(true);
			}
		}
	);
}

// --- Getters / Setters ---
export function getPackages(): Map<PackageManager, Package[]> {
	return packages;
}

export function getAllPackages(): Package[] {
	return allPackages;
}

export function getFilteredPackages(): Package[] {
	return filteredPackages;
}

export function getOutdatedPackages(): Map<PackageManager, OutdatedPackage[]> {
	return outdatedPackages;
}

export function getAllOutdated(): OutdatedPackage[] {
	return allOutdated;
}

export function getTotalOutdatedCount(): number {
	return totalOutdatedCount;
}

export function getAvailableManagers(): ManagerInfo[] {
	return availableManagers;
}

export function getManagers(): ManagerInfo[] {
	return managers;
}

export function getSelectedPackage(): Package | null {
	return selectedPackage;
}

export function setSelectedPackage(pkg: Package | null): void {
	selectedPackage = pkg;
}

export function isLoading(): boolean {
	return loading;
}

export function isLoadingOutdated(): boolean {
	return loadingOutdated;
}

export function getLoadingAction(): string | null {
	return loadingAction;
}

export function getError(): string | null {
	return error;
}

export function getSearchQuery(): string {
	return searchQuery;
}

export function setSearchQuery(query: string): void {
	searchQuery = query;
}

export function getSearchResults(): Package[] {
	return searchResults;
}

export function getSearchError(): string | null {
	return searchError;
}

export function isLoadingSearch(): boolean {
	return loadingSearch;
}

export function getActiveView(): ViewId {
	return activeView;
}

export function setActiveView(view: ViewId): void {
	activeView = view;
	log.info(`View changed to ${view}`);
}

export function getActiveManagerFilter(): PackageManager | 'all' {
	return activeManagerFilter;
}

export function setActiveManagerFilter(filter: PackageManager | 'all'): void {
	activeManagerFilter = filter;
	log.info(`Manager filter changed to ${filter}`);
}

export function isPackageOutdated(name: string, manager: PackageManager): OutdatedPackage | undefined {
	return allOutdated.find((p: OutdatedPackage) => p.name === name && p.manager === manager);
}

export function getSystemStats(): SystemStats | null {
	return systemStats;
}

export function getLastRefreshed(): Date | null {
	return lastRefreshed;
}
