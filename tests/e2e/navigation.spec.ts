import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
	test('app loads with dashboard', async ({ page }) => {
		await page.goto('/');
		await expect(page.locator('text=Dashboard')).toBeVisible();
	});

	test('sidebar navigation works', async ({ page }) => {
		await page.goto('/');

		// Click Installed
		await page.click('button:has-text("Installed")');
		await expect(page.locator('h1:has-text("Installed Packages")')).toBeVisible();

		// Click Outdated
		await page.click('button:has-text("Outdated")');
		await expect(page.locator('h1:has-text("Outdated Packages")')).toBeVisible();

		// Click Browse
		await page.click('button:has-text("Browse")');
		await expect(page.locator('h1:has-text("Browse Packages")')).toBeVisible();

		// Click Managers
		await page.click('button:has-text("Managers")');
		await expect(page.locator('h1:has-text("Package Managers")')).toBeVisible();
	});

	test('preferences modal opens', async ({ page }) => {
		await page.goto('/');
		await page.click('button:has-text("Preferences")');
		await expect(page.locator('h2:has-text("Preferences")')).toBeVisible();
	});
});
