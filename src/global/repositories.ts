import {writable} from "svelte/store"
import type {RepositoriesMap} from "../backend/Calls"

export const selectedRepository = writable<string>("")

export const repositories = writable<RepositoriesMap>()
