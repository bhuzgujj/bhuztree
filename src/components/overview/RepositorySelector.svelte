<script lang="ts">
	import {local} from "../../global/localizations"
	import {repositories, selectedRepository} from "../../global/repositories"
	import CustomButton from "../CustomButton.svelte"

	$: repos = $repositories
	$: title = $local.components.overview.RepositorySelector.title
	$: loadingLabel = $local.common.loading

	function getBranches(branch): void {
		selectedRepository.set(branch)
	}
</script>
<div class="bg-3 rounded-md p-1">
    <h1>{title}</h1>
    <div class="flex flex-row items-stretch gap-2">
        {#each Object.keys(repos) as repository}
            <CustomButton
                    onclick={() => getBranches(repository)}
                    selected={repository === $selectedRepository}
            >
                {repository}
            </CustomButton>
        {/each}
    </div>
</div>