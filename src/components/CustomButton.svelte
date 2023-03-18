<script lang="ts">
	export let onclick: () => void
	export let selected: boolean = false
</script>

{#if selected}
    <button on:click={onclick} class="rounded-md border-gray-400 px-5 bg-5 h-full" disabled>
        <slot/>
    </button>
{:else}
    <button on:click={onclick} class="rounded-md border-gray-400 px-5 bg-1 h-full">
        <slot/>
    </button>
{/if}

<style>
	button:hover:enabled {
		background: #ddd;
		color: #333;
	}

	button:active:enabled {
		background: #fff;
	}
</style>