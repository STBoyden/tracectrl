import "./App.css";

import { HeaderBar } from "@/components/header-bar";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/toaster";

function App() {
  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <>
        <HeaderBar />

        <Toaster />
      </>
    </ThemeProvider>
  );
}

export default App;
