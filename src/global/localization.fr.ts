import type {Localization} from "./localization.type"

export const fr: Localization = {
	pages: {
		home: "Accueil",
		settings: "Paramètres",
		overview: "Vue d'ensemble",
	},
	common: {
		cancel: "Annuler",
		save: "Sauvegarder",
		loading: "Chargement...",
	},
	components: {
		settings: {
			LanguagePicker: {
				title: "Langue",
			},
			DebugLevelPicker: {
				title: "Sensibilité des logs",
				debug: "Debug",
				warning: "Avertissement",
				error: "Erreur",
				none: "Aucun",
			},
			SettingsSectionsTitles: {
				general: "Général",
			},
		},
		overview: {
			RepositorySelector: {
				title: "Répertoires",
			},
			AddLocalRepositoryForm: {
				alreadyExist: (value: string) => `'${value}' existe déjà`,
				namePlaceholder: "Nom du répertoire",
				pathPlaceholder: "Chemain vers le répertoire",
				addRepository: "Ajouté le répertoire",
			},
		}
	}
}