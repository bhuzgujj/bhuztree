<script lang="ts">
	import type {ButtonStyle, ButtonType} from "../global/button.type"

	export let onclick: () => void
	export let selected: boolean = false
    export let styles: ButtonStyle = "none"
    export let type: ButtonType = "button"
</script>

<button on:click={onclick} class="rounded-md px-5 h-full {styles}" disabled={selected} type={type}>
    <slot/>
</button>

<style>
	button:disabled {
		background-color: #111;
        border: #0000 2px solid;
	}

	.none {
		background-color: #555;
	}

	.none:hover:enabled  {
		background: #ddd;
		color: #333;
	}

	.none:active:enabled  {
		background: #fff;
	}

	.cancel {
		border: #800 2px solid;
	}

	.cancel:hover:enabled  {
		background: #2005;
	}

	.cancel:active:enabled  {
		background: #400;
	}

	.save {
		border: #080 2px solid;
	}

	.save:hover:enabled  {
		background: #0205;
	}

	.save:active:enabled  {
		background: #040;
	}
</style>