<script lang="ts">
	import {repositories, selectedRepository} from "../../global/repositories"
	import {saveRepositories} from "../../backend/Calls"
	import CustomButton from "../CustomButton.svelte"
	import BranchControl from "./BranchControl.svelte"
	import type {Repositories} from "../../backend/types/Repositories"

	export let repo: Repositories
    let inAction = false

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
        <table class="flex flex-col bg-5 rounded-md">
            {#each Object.keys(repo.branches) as name}
                <BranchControl branch={repo.branches[name]} name={name} path={repo.path} bind:inAction={inAction}/>
            {/each}
        </table>
    </div>
{/if}