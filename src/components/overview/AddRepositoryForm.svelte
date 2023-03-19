<script lang="ts">
	import {repositories} from "../../global/repositories"
	import {local} from "../../global/localizations"
	import {saveRepositories} from "../../backend/Calls"
	import {open} from '@tauri-apps/api/dialog'

	import InputWithError from "./InputWithError.svelte"
	import CustomButton from "../CustomButton.svelte"

	export let title: string
	export let formName: string | null
	export let formPath: string | null
	export let nameError: string | null
	export let pathError: string | null
	export let isValid: boolean
	export let isLoading: boolean = false

	export let onsubmit: () => Promise<void>

	$: namePlaceholder = $local.components.overview.AddLocalRepositoryForm.namePlaceholder
	$: pathPlaceholder = $local.components.overview.AddLocalRepositoryForm.pathPlaceholder
	$: addRepository = $local.components.overview.AddLocalRepositoryForm.addRepository

	function addRepo(): void {
		onsubmit().then((_) => saveRepositories($repositories))
	}

	async function getFolder(): Promise<void> {
		const folder = await open({
			directory: true,
			multiple: false
		})

		if (typeof folder == "string")
			formPath = folder
	}
</script>

<div class="flex flex-row justify-center">
    <div class="flex flex-col m-1 p-1 bg-4 rounded-md basis-2/3">
        <h1>{title}</h1>
        <form on:submit|preventDefault={addRepo}>
            <InputWithError error={nameError} placeholder={namePlaceholder} bind:value={formName} bind:disabling={isLoading}/>
            <div class="flex flex-row gap-0.5 items-center justify-between">
                <div class="flex-grow flex flex-col items-stretch">
                    <InputWithError error={pathError} placeholder={pathPlaceholder} bind:value={formPath} bind:disabling={isLoading}/>
                </div>
                <div class="flex-grow-0 p-1 flex flex-col items-stretch">
                    <CustomButton onclick={getFolder}>
                        <img src="https://api.iconify.design/mdi:folder.svg" alt="open"/>
                    </CustomButton>
                </div>
            </div>
            <slot/>
            <CustomButton type="submit" styles="clickable-save" selected={!isValid}>
                {addRepository}
            </CustomButton>
        </form>
    </div>
</div>
