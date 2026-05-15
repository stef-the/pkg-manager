/**
 * Maps common package names to Simple Icons slugs.
 * Icons served from https://cdn.simpleicons.org/{slug}/{color}
 */
const ICON_MAP: Record<string, string> = {
	// Dev tools
	git: 'git',
	node: 'nodedotjs',
	nodejs: 'nodedotjs',
	python: 'python',
	python3: 'python',
	ruby: 'ruby',
	go: 'go',
	rust: 'rust',
	'rust-analyzer': 'rust',
	java: 'openjdk',
	openjdk: 'openjdk',
	php: 'php',
	lua: 'lua',
	perl: 'perl',
	swift: 'swift',
	kotlin: 'kotlin',
	dart: 'dart',
	flutter: 'flutter',
	deno: 'deno',
	bun: 'bun',

	// Build tools
	cmake: 'cmake',
	gradle: 'gradle',
	maven: 'apachemaven',
	webpack: 'webpack',
	vite: 'vite',
	rollup: 'rollupdotjs',
	esbuild: 'esbuild',
	turbo: 'turborepo',
	nx: 'nx',

	// Editors/IDEs
	neovim: 'neovim',
	vim: 'vim',
	emacs: 'gnuemacs',

	// Databases
	postgresql: 'postgresql',
	postgres: 'postgresql',
	mysql: 'mysql',
	redis: 'redis',
	sqlite: 'sqlite',
	mongodb: 'mongodb',

	// Containers & infra
	docker: 'docker',
	podman: 'podman',
	kubernetes: 'kubernetes',
	kubectl: 'kubernetes',
	terraform: 'terraform',
	ansible: 'ansible',
	nginx: 'nginx',
	apache: 'apache',

	// Cloud CLIs
	'awscli': 'amazonwebservices',
	'aws-cli': 'amazonwebservices',
	'azure-cli': 'microsoftazure',
	'google-cloud-sdk': 'googlecloud',
	gh: 'github',
	'github-cli': 'github',

	// JS ecosystem
	typescript: 'typescript',
	eslint: 'eslint',
	prettier: 'prettier',
	react: 'react',
	'react-dom': 'react',
	svelte: 'svelte',
	vue: 'vuedotjs',
	angular: 'angular',
	'next': 'nextdotjs',
	nuxt: 'nuxtdotjs',
	express: 'express',
	fastify: 'fastify',
	jest: 'jest',
	vitest: 'vitest',
	playwright: 'playwright',
	cypress: 'cypress',
	tailwindcss: 'tailwindcss',
	prisma: 'prisma',
	graphql: 'graphql',
	apollo: 'apollographql',
	axios: 'axios',
	lodash: 'lodash',
	'date-fns': 'datefns',
	zod: 'zod',
	trpc: 'trpc',
	'socket.io': 'socketdotio',
	storybook: 'storybook',
	babel: 'babel',

	// Python ecosystem
	flask: 'flask',
	django: 'django',
	fastapi: 'fastapi',
	numpy: 'numpy',
	pandas: 'pandas',
	scipy: 'scipy',
	pytorch: 'pytorch',
	tensorflow: 'tensorflow',
	jupyter: 'jupyter',
	black: 'python',
	mypy: 'python',
	ruff: 'ruff',

	// Rust ecosystem
	'cargo-edit': 'rust',
	'cargo-watch': 'rust',
	tokio: 'rust',

	// CLI tools
	wget: 'gnu',
	curl: 'curl',
	ffmpeg: 'ffmpeg',
	imagemagick: 'imagemagick',
	htop: 'gnubash',
	tmux: 'tmux',
	ripgrep: 'rust',
	fd: 'rust',
	bat: 'rust',
	fzf: 'gnubash',
	jq: 'json',
	yq: 'yaml',

	// Apps
	firefox: 'firefox',
	'google-chrome': 'googlechrome',
	'visual-studio-code': 'visualstudiocode',
	slack: 'slack',
	discord: 'discord',
	spotify: 'spotify',
	vlc: 'vlcmediaplayer',
	iterm2: 'iterm2',
	'1password': '1password',
	figma: 'figma',
	notion: 'notion',
	obsidian: 'obsidian',
	'raycast': 'raycast',
	'alfred': 'alfred',
};

/**
 * Get a Simple Icons CDN URL for a package name.
 * Returns null if no mapping exists.
 */
export function getSimpleIconUrl(name: string, color = '88c0d0'): string | null {
	const slug = ICON_MAP[name.toLowerCase()];
	if (!slug) return null;
	return `https://cdn.simpleicons.org/${slug}/${color}`;
}

/**
 * Check if a package has a known icon.
 */
export function hasKnownIcon(name: string): boolean {
	return name.toLowerCase() in ICON_MAP;
}
