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
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginRight: '1rem' }}>
            <div className="dot" style={{ width: '8px', height: '8px', backgroundColor: 'var(--accent-neon)', borderRadius: '50%', boxShadow: '0 0 10px var(--accent-neon)' }}></div>
            <span style={{ fontSize: '0.85rem', color: 'var(--accent-neon)', letterSpacing: '1px', fontWeight: '600' }}>SWARM ONLINE</span>
          </div>
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
