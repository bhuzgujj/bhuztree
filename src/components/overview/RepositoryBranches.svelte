<script lang="ts">
	import {repositories, selectedRepository} from "../../global/repositories"
	import {saveRepositories} from "../../backend/Calls"
	import CustomButton from "../CustomButton.svelte"
	import BranchControl from "./BranchControl.svelte"
	import type {Repositories} from "../../backend/types/Repositories"

	export let repo: Repositories
    let inAction: boolean = false

	$: repos = $repositories

	async function remove(repository): Promise<void> {
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
    <CustomButton styles="clickable-cancel" onclick={() => {remove($selectedRepository)}}>Remove</CustomButton>
    <CustomButton styles="clickable-cancel" onclick={() => {remove($selectedRepository)}}>Delete</CustomButton>
</div>
<div>
    <h2 class="underline">Branches:</h2>
    <table class="flex flex-col bg-5 rounded-md">
        {#each Object.keys(repo.branches) as name}
            <BranchControl branch={repo.branches[name]} branch_name={name} repo={repo} bind:inAction={inAction}/>
        {/each}
    </table>
</div>