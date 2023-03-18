export type Localization = {
	pages: {
		settings: string,
		home: string,
		overview: string,
	},
	common: {
		save: string,
		cancel: string,
		loading: string,
	},
	components: {
		settings: {
			LanguagePicker: {
				title: string,
			},
			DebugLevelPicker: {
				title: string,
				debug: string,
				warning: string,
				error: string,
				none: string,
			},
			SettingsSectionsTitles: {
				general: string,
			},
		},
		overview: {
			RepositorySelector: {
				title: string,
			},
			AddLocalRepositoryForm: {
				alreadyExist: (value: string) => string,
				namePlaceholder: string,
				pathPlaceholder: string,
				addRepository: string,
			},
		},
	}
}