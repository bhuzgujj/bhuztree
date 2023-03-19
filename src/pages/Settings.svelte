<script lang="ts">
	import {logging, saveSettings} from "../backend/Calls"

	import {language, local} from "../global/localizations"
	import {debug} from "../global/debug"
	import {currentPage} from "../routing"

	import SettingsSection from "../components/settings/SettingsSection.svelte"
	import DebugLevelPicker from "../components/settings/DebugLevelPicker.svelte"
	import LangagePicker from "../components/settings/LanguagePicker.svelte"
	import type {Settings} from "../backend/types/Settings"
	import CustomButton from "../components/CustomButton.svelte"

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

	async function save(): Promise<void> {
		try {
			await saveSettings({
				debug_level: $debug,
				language: $language
			})
			previous = {
				language: $language,
				debug_level: $debug
			}
			currentPage.set("home")
		} catch (err: any) {
		}
	}

	function cancel() {
		currentPage.set("home")
	}
</script>

<div class="flex flex-col gap-2 flex-grow bg-3 rounded-md p-1">
    <h1>{settingsLabel}</h1>

    <SettingsSection title={generalSectionLabel}>
        <DebugLevelPicker/>
        <LangagePicker/>
    </SettingsSection>

    <div class="mt-1 flex justify-between items-stretch">
        <CustomButton styles="clickable-save" onclick={save}>{saveLabel}</CustomButton>
        <CustomButton styles="clickable-cancel" onclick={cancel}>{cancelLabel}</CustomButton>
    </div>
</div>