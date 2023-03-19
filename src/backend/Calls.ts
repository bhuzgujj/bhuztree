import {invoke} from "@tauri-apps/api/tauri"
import {get} from "svelte/store"

import {debug} from "../global/debug"

import {debugLevelToNumber} from "./DebugLevel.util"

import type {Repositories} from "./types/Repositories"
import type {Settings} from "./types/Settings"
import type {DebugLevel} from "./types/DebugLevel"

export type RepositoriesMap = { [key: string]: Repositories }

export async function logging(message: any, debugLevel: DebugLevel = "Warning"): Promise<any> {
	if (debugLevelToNumber(get(debug)) <= debugLevelToNumber(debugLevel)) {
		try {
			await invoke("save_logging", {logs: message.toString(), debugLevel: debugLevel})
			console.log(message)
			return message
		} catch (err: any) {
			console.error(err)
			return err
		}
	}
}

export async function loadSettings(): Promise<Settings> {
	try {
		return await invoke<Settings>("load_settings", {})
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

export async function loadRepositories(): Promise<RepositoriesMap> {
	try {
		return await invoke<RepositoriesMap>("load_repositories", {})
	} catch (err) {
		return Promise.reject(await logging(err, "Debug"))
	}
}

export async function saveRepositories(repos: RepositoriesMap): Promise<Repositories> {
	try {
		return await invoke<Repositories>("save_repositories", {cachedRepositories: repos})
	} catch (err) {
		return Promise.reject(await logging(err, "Error"))
	}
}

export async function getBranch(path: string): Promise<RepositoriesMap> {
	try {
		return await invoke<RepositoriesMap>("get_branch", {paths: [path]})
	} catch (err: any) {
		return Promise.reject(await logging(err, "Warning"))
	}
}

export async function cloneRepo(link: string, path: string, name: string): Promise<RepositoriesMap> {
	try {
		return await invoke<RepositoriesMap>("clone_repo", {link, path, name})
	} catch (err: any) {
		return Promise.reject(await logging(err, "Warning"))
	}
}

export async function addWorktree(name: string, path: string): Promise<RepositoriesMap> {
	try {
		return await invoke<RepositoriesMap>("add_worktree", {path, name})
	} catch (err: any) {
		return Promise.reject(await logging(err, "Warning"))
	}
}
