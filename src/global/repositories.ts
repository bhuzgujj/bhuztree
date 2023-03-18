import {writable} from "svelte/store"
import type {Branch} from "../backend/Branch.type"

export type Repository = {
	path: string,
	selected_branch?: string,
	branches: Branch[]
}

export type Repositories = { [key: string]: Repository }

export const selectedRepository = writable<string>("")

export const repositories = writable<Repositories>({})
