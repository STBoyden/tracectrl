/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useState } from "react";
import * as z from "zod";

const settingsKey = "tracectrl-settings" as const;

type Settings = {
	websocketHost: string;
};

export const _defaultSettings: Settings = {
	websocketHost: "localhost:3001",
};

export const settingsFormSchema = z.object({
	websocketHost: z
		.union([
			z.string().url("Invalid URL"),
			z
				.string()
				.ip({ version: "v4", message: "IP address has to be in ipv4 format" }),
		])
		.optional()
		.default(_defaultSettings.websocketHost!),
});

type SettingsProviderProps = {
	children: React.ReactNode;
	defaultSettings?: Settings;
};

type SettingsProviderState = {
	settings: Settings;
	setSettings: (settings: Settings) => void;
};

const initialState: SettingsProviderState = {
	settings: _defaultSettings,
	setSettings: () => null,
};

const SettingsProviderContext =
	createContext<SettingsProviderState>(initialState);

export function SettingsProvider({
	children,
	defaultSettings = _defaultSettings,
	...props
}: SettingsProviderProps) {
	const [settings, setSettings] = useState<Settings>(
		() =>
			(JSON.parse(
				localStorage.getItem(settingsKey) ?? JSON.stringify(""),
			) as Settings) || defaultSettings,
	);

	const value = {
		settings,
		setSettings: (settings: Settings) => {
			localStorage.setItem(settingsKey, JSON.stringify(settings));
			setSettings(settings);
		},
	};

	return (
		<SettingsProviderContext.Provider {...props} value={value}>
			{children}
		</SettingsProviderContext.Provider>
	);
}

export function useSettings() {
	const context = useContext(SettingsProviderContext);

	if (context === undefined)
		throw new Error("useSettings must be used within a SettingsProvider");

	return context;
}
