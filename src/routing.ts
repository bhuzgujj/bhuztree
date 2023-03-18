import {writable} from "svelte/store";

export type Pages = "home" | "settings" | "overview";

export const currentPage = writable<Pages>("home");