<script lang="ts">
	import {local} from "./global/localizations"
	import {loadRepositories, loadSettings} from "./backend/Calls"

	import PageContainer from "./layouts/PageContainer.svelte"
	import NavBar from "./layouts/NavBar.svelte"

	$: loadingLabel = $local.common.loading

	async function initialLoading() {
		try {
			await loadSettings()
			await loadRepositories()
		} catch (err) {

		}
	}
</script>

<main class="flex flex-col flex-grow p-1">
    {#await initialLoading()}
        <h1>{loadingLabel}</h1>
    {:then _}
        <NavBar/>
        <PageContainer/>
    {/await}
</main>