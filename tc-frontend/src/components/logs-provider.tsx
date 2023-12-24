/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useState } from "react";
import { Log } from "@/lib/log";
import { useSettings } from "./settings-provider";
import useWebSocket from "react-use-websocket";

const logsKey = "tracectrl-logs" as const;

type ClientSettings = {
	logs: Array<Log>;
	clientId?: number;
};

const _default: ClientSettings = {
	logs: [],
	clientId: undefined,
};

type LogsProviderProps = {
	children: React.ReactNode;
	defaultSettings?: ClientSettings;
};

type LogsProviderState = {
	client: ClientSettings;
};

const initialState: LogsProviderState = {
	client: _default,
};

const LogsProviderContext = createContext<LogsProviderState>(initialState);

export function LogsProvider({
	children,
	defaultSettings = _default,
	...props
}: LogsProviderProps) {
	const { settings } = useSettings();
	const [clientSettings, setLogs] = useState<ClientSettings>(
		() =>
			(JSON.parse(
				localStorage.getItem(logsKey) ?? JSON.stringify(""),
			) as ClientSettings) || defaultSettings,
	);

	const { logs, clientId } = clientSettings;

	useWebSocket(`ws://${settings.websocketHost}`, {
		reconnectAttempts: 5,
		onOpen: () => {
			console.log("connection established");

			if (clientId) {
				fetch(`/api/logs`, {
					method: "GET",
					headers: {
						"client-id": `${clientId}`,
					},
				}).then(async (res) => {
					try {
						const json = await res.json();
						console.log(json);
						const response = json as Array<Log>;
						const _new = { logs: response, clientId: clientId };

						setLogs(_new);
						localStorage.setItem(logsKey, JSON.stringify(_new));
					} catch (error) {
						console.error(`could not convert response: ${error}`);
					}
				});

				return;
			}

			type Response = {
				client_id: number;
			};

			fetch(`/api/get_or_register_client`, { method: "post" }).then(
				async (res) => {
					try {
						const response = (await res.json()) as Response;
						const _new = { logs: logs, clientId: response.client_id };
						setLogs(_new);
						localStorage.setItem(logsKey, JSON.stringify(_new));
					} catch (error) {
						console.error(`could not convert response: ${error}`);
					}
				},
			);
		},
		onMessage: (event) => {
			const log = JSON.parse(event.data) as Log;

			if (log) {
				console.log(`log recieved: ${log.id}`);

				const _new = { logs: [...logs, log], clientId };
				setLogs(_new);
				localStorage.setItem(logsKey, JSON.stringify(_new));
			}
		},
		shouldReconnect: () => true,
	});

	const value = {
		client: clientSettings,
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
