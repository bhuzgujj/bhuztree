<script lang="ts">
	import {logging, saveSettings} from "../backend/Calls"

	import {language, local} from "../global/localizations"
	import {debug} from "../global/debug"
	import {currentPage} from "../routing"

	import SaveControlButton from "../components/SaveControlButton.svelte"
	import CancelControlButton from "../components/CancelControlButton.svelte"
	import SettingsSection from "../components/settings/SettingsSection.svelte"
	import DebugLevelPicker from "../components/settings/DebugLevelPicker.svelte"
	import LangagePicker from "../components/settings/LanguagePicker.svelte"
	import type {Settings} from "../backend/types/Settings"

	$: settingsLabel = $local.pages.settings
	$: generalSectionLabel = $local.components.settings.SettingsSectionsTitles.general
	$: saveLabel = $local.common.save
	$: cancelLabel = $local.common.cancel

	let previous: Settings = {
		language: $language,
		debug_level: $debug
	}

	currentPage.subscribe((_) => {
		language.set(previous.language)
		debug.set(previous.debug_level)
	})

	function save() {
		saveSettings({
			debug_level: $debug,
			language: $language
		})
			.then(() => {
				previous = {
					language: $language,
					debug_level: $debug
				}
				currentPage.set("home")
			})
			.catch(logging)
	}

	function cancel() {
		currentPage.set("home")
	}
</script>

<div class="flex flex-col gap-2 flex-grow bg-3 rounded-md p-1">
    <h1 class="pl-1">{settingsLabel}</h1>

    <SettingsSection title={generalSectionLabel}>
        <DebugLevelPicker/>
        <LangagePicker/>
    </SettingsSection>

    <div class="mt-1 flex justify-between items-stretch">
        <SaveControlButton onclick={save}>{saveLabel}</SaveControlButton>
        <CancelControlButton onclick={cancel}>{cancelLabel}</CancelControlButton>
    </div>
</div>