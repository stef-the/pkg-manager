<script lang="ts">
	let {
		open = $bindable(false),
		title = 'Are you sure?',
		message = '',
		confirmLabel = 'Confirm',
		cancelLabel = 'Cancel',
		variant = 'danger' as 'danger' | 'warning' | 'default',
		onconfirm
	}: {
		open: boolean;
		title?: string;
		message?: string;
		confirmLabel?: string;
		cancelLabel?: string;
		variant?: 'danger' | 'warning' | 'default';
		onconfirm: () => void;
	} = $props();

	function confirmStyle(): string {
		switch (variant) {
			case 'danger': return 'background-color: var(--error); color: var(--color-nord6);';
			case 'warning': return 'background-color: var(--warning); color: var(--color-nord0);';
			default: return 'background-color: var(--accent); color: var(--bg-primary);';
		}
	}

	function handleConfirm() {
		open = false;
		onconfirm();
	}
</script>

{#if open}
	<button
		class="fixed inset-0 z-40"
		style="background-color: rgba(0, 0, 0, 0.4);"
		onclick={() => (open = false)}
		aria-label="Cancel"
	></button>

	<div
		class="fixed left-1/2 top-1/2 z-50 w-[360px] -translate-x-1/2 -translate-y-1/2 rounded-xl shadow-2xl"
		style="background-color: var(--bg-secondary); border: 1px solid var(--border-subtle);"
	>
		<div class="flex flex-col gap-3 px-5 py-5">
			<h2 class="text-[14px] font-semibold" style="color: var(--text-primary);">{title}</h2>
			{#if message}
				<p class="text-[13px]" style="color: var(--text-secondary);">{message}</p>
			{/if}
			<div class="flex justify-end gap-2 pt-2">
				<button
					class="rounded-lg border px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:bg-[var(--bg-hover)]"
					style="border-color: var(--border); color: var(--text-primary);"
					onclick={() => (open = false)}
				>
					{cancelLabel}
				</button>
				<button
					class="rounded-lg px-4 py-2 text-[13px] font-medium transition-colors duration-100 hover:opacity-90"
					style={confirmStyle()}
					onclick={handleConfirm}
				>
					{confirmLabel}
				</button>
			</div>
		</div>
	</div>
{/if}
