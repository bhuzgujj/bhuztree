<script lang="ts">
	import NavButton from "../components/CustomButton.svelte"
	import {local} from "../global/localizations"
	import {currentPage} from "../routing"

	$: currPage = $currentPage
	$: pagesLabel = $local.pages
</script>

<nav class="flex justify-between items-stretch bg-3 p-1 rounded-md">
    <div>
        <NavButton onclick={() => currentPage.set("home")} selected={currPage === "home"}>
            {pagesLabel.home}
        </NavButton>
        <NavButton onclick={() => currentPage.set("overview")} selected={currPage === "overview"}>
            {pagesLabel.overview}
        </NavButton>
    </div>
    <div>
        <NavButton onclick={() => currentPage.set("settings")} selected={currPage === "settings"}>
            {pagesLabel.settings}
        </NavButton>
    </div>
</nav>