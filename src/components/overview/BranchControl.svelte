<script lang="ts">
	import CustomButton from "../CustomButton.svelte"
	import type {BranchDetails} from "../../backend/types/BranchDetails"
	import {addWorktree} from "../../backend/Calls"

	export let branch: BranchDetails
	export let name: string
	export let path: string
    export let inAction: boolean

    async function add(): Promise<void> {
	    try {
		    inAction = true
		    await addWorktree(name, path)
		} finally {
		    inAction = false
	    }
    }
</script>

<tr class="flex">
    <td class="flex-grow basis-4/5">
        <CustomButton styles="clickable-none w-full text-start">{name}</CustomButton>
    </td>
    <td class="flex-grow basis-1/5 flex justify-end">
        <CustomButton styles="clickable-save w-full" selected={!!branch.worktree_path || inAction} onclick={add}>+</CustomButton>
        <CustomButton styles="clickable-cancel w-full" selected={!branch.worktree_path || inAction}>-</CustomButton>
    </td>
</tr>

