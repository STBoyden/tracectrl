import { DataTable } from "@/components/ui/data-table";
import { useLogs } from "@/components/logs-provider";
import dayjs from "dayjs";

const columns = [
	{ accessorKey: "index", header: "Index" },
	{ accessorKey: "warnings", header: "Warnings" },
	{ accessorKey: "message", header: "Message" },
	{ accessorKey: "language", header: "Language" },
	{ accessorKey: "date", header: "Date" },
];

export function LogsArea() {
	const { logs } = useLogs();

	const _logs = logs.logs.map((log, index) => {
		return {
			index: index,
			...log,
			warnings: log.warnings.length,
			date: dayjs(log.date).format("YYYY-MM-DD HH:mm:ss"),
		};
	});

	return (
		<DataTable
			columns={columns}
			data={_logs}
			caption="New logs will appear here."
		></DataTable>
	);
}
