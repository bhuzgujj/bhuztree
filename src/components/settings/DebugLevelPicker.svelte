<script lang="ts">
	import {debug} from "../../global/debug"
	import {local} from "../../global/localizations"

	$: debugLevelLabels = $local.components.settings.DebugLevelPicker
</script>

<tr class="flex-grow">
    <td class="flex-grow basis-3/4 rounded-md">{debugLevelLabels.title}</td>
    <td class="flex-grow basis-1/4 rounded-md items-stretch px-2">
        <select bind:value={$debug} class="w-full px-1 bg-1 rounded-md">
            <option value={"Debug"}>{debugLevelLabels.debug}</option>
            <option value={"Warning"}>{debugLevelLabels.warning}</option>
            <option value={"Error"}>{debugLevelLabels.error}</option>
            <option value={"None"}>{debugLevelLabels.none}</option>
        </select>
    </td>
</tr>