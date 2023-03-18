import {writable} from "svelte/store"
import type {DebugLevel} from "../backend/types/DebugLevel"

export const debug = writable<DebugLevel>( "Error")