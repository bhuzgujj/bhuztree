<script lang="ts">
	export let action: () => void
</script>

<div>
    <button on:click={action} class="rounded-md border-gray-400 px-5 bg-green-500">
        <slot/>
    </button>
</div>