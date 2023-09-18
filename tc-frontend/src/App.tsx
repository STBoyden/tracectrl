import './App.css'
import { ModeToggle } from './components/mode-toggle'

import { ThemeProvider } from './components/theme-provider'

function App() {

  return (
    <ThemeProvider defaultTheme='dark' storageKey='vite-ui-theme'>
      <div className='p-4'>
        <ModeToggle />
      </div>
    </ThemeProvider>
  )
}

export default App
