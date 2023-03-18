<script lang="ts">
	import {language, local} from "../../global/localizations"

	$: langueLabel = $local.components.settings.LanguagePicker.title
</script>

<tr class="flex-grow">
    <td class="flex-grow basis-3/4 rounded-md">{langueLabel}</td>
    <td class="flex-grow basis-1/4 rounded-md items-stretch px-2">
        <select bind:value={$language} class="w-full px-1 bg-1 rounded-md">
            <option value="French">Français</option>
            <option value="English">English</option>
        </select>
    </td>
</tr>