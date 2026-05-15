import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import {
	listPackages,
	searchPackages,
	getOutdated,
	installPackage,
	uninstallPackage,
	updatePackage,
	getPackageManagers
} from './commands';

const mockInvoke = vi.mocked(invoke);

describe('commands', () => {
	beforeEach(() => {
		mockInvoke.mockReset();
	});

	it('listPackages calls invoke with correct args', async () => {
		const mockPkgs = [{ name: 'wget', version: '1.21', description: '', manager: 'brew' }];
		mockInvoke.mockResolvedValue(mockPkgs);

		const result = await listPackages('brew');
		expect(mockInvoke).toHaveBeenCalledWith('list_packages', { manager: 'brew' });
		expect(result).toEqual(mockPkgs);
	});

	it('searchPackages calls invoke with correct args', async () => {
		mockInvoke.mockResolvedValue([]);
		await searchPackages('npm', 'express');
		expect(mockInvoke).toHaveBeenCalledWith('search_packages', {
			manager: 'npm',
			query: 'express'
		});
	});

	it('getOutdated calls invoke with correct args', async () => {
		mockInvoke.mockResolvedValue([]);
		await getOutdated('brew');
		expect(mockInvoke).toHaveBeenCalledWith('get_outdated', { manager: 'brew' });
	});

	it('installPackage calls invoke with correct args', async () => {
		mockInvoke.mockResolvedValue(undefined);
		await installPackage('brew', 'wget');
		expect(mockInvoke).toHaveBeenCalledWith('install_package', {
			manager: 'brew',
			name: 'wget'
		});
	});

	it('uninstallPackage calls invoke with correct args', async () => {
		mockInvoke.mockResolvedValue(undefined);
		await uninstallPackage('npm', 'eslint');
		expect(mockInvoke).toHaveBeenCalledWith('uninstall_package', {
			manager: 'npm',
			name: 'eslint'
		});
	});

	it('updatePackage calls invoke with correct args', async () => {
		mockInvoke.mockResolvedValue(undefined);
		await updatePackage('brew', 'git');
		expect(mockInvoke).toHaveBeenCalledWith('update_package', {
			manager: 'brew',
			name: 'git'
		});
	});

	it('getPackageManagers calls invoke with correct args', async () => {
		const mockManagers = [
			{ id: 'brew', name: 'Homebrew', available: true },
			{ id: 'npm', name: 'npm', available: true }
		];
		mockInvoke.mockResolvedValue(mockManagers);

		const result = await getPackageManagers();
		expect(mockInvoke).toHaveBeenCalledWith('get_package_managers');
		expect(result).toEqual(mockManagers);
	});

	it('propagates errors from invoke', async () => {
		mockInvoke.mockRejectedValue(new Error('Command failed'));
		await expect(listPackages('brew')).rejects.toThrow('Command failed');
	});
});
