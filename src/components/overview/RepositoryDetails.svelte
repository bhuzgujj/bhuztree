<script lang="ts">
    import RepositorySelector from "./RepositorySelector.svelte";
    import RepositoryOverview from "./RepositoryOverview.svelte";
    import {local} from "../../global/localizations.js";
    import type {Repository} from "../../global/repositories.js";

    export let repo: Repository

    $: title = $local.pages.overview
</script>
<RepositorySelector/>
<div class="flex flex-grow mt-1">
    <div class="flex-grow basis-1/2 rounded-md p-1 bg-3">
        <RepositoryOverview bind:repo={repo}/>
    </div>
    <div class="flex-grow basis-1/2 rounded-md p-1 bg-3">
        <h1>{title}</h1>
    </div>
</div>