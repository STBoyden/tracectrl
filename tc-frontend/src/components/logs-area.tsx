/* eslint-disable no-mixed-spaces-and-tabs */
import { DataTable } from "@/components/ui/data-table";
import { useLogs } from "@/components/logs-provider";
import dayjs from "dayjs";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
	SheetTrigger,
} from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import {
	HoverCard,
	HoverCardContent,
	HoverCardTrigger,
} from "@/components/ui/hover-card";

import SyntaxHighlighter from "react-syntax-highlighter";
import {
	a11yLight,
	a11yDark,
} from "react-syntax-highlighter/dist/esm/styles/hljs";
import { useTheme } from "@/components/theme-provider";
import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@/components/ui/accordion";

const columns = [
	{ accessorKey: "index", header: "Index" },
	{ accessorKey: "warnings", header: "Warnings" },
	{ accessorKey: "message", header: "Message" },
	{ accessorKey: "language", header: "Language" },
	{ accessorKey: "date", header: "Date" },
	{ accessorKey: "received_from", header: "Sender" },
	{ accessorKey: "sheet", header: "" },
];

export function LogsArea() {
	const { logs } = useLogs();
	const { variant } = useTheme();

	const _logs = logs.logs.map((log, index) => {
		return {
			index: index,
			...log,
			warnings:
				log.warnings.length > 0 ? (
					<HoverCard>
						<HoverCardTrigger>{log.warnings.length}</HoverCardTrigger>
						<HoverCardContent className="w-[400px]">
							{log.warnings.map((warning, index) => {
								return (
									<p>
										<span className="font-mono">{index + 1}:</span> {warning}
									</p>
								);
							})}
						</HoverCardContent>
					</HoverCard>
				) : (
					0
				),
			date: dayjs(log.date).format("YYYY-MM-DD HH:mm:ss"),
			sheet: (
				<Sheet>
					<SheetTrigger>
						<Button>Open</Button>
					</SheetTrigger>
					<SheetContent side="bottom" className="h-[500px]">
						<SheetHeader>
							<SheetTitle>Log information</SheetTitle>
							<SheetDescription className="font-mono">
								<p>
									<span className="font-sans">Position: </span>
									{log.snippet.file}:{log.snippet.line}
								</p>
								<p>
									<span className="font-sans">Message: </span> {log.message}
								</p>
								<p>
									<span className="font-sans">Received at: </span>
									{dayjs(log.date).format("YYYY-MM-DD HH:mm:ss")}
								</p>
								<p>
									<span className="font-sans">Received from: </span>
									{log.received_from}
								</p>
							</SheetDescription>
						</SheetHeader>
						<div className="w-full grid md:grid-cols-2 md:grid-rows-1 grid-cols-1 grid-rows-2">
							<div className="w-full resize-y border-r pr-2">
								<Accordion type="single" className="w-full">
									<AccordionItem value="stacktrace">
										<AccordionTrigger>
											Stacktrace ({log.backtrace.layers.length ?? 0} layer(s))
										</AccordionTrigger>
										<AccordionContent className="pl-4">
											<Accordion type="multiple" className="w-full">
												{log.backtrace.layers.map(
													({ line, code, file }, index) => {
														return (
															<AccordionItem value={`layer-${index}`}>
																<AccordionTrigger>
																	Layer {index + 1}
																</AccordionTrigger>
																<AccordionContent className="font-mono pl-4">
																	<p>
																		<span className="font-sans">File: </span>
																		{file}:{line}
																	</p>
																	<p>
																		<span className="font-sans">Code: </span>
																		{code}
																	</p>
																</AccordionContent>
															</AccordionItem>
														);
													},
												)}
											</Accordion>
										</AccordionContent>
									</AccordionItem>
									<AccordionItem value="warnings">
										<AccordionTrigger>Warnings</AccordionTrigger>
										<AccordionContent>
											{log.warnings.map((warning, index) => {
												return (
													<div className="font-mono">
														<p>
															<span className="font-sans">{index + 1}: </span>
															{warning}
														</p>
													</div>
												);
											})}
										</AccordionContent>
									</AccordionItem>
								</Accordion>
							</div>
							<div className="w-full resize-y border-l pl-2">
								<SyntaxHighlighter
									children={log.snippet.code}
									language={log.language.toLowerCase()}
									style={variant == "dark" ? a11yDark : a11yLight}
									lineNumberStyle={(line) => {
										return (
											line == log.snippet.line
												? {
														"font-weight": "bold",
														"font-style": "italic",
														"text-decoration-line": "underline",
												  }
												: {}
										) as React.CSSProperties;
									}}
									showLineNumbers
								/>
							</div>
						</div>
					</SheetContent>
				</Sheet>
			),
		};
	});

	return (
		<div className="mx-4 mt-2">
			<DataTable
				columns={columns}
				data={_logs}
				caption="New logs will appear here."
			/>
		</div>
	);
}
