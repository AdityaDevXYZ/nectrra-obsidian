import { useState } from 'react'
import { motion } from 'framer-motion'
import { Send, Hexagon } from 'lucide-react'

export default function ChatInterface() {
  const [messages, setMessages] = useState([
    { role: 'obsidian', content: "Node initialized. I am Obsidian. How can I assist you today?" }
  ])
  const [input, setInput] = useState('')
  const [isThinking, setIsThinking] = useState(false)

  const handleSend = async () => {
    if (!input.trim()) return
    const userMsg = input
    setMessages(prev => [...prev, { role: 'user', content: userMsg }])
    setInput('')
    setIsThinking(true)

    try {
      const response = await fetch('https://dirty-houses-ring.loca.lt/query', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Bypass-Tunnel-Reminder': 'true' // Required to bypass Localtunnel's warning screen!
        },
        body: JSON.stringify({ prompt: userMsg })
      })
      
      const data = await response.json()
      setMessages(prev => [...prev, { role: 'obsidian', content: data.answer }])
    } catch (e) {
      setMessages(prev => [...prev, { role: 'obsidian', content: "SYSTEM ERROR: Failed to connect to local Obsidian Node." }])
    } finally {
      setIsThinking(false)
    }
  }

  return (
    <motion.div 
      className="page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <div className="chat-container">
        <div className="chat-history">
          {messages.map((msg, idx) => (
            <motion.div 
              key={idx} 
              className={`chat-bubble ${msg.role}`}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
            >
              {msg.role === 'obsidian' && <Hexagon size={16} color="var(--accent-neon)" style={{ marginBottom: '8px' }}/>}
              {msg.content}
            </motion.div>
          ))}
          {isThinking && (
            <div className="typing-indicator">
              <span>Routing to Swarm</span>
              <div className="dot"></div><div className="dot"></div><div className="dot"></div>
            </div>
          )}
        </div>
        
        <div className="input-area">
          <input 
            type="text" 
            placeholder="Ask Obsidian anything..." 
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
          />
          <button className="send-btn" onClick={handleSend}>
            <Send size={20} />
          </button>
        </div>
      </div>
    </motion.div>
  )
}
