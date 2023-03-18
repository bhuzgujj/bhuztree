<script lang="ts">
	import {repositories} from "../../global/repositories"
	import {local} from "../../global/localizations"
	import {getBranch, saveRepositories} from "../../backend/Calls"
	import { open } from '@tauri-apps/api/dialog';

	import InputWithError from "./InputWithError.svelte"
	import CustomButton from "../CustomButton.svelte"

	$: alreadyExist = $local.components.overview.AddLocalRepositoryForm.alreadyExist
	$: namePlaceholder = $local.components.overview.AddLocalRepositoryForm.namePlaceholder
	$: pathPlaceholder = $local.components.overview.AddLocalRepositoryForm.pathPlaceholder
	$: addRepository = $local.components.overview.AddLocalRepositoryForm.addRepository

	let formName: string | null
	let formPath: string | null
	let nameError: string | null
	let pathError: string | null

	$: {
		nameError = !!$repositories[formName] ?
			alreadyExist(formName) :
			null
		pathError = Object.values($repositories).some((repo) => repo.path === formPath) ?
			alreadyExist(formPath) :
			null
	}

	function isInputInvalid(name, path): boolean {
		return isNameInvalid(name) || isPathInvalid(path)
	}

	function isNameInvalid(name) {
		return !!$repositories[name] || !name
	}

	function isPathInvalid(path) {
		return !path || Object.values($repositories).some((repo) => repo.path === path)
	}

	function addRepo(): void {
		if (isInputInvalid(formName, formPath)) {
			return
		}
		getBranch({path: formPath, branches: []}, $repositories, formName)
			.then((_) => {
				formName = null
				formPath = null
				saveRepositories($repositories)
			})
			.catch(() => pathError = `No repository at ${formPath}`)
	}

	async function getFolder() {
		const folder = await open({
			directory: true,
			multiple: false
		});

		if (typeof folder == "string")
			formPath = folder
    }
</script>

<h1>Add local repository</h1>
<form on:submit|preventDefault={addRepo}>
    <InputWithError error={nameError} placeholder={namePlaceholder} bind:value={formName}/>
    <div class="flex flex-row gap-0.5 items-center justify-between">
        <InputWithError error={pathError} placeholder={pathPlaceholder} bind:value={formPath}/>
        <CustomButton onclick={getFolder}>
            <img src="https://api.iconify.design/mdi:folder.svg" alt="open"/>
        </CustomButton>
    </div>
    <CustomButton type="submit" styles="save" selected={isInputInvalid(formName, formPath)}>
        {addRepository}
    </CustomButton>
</form>