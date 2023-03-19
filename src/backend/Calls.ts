import {invoke} from "@tauri-apps/api/tauri"
import {get} from "svelte/store"

import {repositories} from "../global/repositories"
import {language} from "../global/localizations"
import {debug} from "../global/debug"

import {debugLevelToNumber} from "./DebugLevel.util"


import type {Repositories} from "./types/Repositories"
import type {Settings} from "./types/Settings"
import type {DebugLevel} from "./types/DebugLevel"
import type {BranchDetails} from "./types/BranchDetails"

export async function logging(message: any, debugLevel: DebugLevel = "Warning"): Promise<any> {
	if (debugLevelToNumber(get(debug)) <= debugLevelToNumber(debugLevel)) {
		try {
			await invoke("save_logging", {logs: message.toString(), debugLevel: debugLevel})
			console.log(message)
		} catch (err) {
			console.error(err)
			return Promise.reject(err)
		}
	}
	return message
}

export async function loadSettings(): Promise<Settings | any> {
	try {
		const settings = await invoke<Settings>("load_settings", {})
		language.set(settings.language)
		debug.set(settings.debug_level)
		return settings
	} catch (err) {
		return Promise.reject(await logging(err, "Debug"))
	}
}

export async function saveSettings(settings: Settings): Promise<any> {
	try {
		return await invoke<any>("save_settings", {settings})
	} catch (err) {
		return Promise.reject(await logging(err, "Error"))
	}
}

export async function loadRepositories(): Promise<Repositories | any> {
	try {
		const repos = await invoke<{ [key: string]: Repositories }>("load_repositories", {})
		repositories.set(repos)
		return repos
	} catch (err) {
		return Promise.reject(await logging(err, "Debug"))
	}
}

export async function saveRepositories(repos:  { [key: string]: Repositories }): Promise<Repositories | any> {
	try {
		return await invoke<Repositories>("save_repositories", {cachedRepositories: repos})
	} catch (err) {
		return Promise.reject(await logging(err, "Error"))
	}
}

export async function getBranch(path: string, repos: { [key: string]: Repositories }, name: string): Promise<{ [key: string]: Repositories[] } | any> {
	try {
		const branches = await invoke<Repositories>("get_branch", {paths: [path]})
		repositories.set({
			...repos,
			[name]: {
				path: path,
				branches: branches[path]
			} as Repositories
		})
		return branches
	} catch (err: any) {
		return Promise.reject(await logging(err, "Warning"))
	}
}

export async function cloneRepo(link: string, path: string, repos: Repositories, name: string) {
	try {
		return await invoke<{ [key: string]: BranchDetails[] }>("clone_repo", {link, path, name})
	} catch (err: any) {
		return Promise.reject(await logging(err, "Warning"))
	}
}