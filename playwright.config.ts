import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
	testDir: 'tests/e2e',
	fullyParallel: true,
	retries: 0,
	reporter: 'html',
	use: {
		baseURL: 'http://localhost:5180',
		trace: 'on-first-retry'
	},
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		}
	],
	webServer: {
		command: 'npm run dev -- --port 5180',
		port: 5180,
		reuseExistingServer: true
	}
});
