<script lang="ts">
	import {repositories, selectedRepository} from "../../global/repositories"
	import type {Repository} from "../../global/repositories"
	import {saveRepositories} from "../../backend/Calls"
	import CustomButton from "../CustomButton.svelte"

	export let repo: Repository

	$: repos = $repositories

	async function deleteRepository(repository) {
		delete repos[repository]
		try {
			await saveRepositories(repos)
			repositories.set(repos)
		} catch (err) {

		}
	}
</script>

{#if !repo?.branches}
    <div>
        <h1>Select a repo</h1>
    </div>
{:else}
    <h1>{$selectedRepository}</h1>
    <div class="flex flex-row">
        <CustomButton styles="cancel" onclick={() => {deleteRepository($selectedRepository)}}>Delete</CustomButton>
    </div>
    <div>
        <h2 class="underline">Branches:</h2>
        <ul class="pl-3">
            {#each repo.branches as branch}
                {#if branch.is_current}
                    <li class="text-red-700">{branch.name}</li>
                {:else}
                    <li>{branch.name}</li>
                {/if}
            {/each}
        </ul>
    </div>
{/if}