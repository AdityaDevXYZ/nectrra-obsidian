import { motion } from 'framer-motion'
import { ArrowRight, Cpu, Network, Zap } from 'lucide-react'

export default function LandingPage({ onLaunch }) {
  return (
    <motion.div 
      className="page"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.5 }}
    >
      <div style={{ textAlign: 'center', marginTop: '10vh', padding: '0 2rem' }}>
        <motion.h1 
          style={{ fontSize: '4rem', marginBottom: '1rem', background: 'linear-gradient(90deg, #fff, var(--accent-neon))', WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent' }}
          initial={{ scale: 0.9 }}
          animate={{ scale: 1 }}
          transition={{ duration: 0.8, type: 'spring' }}
        >
          Sovereign Hyper-Sparse<br/>Swarm Intelligence
        </motion.h1>
        <p style={{ color: 'var(--text-secondary)', fontSize: '1.2rem', maxWidth: '600px', margin: '0 auto 3rem auto', lineHeight: '1.6' }}>
          Obsidian replaces capital-heavy datacenters with architectural ingenuity. Harness the power of a decentralized, 1-bit quantized neural network.
        </p>
        <button 
          className="primary-btn" 
          style={{ fontSize: '1.2rem', padding: '1rem 3rem' }}
          onClick={onLaunch}
        >
          Initialize Obsidian <ArrowRight size={20} style={{ display: 'inline', marginLeft: '10px', verticalAlign: 'middle' }}/>
        </button>
      </div>

      <div style={{ display: 'flex', justifyContent: 'center', gap: '2rem', marginTop: '15vh', flexWrap: 'wrap', padding: '0 2rem' }}>
        {[
          { icon: <Cpu size={32} color="var(--accent-purple)"/>, title: 'Ternary Quantization', desc: '1-bit {-1, 0, 1} matrix math bypasses floating point ALUs completely.' },
          { icon: <Network size={32} color="var(--accent-neon)"/>, title: 'Decentralized Swarm', desc: 'Secure MCTS logic routing across a global P2P mesh network.' },
          { icon: <Zap size={32} color="#fff"/>, title: 'Zero Capital', desc: 'Training autonomously via RLAIF on consumer edge devices.' }
        ].map((feature, i) => (
          <motion.div 
            key={i}
            style={{ background: 'var(--glass-bg)', padding: '2rem', borderRadius: '15px', border: '1px solid var(--glass-border)', width: '300px' }}
            whileHover={{ y: -10, borderColor: 'var(--accent-neon)' }}
          >
            {feature.icon}
            <h3 style={{ marginTop: '1rem', marginBottom: '0.5rem' }}>{feature.title}</h3>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.9rem', lineHeight: '1.5' }}>{feature.desc}</p>
          </motion.div>
        ))}
      </div>
    </motion.div>
  )
}
