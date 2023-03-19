<script lang="ts">
	import CustomButton from "../CustomButton.svelte"
	import type {BranchDetails} from "../../backend/types/BranchDetails"

	export let branch: BranchDetails
	export let name: string
</script>

<tr class="flex">
    <td class="flex-grow basis-3/5">
        <CustomButton styles="none w-full">{name}</CustomButton>
    </td>
    <td class="flex-grow basis-2/5 flex justify-end">
        <CustomButton styles="save w-full" selected={!!branch.worktree_path}>+</CustomButton>
        <CustomButton styles="cancel w-full" selected={!branch.worktree_path}>-</CustomButton>
    </td>
</tr>

