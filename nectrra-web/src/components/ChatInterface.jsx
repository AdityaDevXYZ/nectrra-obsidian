import { useState } from 'react'
import { motion } from 'framer-motion'
import { Send, Hexagon } from 'lucide-react'
import ReactMarkdown from 'react-markdown'

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
      // Fetch from the permanent cloud API (or localhost if testing)
      const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080'
      const response = await fetch(`${API_URL}/query`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
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
              {msg.role === 'obsidian' ? (
                <div className="markdown-body">
                  <ReactMarkdown>{msg.content}</ReactMarkdown>
                </div>
              ) : (
                msg.content
              )}
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
