import React, { createContext, useContext, useState } from "react";
import * as z from "zod";

const settingsKey = "tracectrl-settings" as const;

type Settings = {
  host?: string;
};

export const _defaultSettings: Settings = {
  host: "localhost:3002",
};

export const settingsFormSchema = z.object({
  host: z
    .union([
      z.string().url("Invalid URL"),
      z
        .string()
        .ip({ version: "v4", message: "IP address has to be in ipv4 format" }),
    ])
    .optional()
    .default(_defaultSettings.host!),
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
      (JSON.parse(localStorage.getItem(settingsKey) ?? "") as Settings) ||
      defaultSettings
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
