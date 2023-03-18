<script lang="ts">
	import {currentPage} from "../routing"
	import Overview from "../pages/Overview.svelte"
	import Settings from "../pages/Settings.svelte"
	import Home from "../pages/Home.svelte"

	$: currPage = $currentPage
</script>

<div class="my-1 flex justify-between items-stretch flex-grow">
    {#if currPage === 'home'}
        <Home/>
    {:else if currPage === 'settings'}
        <Settings/>
    {:else if currPage === 'overview'}
        <Overview/>
    {/if}
</div>