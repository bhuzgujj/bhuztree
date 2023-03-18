<script lang="ts">
	import {repositories, selectedRepository} from "../../global/repositories"
    import type {Repository} from "../../global/repositories"
	import CancelControlButton from "../CancelControlButton.svelte"
	import {saveRepositories} from "../../backend/Calls"

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

<h1>{$selectedRepository}</h1>
<div class="flex flex-row">
    <CancelControlButton onclick={() => {deleteRepository($selectedRepository)}}>Delete</CancelControlButton>
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