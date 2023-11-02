import "./App.css";

import { HeaderBar } from "@/components/header-bar";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/toaster";
import { SettingsProvider } from "./components/settings-provider";

function App() {
  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <SettingsProvider>
        <>
          <HeaderBar />

          <Toaster />
        </>
      </SettingsProvider>
    </ThemeProvider>
  );
}

export default App;
