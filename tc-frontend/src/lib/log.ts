import { v4 as uuidv4 } from "uuid";

type Layer = {
	code: string;
	column_number: number;
	file_path: string;
	line_number: number;
	name: string;
};

export type Log = {
	id: typeof uuidv4;
	backtrace: { layers: Array<Layer> };
	date: Date;
	file_name: string;
	language: string;
	line_number: number;
	message: string;
	messsage_type: string;
	received_from: string;
	snippet: { [key: number]: string };
	warnings: Array<string>;
};
