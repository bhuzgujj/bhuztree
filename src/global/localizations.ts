import {writable} from "svelte/store"
import type {Readable} from "svelte/store"

import {fr} from "./localization.fr"
import {en} from "./localization.en"

import type {Localization} from "./localization.type"
import type {Language} from "../backend/types/Language"

export const language = writable<Language>("English")
const localization = writable<Localization>(en)

let currentLocalization: Localization = en

language.subscribe((language: Language) => {
	switch (language) {
		case "French":
			currentLocalization = fr
			localization.set(fr)
			break
		default:
			currentLocalization = en
			localization.set(en)
			break
	}
})

export const local: Readable<Localization> = localization as Readable<Localization>
