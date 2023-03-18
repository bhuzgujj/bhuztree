<script lang="ts">
	export let error: string | null
	export let placeholder: string
	export let value: string
    export let disabling: boolean = false;
</script>

{#if error}
    <p class="text-red-700">{error}</p>
{/if}
<input bind:value={value} placeholder={placeholder} disabled={disabling}/>

<style>
	input:disabled {
		background-color: #222;
		color: #444;
		border: #0000 2px solid;
	}
</style>