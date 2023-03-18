import type {DebugLevel} from "./types/DebugLevel"

export function debugLevelToNumber(level: DebugLevel): number {
	switch (level) {
		case "Debug":
			return 0;
		case "Warning":
			return 1;
		case "Error":
			return 2;
		default:
			return 3;
	}
}