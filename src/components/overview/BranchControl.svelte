<script lang="ts">
	import CustomButton from "../CustomButton.svelte"
	import {addWorktree} from "../../backend/Calls"
	import type {Repositories} from "../../backend/types/Repositories"

	export let branch_name: string
	export let repo: Repositories
    export let inAction: boolean

    async function add(): Promise<void> {
	    try {
		    inAction = true
		    await addWorktree(branch_name, repo.path, repo)
		} finally {
		    inAction = false
	    }
    }
</script>

<tr class="flex">
    <td class="flex-grow basis-4/5">
        <CustomButton styles="clickable-none w-full text-start">{branch_name}</CustomButton>
    </td>
    <td class="flex-grow basis-1/5 flex justify-end">
        <CustomButton styles="clickable-save w-full" selected={!repo.branches[branch_name] || inAction} onclick={add}>+</CustomButton>
        <CustomButton styles="clickable-cancel w-full" selected={!repo.branches[branch_name] || inAction}>-</CustomButton>
    </td>
</tr>

