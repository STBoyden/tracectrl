import { v4 as uuidv4 } from "uuid";

type Layer = {
	line: number;
	code: string;
	file: string;
};

export type Log = {
	id: typeof uuidv4;
	message: string;
	language: string;
	snippet: Layer;
	backtrace: { layers: Array<Layer> };
	warnings: Array<string>;
	received_from: string;
	date: Date;
};
