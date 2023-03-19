<script lang="ts">
    import AddRepositoryForm from "./AddRepositoryForm.svelte";
    import {getBranch} from "../../backend/Calls";
    import {repositories} from "../../global/repositories";
    import {local} from "../../global/localizations.js";
    import type {Repositories} from "../../backend/types/Repositories"

    let formPath: string | null;
    let formName: string | null;
    let isLoading: boolean = false

	$: nameError = !!$repositories[formName] ?
	    alreadyExist(formName) :
	    null

    $: pathError = Object.values($repositories).some((repo) => repo.path === formPath) ?
	    alreadyExist(formPath) :
	    null

    $: alreadyExist = $local.components.overview.AddLocalRepositoryForm.alreadyExist

    $: isValid = formPath && formName && !nameError && !pathError;

    async function onSubmit(): Promise<void> {
        try {
	        isLoading = true
	        const repos = await getBranch(formPath ?? "")
	        console.log(repos)
	        repositories.set({
		        ...$repositories,
		        [formName]: {
			        path: formPath,
			        branches: repos[formPath]
		        } as Repositories
	        })
	        formName= null;
	        formPath= null;
        }  catch (err: any) {
	        pathError = `No repository at ${formPath}`
        } finally {
	        isLoading = false
        }
    }
</script>

<div class="flex flex-col rounded-md p-1 bg-3">
    <AddRepositoryForm
            title="Add local repository"
            bind:formName={formName}
            bind:nameError={nameError}
            bind:formPath={formPath}
            bind:pathError={pathError}
            bind:isValid
            bind:isLoading={isLoading}
            onsubmit={onSubmit}
    />
</div>