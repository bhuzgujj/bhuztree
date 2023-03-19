import {writable} from "svelte/store"
import type {Repositories} from "../backend/types/Repositories"

export const selectedRepository = writable<string>("")

export const repositories = writable<{ [key: string]: Repositories }>()
