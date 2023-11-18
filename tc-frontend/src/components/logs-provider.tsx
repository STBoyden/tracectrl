/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useState } from "react";
import { Log } from "@/lib/log";
import { useSettings } from "./settings-provider";
import useWebSocket from "react-use-websocket";

const logsKey = "tracectrl-logs" as const;

type Logs = {
	logs: Array<Log>;
};

const _default: Logs = {
	logs: [],
};

type LogsProviderProps = {
	children: React.ReactNode;
	defaultSettings?: Logs;
};

type LogsProviderState = {
	logs: Logs;
	addLog: (log: Log) => void;
};

const initialState: LogsProviderState = {
	logs: _default,
	addLog: () => null,
};

const LogsProviderContext = createContext<LogsProviderState>(initialState);

export function LogsProvider({
	children,
	defaultSettings = _default,
	...props
}: LogsProviderProps) {
	const { settings } = useSettings();
	const [logs, setLogs] = useState<Logs>(
		() =>
			(JSON.parse(
				localStorage.getItem(logsKey) ?? JSON.stringify(""),
			) as Logs) || defaultSettings,
	);

	useWebSocket(`ws://${settings.websocketHost}`, {
		reconnectAttempts: 5,
		onOpen: () => console.log("connection established"),
		onMessage: (event) => {
			const log = JSON.parse(event.data) as Log;

			if (log) {
				console.log(`log recieved: ${log.id}`);
				setLogs({ logs: [...logs.logs, log] });
				localStorage.setItem(logsKey, JSON.stringify(logs));
			}
		},
		shouldReconnect: () => true,
	});

	const value = {
		logs: logs,
		addLog: (log: Log) => {
			const newLogs = { logs: [...logs.logs, log] };

			setLogs(newLogs);
			localStorage.setItem(logsKey, JSON.stringify(newLogs));
		},
	};

	return (
		<LogsProviderContext.Provider {...props} value={value}>
			{children}
		</LogsProviderContext.Provider>
	);
}

export function useLogs() {
	const context = useContext(LogsProviderContext);

	if (context === undefined)
		throw new Error("useLogs must be used within a LogsProvider");

	return context;
}
