<script lang="ts">
    import AddRepositoryForm from "./AddRepositoryForm.svelte";
    import InputWithError from "./InputWithError.svelte";
    import {cloneRepo, getBranch, saveRepositories} from "../../backend/Calls"
    import {repositories} from "../../global/repositories";
    import {local} from "../../global/localizations.js";
    import type {Repositories} from "../../backend/types/Repositories"

    let formPath: string | null;
    let formName: string | null;
    let formLink: string | null;
	let isLoading: boolean = false

    $: nameError = !!$repositories[formName] ?
	    alreadyExist(formName) :
	    null

    $: pathError = Object.values($repositories).some((repo) => repo.path === formPath) ?
	    alreadyExist(formPath) :
	    null

    $: alreadyExist = $local.components.overview.AddLocalRepositoryForm.alreadyExist

    $: isValid = formPath && formName && formLink && !nameError && !pathError && !isLoading;

    async function onSubmit(): Promise<void> {
	    try {
		    isLoading = true
		    await cloneRepo(formLink ?? "", formPath ?? "", formName ?? "")
		    const repos = await getBranch(formName ?? "")
            repositories.set({
			    ...$repositories,
                [formName]: {
	                path: repos[formName].path,
                    worktrees_path: `${formPath}\\${formName}`,
                    branches: repos[formName].branches
                } as Repositories
		    })
            await saveRepositories($repositories)
		    formName= null;
		    formPath= null;
		    formLink= null;
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
    >
        <InputWithError error={null} placeholder={"Repository Link"} bind:value={formLink} bind:disabling={isLoading}/>
    </AddRepositoryForm>
</div>