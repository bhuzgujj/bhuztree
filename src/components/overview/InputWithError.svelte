<script lang="ts">
	export let error: string | null
	export let placeholder: string
	export let value: string
</script>

{#if error}
    <p class="text-red-700">{error}</p>
{/if}
<input bind:value={value} placeholder={placeholder}/>