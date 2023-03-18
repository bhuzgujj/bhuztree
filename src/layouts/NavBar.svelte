<script lang="ts">
	import CustomButton from "../components/CustomButton.svelte"
	import {local} from "../global/localizations"
	import {currentPage} from "../routing"

	$: currPage = $currentPage
	$: pagesLabel = $local.pages
</script>

<nav class="flex justify-between items-stretch bg-3 p-1 rounded-md">
    <div>
        <CustomButton onclick={() => currentPage.set("home")} selected={currPage === "home"}>
            {pagesLabel.home}
        </CustomButton>
        <CustomButton onclick={() => currentPage.set("overview")} selected={currPage === "overview"}>
            {pagesLabel.overview}
        </CustomButton>
    </div>
    <div>
        <CustomButton onclick={() => currentPage.set("settings")} selected={currPage === "settings"}>
            {pagesLabel.settings}
        </CustomButton>
    </div>
</nav>