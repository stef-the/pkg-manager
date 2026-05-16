import { invoke } from '@tauri-apps/api/core';
import type { Package, OutdatedPackage, ManagerInfo, SystemStats } from '$lib/types';
import { createLogger } from './logger';

const log = createLogger('commands');

export async function listPackages(manager: string): Promise<Package[]> {
	log.info(`Listing packages for ${manager}`);
	try {
		const result = await invoke<Package[]>('list_packages', { manager });
		log.info(`Listed ${result.length} packages for ${manager}`);
		return result;
	} catch (e) {
		log.error(`Failed to list packages for ${manager}: ${e}`);
		throw e;
	}
}

export async function searchPackages(manager: string, query: string): Promise<Package[]> {
	log.info(`Searching ${manager} for "${query}"`);
	try {
		const result = await invoke<Package[]>('search_packages', { manager, query });
		log.info(`Found ${result.length} results for "${query}" in ${manager}`);
		return result;
	} catch (e) {
		log.error(`Failed to search ${manager} for "${query}": ${e}`);
		throw e;
	}
}

export async function getOutdated(manager: string): Promise<OutdatedPackage[]> {
	log.info(`Checking outdated packages for ${manager}`);
	try {
		const result = await invoke<OutdatedPackage[]>('get_outdated', { manager });
		log.info(`Found ${result.length} outdated packages for ${manager}`);
		return result;
	} catch (e) {
		log.error(`Failed to get outdated packages for ${manager}: ${e}`);
		throw e;
	}
}

export async function installPackage(manager: string, name: string): Promise<void> {
	log.info(`Installing ${name} via ${manager}`);
	try {
		await invoke<void>('install_package', { manager, name });
		log.info(`Successfully installed ${name} via ${manager}`);
	} catch (e) {
		log.error(`Failed to install ${name} via ${manager}: ${e}`);
		throw e;
	}
}

export async function uninstallPackage(manager: string, name: string): Promise<void> {
	log.info(`Uninstalling ${name} via ${manager}`);
	try {
		await invoke<void>('uninstall_package', { manager, name });
		log.info(`Successfully uninstalled ${name} via ${manager}`);
	} catch (e) {
		log.error(`Failed to uninstall ${name} via ${manager}: ${e}`);
		throw e;
	}
}

export async function updatePackage(manager: string, name: string): Promise<void> {
	log.info(`Updating ${name} via ${manager}`);
	try {
		await invoke<void>('update_package', { manager, name });
		log.info(`Successfully updated ${name} via ${manager}`);
	} catch (e) {
		log.error(`Failed to update ${name} via ${manager}: ${e}`);
		throw e;
	}
}

export async function getPackageManagers(): Promise<ManagerInfo[]> {
	log.info('Getting available package managers');
	try {
		const result = await invoke<ManagerInfo[]>('get_package_managers');
		log.info(`Found ${result.length} package managers`);
		return result;
	} catch (e) {
		log.error(`Failed to get package managers: ${e}`);
		throw e;
	}
}

export async function enrichDescriptions(manager: string, names: string[]): Promise<[string, string][]> {
	try {
		return await invoke<[string, string][]>('enrich_descriptions', { manager, names });
	} catch (e) {
		log.warn(`Failed to enrich descriptions: ${e}`);
		return [];
	}
}

export async function getSystemStats(): Promise<SystemStats> {
	log.info('Getting system stats');
	try {
		const result = await invoke<SystemStats>('get_system_stats');
		log.info(`System: ${result.os}`);
		return result;
	} catch (e) {
		log.error(`Failed to get system stats: ${e}`);
		throw e;
	}
}
