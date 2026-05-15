import { test, expect } from '@playwright/test';

test.describe('Theme', () => {
	test('dark mode is default', async ({ page }) => {
		await page.goto('/');
		const html = page.locator('html');
		await expect(html).toHaveClass(/dark/);
	});

	test('theme can be toggled in preferences', async ({ page }) => {
		await page.goto('/');
		await page.click('button:has-text("Preferences")');

		// Click Light
		await page.click('button:has-text("Light")');
		const html = page.locator('html');
		await expect(html).not.toHaveClass(/dark/);

		// Click Dark
		await page.click('button:has-text("Dark")');
		await expect(html).toHaveClass(/dark/);
	});
});
