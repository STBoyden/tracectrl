import "./App.css";

import { HeaderBar } from "@/components/header-bar";
import { ThemeProvider } from "@/components/theme-provider";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <>
        <HeaderBar />
      </>
    </ThemeProvider>
  );
}

export default App;
