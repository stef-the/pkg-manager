<script lang="ts" module>
	export type IconName =
		| 'dashboard'
		| 'installed'
		| 'outdated'
		| 'search'
		| 'cleanup'
		| 'debloat'
		| 'export'
		| 'logs'
		| 'terminal'
		| 'refresh'
		| 'settings'
		| 'close'
		| 'check'
		| 'warning'
		| 'error'
		| 'info'
		| 'all'
		| 'brew'
		| 'npm'
		| 'winget'
		| 'mas'
		| 'pip'
		| 'cargo'
		| 'apt'
		| 'flatpak'
		| 'snap'
		| 'browse'
		| 'managers';
</script>

<script lang="ts">
	let { name, size = 16 }: { name: IconName; size?: number } = $props();
</script>

<svg
	width={size}
	height={size}
	viewBox="0 0 16 16"
	fill="none"
	stroke="currentColor"
	stroke-width="1.5"
	stroke-linecap="round"
	stroke-linejoin="round"
>
	{#if name === 'dashboard'}
		<!-- Grid / dashboard -->
		<rect x="2" y="2" width="5" height="5" rx="1" />
		<rect x="9" y="2" width="5" height="5" rx="1" />
		<rect x="2" y="9" width="5" height="5" rx="1" />
		<rect x="9" y="9" width="5" height="5" rx="1" />
	{:else if name === 'installed'}
		<!-- Package / box -->
		<path d="M2 5l6-3 6 3v6l-6 3-6-3V5z" />
		<path d="M2 5l6 3 6-3" />
		<path d="M8 8v6" />
	{:else if name === 'outdated'}
		<!-- Arrow up circle -->
		<circle cx="8" cy="8" r="6" />
		<path d="M8 11V5" />
		<path d="M5.5 7.5L8 5l2.5 2.5" />
	{:else if name === 'search'}
		<!-- Magnifying glass -->
		<circle cx="7" cy="7" r="4" />
		<path d="M10 10l3.5 3.5" />
	{:else if name === 'cleanup'}
		<!-- Broom / sparkle -->
		<path d="M4 12l3-3" />
		<path d="M7 9l2-6" />
		<path d="M12 3l-1 1" />
		<path d="M11 6l1-1" />
		<path d="M3 10c0 2 2 4 4 4" />
		<circle cx="12" cy="4" r="0.5" fill="currentColor" stroke="none" />
	{:else if name === 'debloat'}
		<!-- Slider / chart -->
		<path d="M2 4h12" />
		<path d="M2 8h12" />
		<path d="M2 12h12" />
		<circle cx="5" cy="4" r="1.5" fill="currentColor" />
		<circle cx="10" cy="8" r="1.5" fill="currentColor" />
		<circle cx="7" cy="12" r="1.5" fill="currentColor" />
	{:else if name === 'export'}
		<!-- Arrow up from box -->
		<path d="M4 7v5a1 1 0 001 1h6a1 1 0 001-1V7" />
		<path d="M8 10V3" />
		<path d="M5.5 5.5L8 3l2.5 2.5" />
	{:else if name === 'logs'}
		<!-- File / document -->
		<path d="M4 2h5l3 3v9H4V2z" />
		<path d="M9 2v3h3" />
		<path d="M6 8h4" />
		<path d="M6 10.5h3" />
	{:else if name === 'terminal'}
		<!-- Terminal / console -->
		<rect x="2" y="3" width="12" height="10" rx="1.5" />
		<path d="M5 7l2 1.5L5 10" />
		<path d="M9 10h2.5" />
	{:else if name === 'refresh'}
		<!-- Circular arrows -->
		<path d="M2.5 8a5.5 5.5 0 019.5-3.5" />
		<path d="M13.5 8a5.5 5.5 0 01-9.5 3.5" />
		<path d="M12 2v3h-3" />
		<path d="M4 14v-3h3" />
	{:else if name === 'settings'}
		<!-- Gear -->
		<circle cx="8" cy="8" r="2" />
		<path d="M8 2v1.5M8 12.5V14M2 8h1.5M12.5 8H14M3.75 3.75l1.06 1.06M11.19 11.19l1.06 1.06M3.75 12.25l1.06-1.06M11.19 4.81l1.06-1.06" />
	{:else if name === 'close'}
		<path d="M4 4l8 8M12 4l-8 8" />
	{:else if name === 'check'}
		<path d="M3 8.5l3.5 3.5L13 5" />
	{:else if name === 'all'}
		<!-- Circle with dot -->
		<circle cx="8" cy="8" r="5" />
		<circle cx="8" cy="8" r="1.5" fill="currentColor" />
	{:else if name === 'brew'}
		<!-- Homebrew — stylized bottle -->
		<path d="M6 3h4v2H6z" fill="currentColor" stroke="none" />
		<path d="M5 5h6v1H5z" fill="currentColor" stroke="none" opacity="0.6" />
		<rect x="5.5" y="6" width="5" height="7" rx="1" fill="currentColor" stroke="none" />
	{:else if name === 'npm'}
		<!-- npm — the classic red box n -->
		<rect x="2" y="3" width="12" height="10" rx="0.5" fill="currentColor" stroke="none" />
		<path d="M4 5h2v6H4z" fill="var(--bg-secondary)" stroke="none" />
		<path d="M6 5h2v4H6z" fill="var(--bg-secondary)" stroke="none" />
		<path d="M10 5h2v6h-2z" fill="var(--bg-secondary)" stroke="none" />
	{:else if name === 'winget'}
		<!-- Windows — four panes -->
		<rect x="2" y="2" width="5.5" height="5.5" rx="0.5" fill="currentColor" stroke="none" />
		<rect x="8.5" y="2" width="5.5" height="5.5" rx="0.5" fill="currentColor" stroke="none" />
		<rect x="2" y="8.5" width="5.5" height="5.5" rx="0.5" fill="currentColor" stroke="none" />
		<rect x="8.5" y="8.5" width="5.5" height="5.5" rx="0.5" fill="currentColor" stroke="none" />
	{:else if name === 'mas'}
		<!-- Apple / App Store -->
		<circle cx="8" cy="8" r="6" fill="currentColor" stroke="none" />
		<text x="8" y="11" text-anchor="middle" font-size="8" font-weight="bold" fill="var(--bg-secondary)" font-family="system-ui">A</text>
	{:else if name === 'pip'}
		<!-- Python -->
		<path d="M8 2C5 2 5 3.5 5 3.5v2h3v.5H4S2 5.7 2 8.3 3.8 11 3.8 11H5v-2s-.1-1.8 1.8-1.8h3c1 0 1.8-.8 1.8-1.8V3.5S11.8 2 8 2z" fill="currentColor" stroke="none" />
		<circle cx="6.2" cy="3.8" r=".7" fill="var(--bg-secondary)" stroke="none" />
		<path d="M8 14c3 0 3-1.5 3-1.5v-2H8v-.5h4s2 .3 2-2.3S12.2 5 12.2 5H11v2s.1 1.8-1.8 1.8h-3c-1 0-1.8.8-1.8 1.8v1.9S4.2 14 8 14z" fill="currentColor" stroke="none" opacity="0.6" />
	{:else if name === 'cargo'}
		<!-- Rust gear -->
		<circle cx="8" cy="8" r="5" />
		<circle cx="8" cy="8" r="2" />
		<path d="M8 2v1M8 13v1M2 8h1M13 8h1M3.8 3.8l.7.7M11.5 11.5l.7.7M3.8 12.2l.7-.7M11.5 4.5l.7-.7" stroke-width="1.5" />
	{:else if name === 'apt'}
		<!-- Debian/Ubuntu circle -->
		<circle cx="8" cy="8" r="6" fill="currentColor" stroke="none" />
		<path d="M6 6c1-1.5 3-1.5 4 0s.5 3-1 4-3 .5-4-.5" fill="none" stroke="var(--bg-secondary)" stroke-width="1.5" stroke-linecap="round" />
	{:else if name === 'flatpak'}
		<!-- Flatpak — package with ribbon -->
		<rect x="3" y="4" width="10" height="8" rx="1.5" fill="currentColor" stroke="none" />
		<path d="M3 7h10" stroke="var(--bg-secondary)" stroke-width="1" />
		<path d="M8 4v8" stroke="var(--bg-secondary)" stroke-width="1" />
	{:else if name === 'snap'}
		<!-- Snap — S-like -->
		<circle cx="8" cy="8" r="6" fill="currentColor" stroke="none" />
		<path d="M5.5 6.5l5-1v4.5l-5 1z" fill="var(--bg-secondary)" stroke="none" />
	{:else if name === 'browse'}
		<!-- Compass / browse -->
		<circle cx="8" cy="8" r="6" />
		<path d="M6 10l1-4 4-1-1 4z" fill="currentColor" stroke="none" />
	{:else if name === 'managers'}
		<!-- Layers / stack -->
		<path d="M2 8l6-4 6 4-6 4z" />
		<path d="M2 11l6 4 6-4" />
	{/if}
</svg>
