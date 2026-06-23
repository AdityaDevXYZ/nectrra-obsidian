import { useState } from 'react'
import { AnimatePresence } from 'framer-motion'
import LandingPage from './components/LandingPage'
import ChatInterface from './components/ChatInterface'
import ArchitecturePage from './components/ArchitecturePage'

function App() {
  const [currentPage, setCurrentPage] = useState('landing')

  return (
    <div className="app-container">
      <nav className="glass-nav">
        <div className="logo" onClick={() => setCurrentPage('landing')}>Nectrra</div>
        <div className="nav-links">
          <button onClick={() => setCurrentPage('architecture')}>Architecture</button>
          <button className="primary-btn" onClick={() => setCurrentPage('chat')}>Launch Obsidian</button>
        </div>
      </nav>

      <AnimatePresence mode="wait">
        {currentPage === 'landing' && <LandingPage key="landing" onLaunch={() => setCurrentPage('chat')} />}
        {currentPage === 'chat' && <ChatInterface key="chat" />}
        {currentPage === 'architecture' && <ArchitecturePage key="architecture" />}
      </AnimatePresence>
    </div>
  )
}

export default App
