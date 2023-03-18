<script lang="ts">
	import {local} from "../global/localizations"

	$: homeLabel = $local.pages.home
</script>

<div class="flex-grow basis-1/3 rounded-md p-1 bg-3 mr-1">
    <h1>{homeLabel}</h1>
</div>
<div class="flex-grow basis-2/3 rounded-md p-1 bg-3">
    <h1>lol</h1>
</div>