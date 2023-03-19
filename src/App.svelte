<script lang="ts">
	import {language, local} from "./global/localizations"
	import {loadRepositories, loadSettings} from "./backend/Calls"

	import PageContainer from "./layouts/PageContainer.svelte"
	import NavBar from "./layouts/NavBar.svelte"
	import {debug} from "./global/debug"
	import {repositories} from "./global/repositories"

	$: loadingLabel = $local.common.loading

	async function initialLoading(): Promise<void> {
		try {
			const settings = await loadSettings()
			language.set(settings.language)
			debug.set(settings.debug_level)

            const repos = await loadRepositories()
			repositories.set(repos)
		} catch (err: any) {
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