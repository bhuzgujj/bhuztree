<script lang="ts">
	import {repositories, selectedRepository} from "../global/repositories"
	import {local} from "../global/localizations"
	import CustomButton from "../components/CustomButton.svelte"
	import AddDistantRepository from "../components/overview/AddDistantRepository.svelte"
	import AddLocalRepository from "../components/overview/AddLocalRepository.svelte"
	import RepositoryDetails from "../components/overview/RepositoryDetails.svelte"

	$: title = $local.pages.overview
	$: repo = $repositories[$selectedRepository]

	let subMenu: string = "repos"
</script>

<div class="flex flex-col flex-grow">
    <div class="bg-3 rounded-md p-1 mb-1 flex flex-col items-center">
        <div class="mr-2">
            <CustomButton styles="save" onclick={() => {subMenu = "addlocal"}} selected={subMenu === "addlocal"}>
                Add local repos
            </CustomButton>
            <CustomButton styles="save" onclick={() => {subMenu = "adddistant"}} selected={subMenu === "adddistant"}>
                Add distant repos
            </CustomButton>
            <CustomButton styles="info" onclick={() => {subMenu = "repos"}} selected={subMenu === "repos"}>
                Local repos
            </CustomButton>
        </div>
    </div>
    {#if subMenu === "repos"}
        <RepositoryDetails repo={repo}/>
    {:else if subMenu === "addlocal"}
        <AddLocalRepository/>
    {:else if subMenu === "adddistant"}
        <AddDistantRepository/>
    {/if}
</div>