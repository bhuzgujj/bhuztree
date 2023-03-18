<script lang="ts">
	import {repositories, selectedRepository} from "../global/repositories"
	import {local} from "../global/localizations"
	import RepositorySelector from "../components/overview/RepositorySelector.svelte"
	import RepositoryOverview from "../components/overview/RepositoryOverview.svelte"
	import AddRepositoryForm from "../components/overview/AddRepositoryForm.svelte"
	import CustomButton from "../components/CustomButton.svelte"
	import InputWithError from "../components/overview/InputWithError.svelte"
	import AddDistantRepository from "../components/overview/AddDistantRepository.svelte"
	import AddLocalRepository from "../components/overview/AddLocalRepository.svelte"

	$: title = $local.pages.overview
	$: repo = $repositories[$selectedRepository]
    let overviewSubMenu = "addlocal"
</script>

<div class="flex flex-col flex-grow">
    <div class="bg-3 rounded-md p-1 mb-1 flex flex-col items-center">
        <div class="mr-2">
            <CustomButton
                    styles="save"
                    onclick={() => {overviewSubMenu = "addlocal"}}
                    selected={overviewSubMenu === "addlocal"}
            >
                Add local repos
            </CustomButton>
            <CustomButton
                    styles="save"
                    onclick={() => {overviewSubMenu = "adddistant"}}
                    selected={overviewSubMenu === "adddistant"}
            >
                Add distant repos
            </CustomButton>
            <CustomButton
                    styles="info"
                    onclick={() => {overviewSubMenu = "repos"}}
                    selected={overviewSubMenu === "repos"}
            >
                Local repos
            </CustomButton>
        </div>
    </div>
    {#if overviewSubMenu === "repos"}
        <RepositorySelector/>
        <div class="flex flex-grow mt-1">
            <div class="flex-grow basis-1/3 rounded-md p-1 bg-3 mr-1">
                <RepositoryOverview bind:repo={repo}/>
            </div>
            <div class="flex-grow basis-2/3 rounded-md p-1 bg-3">
                <h1>{title}</h1>
            </div>
        </div>
    {:else if overviewSubMenu === "addlocal"}
        <AddLocalRepository/>
    {:else if overviewSubMenu === "adddistant"}
        <AddDistantRepository/>
    {/if}
</div>