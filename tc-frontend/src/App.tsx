import "./App.css";

import { HeaderBar } from "@/components/header-bar";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/toaster";
import { SettingsProvider } from "@/components/settings-provider";
import { LogsProvider } from "@/components/logs-provider";
import { LogsArea } from "@/components/logs-area";

function App() {
	return (
		<ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
			<SettingsProvider>
				<LogsProvider>
					<>
						<HeaderBar />
						<LogsArea />
						<Toaster />
					</>
				</LogsProvider>
			</SettingsProvider>
		</ThemeProvider>
	);
}

export default App;
