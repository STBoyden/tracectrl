import { v4 as uuidv4 } from "uuid";

export type Log = {
	id: typeof uuidv4;
	message: string;
	language: string;
	snippet: { line: number; code: string };
	backtrace: Array<{ line: number; code: string }>;
	warnings: Array<string>;
	date: Date;
};
