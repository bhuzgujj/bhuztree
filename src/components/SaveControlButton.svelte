<script lang="ts">
	export let onclick: () => void
</script>

<div>
    <button on:click={onclick} class="rounded-md border-gray-400 px-5 bg-green-700">
        <slot/>
    </button>
</div>