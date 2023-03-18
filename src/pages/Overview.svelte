<script lang="ts">
	import {repositories, selectedRepository} from "../global/repositories"
	import {local} from "../global/localizations"
	import RepositorySelector from "../components/overview/RepositorySelector.svelte"
	import RepositoryOverview from "../components/overview/RepositoryOverview.svelte"
	import AddLocalRepositoryForm from "../components/overview/AddLocalRepositoryForm.svelte"
	import AddRepositoryButton from "../components/overview/AddRepositoryButtons.svelte"

    $: title = $local.pages.overview
	$: repo = $repositories[$selectedRepository]
</script>

<div class="flex flex-col flex-grow">
    <div class="bg-3 rounded-md p-1 mb-1">
        <div class="mr-2">
            <AddRepositoryButton action={() => {selectedRepository.set("")}}>Add local repos</AddRepositoryButton>
        </div>
    </div>
    <RepositorySelector/>
    <div class="flex flex-grow mt-1">
        <div class="flex-grow basis-1/3 rounded-md p-1 bg-3 mr-1">
            {#if repo}
                <RepositoryOverview bind:repo={repo}/>
            {:else}
                <AddLocalRepositoryForm/>
            {/if}
        </div>
        <div class="flex-grow basis-2/3 rounded-md p-1 bg-3">
            <h1>{title}</h1>
        </div>
    </div>
</div>