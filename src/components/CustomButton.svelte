<script lang="ts">
	import type {ButtonStyle, ButtonType} from "../global/button.type"

	export let onclick: () => void = () => {}
	export let selected: boolean = false
    export let styles: string = "clickable-none"
    export let type: ButtonType = "button"
</script>

<button on:click={onclick} class="rounded-md px-5 h-full {styles}" disabled={selected} type={type}>
    <slot/>
</button>

<style>
	button:disabled {
		background-color: #222;
        color: #444;
        border: #0000 2px solid;
	}
</style>