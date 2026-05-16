/**
 * Maps package names to Simple Icons slugs.
 * Icons served from https://cdn.simpleicons.org/{slug}/{color}
 *
 * This is a best-effort mapping. For packages without a known icon,
 * the PackageIcon component falls back to a dimmed manager icon.
 */
const ICON_MAP: Record<string, string> = {
	// --- Languages & Runtimes ---
	git: 'git', node: 'nodedotjs', nodejs: 'nodedotjs', python: 'python',
	python3: 'python', 'python@3.14': 'python', 'python@3.13': 'python', 'python@3.12': 'python',
	ruby: 'ruby', go: 'go', golang: 'go', rust: 'rust', 'rust-analyzer': 'rust',
	java: 'openjdk', openjdk: 'openjdk', php: 'php', lua: 'lua', perl: 'perl',
	swift: 'swift', kotlin: 'kotlin', dart: 'dart', flutter: 'flutter',
	deno: 'deno', bun: 'bun', elixir: 'elixir', erlang: 'erlang',
	haskell: 'haskell', ghc: 'haskell', scala: 'scala', clojure: 'clojure',
	zig: 'zig', nim: 'nim', julia: 'julia', r: 'r', ocaml: 'ocaml',
	dotnet: 'dotnet', 'dotnet-sdk': 'dotnet', mono: 'dotnet',

	// --- Build Tools ---
	cmake: 'cmake', gradle: 'gradle', maven: 'apachemaven', make: 'gnu',
	webpack: 'webpack', vite: 'vite', rollup: 'rollupdotjs', esbuild: 'esbuild',
	turbo: 'turborepo', turborepo: 'turborepo', nx: 'nx', lerna: 'lerna',
	bazel: 'bazel', meson: 'meson', ninja: 'ninja',

	// --- Editors/IDEs ---
	neovim: 'neovim', vim: 'vim', emacs: 'gnuemacs',
	'visual-studio-code': 'visualstudiocode', code: 'visualstudiocode',

	// --- Databases ---
	postgresql: 'postgresql', postgres: 'postgresql', libpq: 'postgresql',
	mysql: 'mysql', 'mysql-client': 'mysql', mariadb: 'mariadb',
	redis: 'redis', sqlite: 'sqlite', mongodb: 'mongodb', 'mongosh': 'mongodb',
	cassandra: 'apachecassandra', couchdb: 'couchbase', neo4j: 'neo4j',
	influxdb: 'influxdb', clickhouse: 'clickhouse',

	// --- Containers & Infra ---
	docker: 'docker', 'docker-compose': 'docker', podman: 'podman',
	kubernetes: 'kubernetes', kubectl: 'kubernetes', helm: 'helm',
	terraform: 'terraform', ansible: 'ansible', vagrant: 'vagrant',
	packer: 'packer', consul: 'consul', vault: 'vault', nomad: 'nomad',
	nginx: 'nginx', apache: 'apache', httpd: 'apache', caddy: 'caddy',
	traefik: 'traefikproxy', envoy: 'envoyproxy', istio: 'istio',
	prometheus: 'prometheus', grafana: 'grafana', datadog: 'datadog',

	// --- Cloud CLIs ---
	awscli: 'amazonwebservices', 'aws-cli': 'amazonwebservices',
	'azure-cli': 'microsoftazure', az: 'microsoftazure',
	'google-cloud-sdk': 'googlecloud', gcloud: 'googlecloud',
	gh: 'github', 'github-cli': 'github', hub: 'github',
	vercel: 'vercel', netlify: 'netlify', 'netlify-cli': 'netlify',
	heroku: 'heroku', fly: 'flydotio', flyctl: 'flydotio',
	'firebase-tools': 'firebase', supabase: 'supabase',
	'aws-cdk': 'amazonwebservices', pulumi: 'pulumi',
	'cloudflare': 'cloudflare', wrangler: 'cloudflare',

	// --- JS Ecosystem ---
	typescript: 'typescript', eslint: 'eslint', prettier: 'prettier',
	react: 'react', 'react-dom': 'react', 'react-native': 'react',
	'create-react-app': 'createreactapp',
	svelte: 'svelte', '@sveltejs/kit': 'svelte', 'svelte-check': 'svelte',
	vue: 'vuedotjs', '@vue/cli': 'vuedotjs', nuxt: 'nuxtdotjs',
	angular: 'angular', '@angular/cli': 'angular',
	next: 'nextdotjs', 'create-next-app': 'nextdotjs',
	astro: 'astro', remix: 'remix', gatsby: 'gatsby', eleventy: '11ty',
	express: 'express', fastify: 'fastify', koa: 'koa', hono: 'hono',
	nest: 'nestjs', '@nestjs/cli': 'nestjs',
	jest: 'jest', vitest: 'vitest', mocha: 'mocha', chai: 'chai',
	playwright: 'playwright', cypress: 'cypress', puppeteer: 'puppeteer',
	tailwindcss: 'tailwindcss', sass: 'sass', less: 'less',
	postcss: 'postcss', 'styled-components': 'styledcomponents',
	prisma: 'prisma', drizzle: 'drizzle', typeorm: 'typeorm', sequelize: 'sequelize',
	graphql: 'graphql', apollo: 'apollographql', '@apollo/client': 'apollographql',
	axios: 'axios', lodash: 'lodash', underscore: 'underscore',
	'date-fns': 'datefns', dayjs: 'dayjs', moment: 'momentjs',
	zod: 'zod', joi: 'joi', yup: 'yup',
	trpc: 'trpc', 'socket.io': 'socketdotio',
	storybook: 'storybook', chromatic: 'chromatic',
	babel: 'babel', swc: 'swc', parcel: 'parcel',
	pnpm: 'pnpm', yarn: 'yarn', npm: 'npm',
	electron: 'electron', tauri: 'tauri',
	redux: 'redux', mobx: 'mobx', zustand: 'zustand', jotai: 'jotai',
	'react-query': 'reactquery', '@tanstack/react-query': 'reactquery',
	d3: 'd3dotjs', 'three': 'threedotjs', 'chart.js': 'chartdotjs',
	marked: 'markdown', 'markdown-it': 'markdown',
	sharp: 'sharp', jimp: 'jimp',
	nodemon: 'nodemon', pm2: 'pm2', 'ts-node': 'tsnode',
	'semantic-release': 'semanticrelease', changesets: 'changesets',
	husky: 'commitlint', 'lint-staged': 'eslint',
	sanity: 'sanity', strapi: 'strapi', contentful: 'contentful',
	sentry: 'sentry', '@sentry/node': 'sentry',

	// --- Python Ecosystem ---
	flask: 'flask', django: 'django', fastapi: 'fastapi',
	numpy: 'numpy', pandas: 'pandas', scipy: 'scipy',
	pytorch: 'pytorch', torch: 'pytorch', tensorflow: 'tensorflow',
	keras: 'keras', scikit: 'scikitlearn', 'scikit-learn': 'scikitlearn',
	jupyter: 'jupyter', jupyterlab: 'jupyter', notebook: 'jupyter',
	matplotlib: 'matplotlib', plotly: 'plotly', seaborn: 'seaborn',
	black: 'python', mypy: 'python', ruff: 'ruff', pylint: 'pylint',
	poetry: 'poetry', pipenv: 'pipenv', conda: 'anaconda',
	celery: 'celery', gunicorn: 'gunicorn', uvicorn: 'uvicorn',
	requests: 'python', httpx: 'python', aiohttp: 'aiohttp',
	pydantic: 'pydantic', sqlalchemy: 'sqlalchemy',
	scrapy: 'scrapy', beautifulsoup4: 'python',
	pillow: 'python', opencv: 'opencv', 'opencv-python': 'opencv',
	transformers: 'huggingface', diffusers: 'huggingface',
	openai: 'openai', anthropic: 'anthropic', langchain: 'langchain',
	streamlit: 'streamlit', gradio: 'gradio', dash: 'plotly',

	// --- Rust Ecosystem ---
	'cargo-edit': 'rust', 'cargo-watch': 'rust', 'cargo-expand': 'rust',
	tokio: 'rust', actix: 'rust', axum: 'rust',
	serde: 'rust', clap: 'rust', ripgrep: 'rust', fd: 'rust',
	bat: 'rust', exa: 'rust', eza: 'rust', starship: 'starship',
	'cargo-nextest': 'rust', 'cargo-deny': 'rust',
	'wasm-pack': 'webassembly', 'trunk': 'rust',

	// --- CLI Tools ---
	wget: 'gnu', curl: 'curl', httpie: 'httpie',
	ffmpeg: 'ffmpeg', imagemagick: 'imagemagick',
	htop: 'gnubash', btop: 'gnubash', top: 'gnubash',
	tmux: 'tmux', screen: 'gnubash', zellij: 'rust',
	fzf: 'gnubash', 'the_silver_searcher': 'gnubash', ag: 'gnubash',
	jq: 'json', yq: 'yaml',
	tree: 'gnubash', 'coreutils': 'gnu', findutils: 'gnu',
	gnupg: 'gnuprivacyguard', gpg: 'gnuprivacyguard',
	openssh: 'openssh', openssl: 'openssl',
	rsync: 'rsync', rclone: 'rclone', restic: 'restic',
	p7zip: '7zip', 'xz': 'xz', zstd: 'zstd', pigz: 'gnu',
	nmap: 'nmap', wireshark: 'wireshark',
	lazygit: 'git', 'git-lfs': 'git', 'git-delta': 'git',
	'gh-dash': 'github', 'act': 'githubactions',
	hugo: 'hugo', jekyll: 'jekyll',
	pandoc: 'pandoc', latex: 'latex', texlive: 'latex',
	shellcheck: 'gnubash', shfmt: 'gnubash',
	'pre-commit': 'precommit',

	// --- Apps (brew casks / winget / snap / flatpak) ---
	firefox: 'firefox', 'firefox-developer-edition': 'firefox',
	'google-chrome': 'googlechrome', chromium: 'googlechrome',
	'microsoft-edge': 'microsoftedge', brave: 'brave', arc: 'arc',
	vscodium: 'vscodium',
	slack: 'slack', discord: 'discord', zoom: 'zoom', teams: 'microsoftteams',
	spotify: 'spotify', vlc: 'vlcmediaplayer',
	iterm2: 'iterm2', alacritty: 'alacritty', warp: 'warp', kitty: 'kitty',
	'1password': '1password', bitwarden: 'bitwarden', lastpass: 'lastpass',
	figma: 'figma', sketch: 'sketch', 'adobe-creative-cloud': 'adobe',
	notion: 'notion', obsidian: 'obsidian', logseq: 'logseq',
	raycast: 'raycast', alfred: 'alfred',
	linear: 'linear', height: 'height',
	'jetbrains-toolbox': 'jetbrains', intellij: 'intellijidea',
	webstorm: 'webstorm', pycharm: 'pycharm', goland: 'goland',
	datagrip: 'datagrip', rider: 'rider', clion: 'clion',
	postman: 'postman', insomnia: 'insomnia',
	'docker-desktop': 'docker',
	'obs-studio': 'obsstudio', obs: 'obsstudio',
	gimp: 'gimp', inkscape: 'inkscape', blender: 'blender',
	krita: 'krita', darktable: 'darktable',
	libreoffice: 'libreoffice', 'microsoft-office': 'microsoftoffice',
	telegram: 'telegram', signal: 'signal', whatsapp: 'whatsapp',
	steam: 'steam', 'epic-games': 'epicgames',
	'android-studio': 'androidstudio', xcode: 'xcode',
	transmission: 'transmission', qbittorrent: 'qbittorrent',
	handbrake: 'handbrake', audacity: 'audacity',
	calibre: 'calibre', anki: 'anki',
	mullvad: 'mullvad', protonvpn: 'protonvpn', tailscale: 'tailscale',
	syncthing: 'syncthing', nextcloud: 'nextcloud',
	'raspberry-pi-imager': 'raspberrypi', balena: 'balena',
	utm: 'utm', parallels: 'parallels', virtualbox: 'virtualbox',

	// --- Mac App Store common apps ---
	'Xcode': 'xcode', 'Numbers': 'apple', 'Pages': 'apple', 'Keynote': 'apple',
	'GarageBand': 'apple', 'iMovie': 'apple', 'Final Cut Pro': 'apple',
	'Logic Pro': 'apple', 'TestFlight': 'apple', 'Developer': 'apple',
	'Compressor': 'apple', 'Motion': 'apple',
	'Things 3': 'things', 'Bear': 'bear', 'Craft': 'craft',
	'Fantastical': 'fantastical', 'Cardhop': 'cardhop',
	'Pixelmator Pro': 'pixelmator', 'Affinity Photo': 'affinity',
	'Affinity Designer': 'affinity', 'DaVinci Resolve': 'davinciresolve',
	'1Password 7': '1password', 'Amphetamine': 'apple',
	'Magnet': 'apple', 'Spark': 'readdle', 'Ulysses': 'ulysses',
};

/**
 * Get a Simple Icons CDN URL for a package name.
 * Returns null if no mapping exists.
 */
export function getSimpleIconUrl(name: string, color = '88c0d0'): string | null {
	// Try exact match first, then lowercase
	const slug = ICON_MAP[name] ?? ICON_MAP[name.toLowerCase()];
	if (!slug) return null;
	return `https://cdn.simpleicons.org/${slug}/${color}`;
}

/**
 * Check if a package has a known icon.
 */
export function hasKnownIcon(name: string): boolean {
	return name in ICON_MAP || name.toLowerCase() in ICON_MAP;
}
