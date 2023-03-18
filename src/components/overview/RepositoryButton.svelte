<script lang="ts">
	export let action: () => void
	export let onDelete: () => void
	export let selected: boolean = false
</script>

<div class="flex">
    {#if selected}
        <button on:click={action} class="rounded-l-md m-0 px-5 bg-gray-400" disabled>
            <slot/>
        </button>
    {:else}
        <button on:click={action} class="rounded-l-md m-0 px-5 bg-gray-600">
            <slot/>
        </button>
    {/if}
    <button on:click={onDelete} class="rounded-r-md m-0 px-5 bg-red-600">
        X
    </button>
</div>

<style>
	button:active:enabled {
		transform: scaleY(0.95);
	}
</style>