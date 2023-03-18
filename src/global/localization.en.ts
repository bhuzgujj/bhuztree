import type {Localization} from "./localization.type"

export const en: Localization = {
	pages: {
		home: "Home",
		settings: "Settings",
		overview: "Overview",
	},
	common: {
		cancel: "Cancel",
		save: "Save",
		loading: "Loading...",
	},
	components: {
		settings: {
			LanguagePicker: {
				title: "Language",
			},
			DebugLevelPicker: {
				title: "Logging Sensitivity",
				debug: "Debug",
				warning: "Warning",
				error: "Error",
				none: "None",
			},
			SettingsSectionsTitles: {
				general: "General",
			},
		},
		overview: {
			RepositorySelector: {
				title: "Repositories",
			},
			AddLocalRepositoryForm: {
				alreadyExist: (value: string) => `'${value}' already exist`,
				namePlaceholder: "Repository Name",
				pathPlaceholder: "Repository Path",
				addRepository: "Add repository",
			},
		},
	}
}