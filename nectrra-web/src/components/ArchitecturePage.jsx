import { motion } from 'framer-motion'

export default function ArchitecturePage() {
  return (
    <motion.div 
      className="page"
      style={{ padding: '40px 4rem 4rem 4rem', maxWidth: '1000px', margin: '0 auto' }}
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
    >
      <h1 style={{ fontSize: '3rem', marginBottom: '2rem' }}>Architecture Breakdown</h1>
      <p style={{ color: 'var(--text-secondary)', fontSize: '1.2rem', marginBottom: '3rem', lineHeight: '1.6' }}>
        The Nectrra ecosystem is built on 7 foundational pillars engineered natively in Rust. We bypassed standard matrix multiplications entirely, replacing them with extremely sparse mathematical operations executable directly on consumer edge devices.
      </p>
      
      <div style={{ display: 'flex', flexDirection: 'column', gap: '2rem' }}>
        {[
          { phase: "Phase 1: Compute Network", desc: "A custom Libp2p mesh network bypassing centralized servers to pass messages across firewalls." },
          { phase: "Phase 2: Mathematical Quantization", desc: "1-Bit Ternary weights {-1, 0, 1} combined with a State Space Model (SSM) for linear-time context." },
          { phase: "Phase 3: Multi-Tier Execution", desc: "A Split-Brain router that handles easy tasks locally, and farms complex logic puzzles to the Swarm." },
          { phase: "Phase 4: Decentralized MCTS", desc: "Monte Carlo Tree Search executing logic branches across thousands of idle devices simultaneously." },
          { phase: "Phase 5: Autonomous RLAIF", desc: "Idle nodes automatically synthesize and mathematically verify golden data to train the model." },
          { phase: "Phase 6: Byzantine Resilience", desc: "Geometric median filtering instantly discards poisoned gradient updates from malicious peers." },
          { phase: "Phase 7: Straight-Through Estimator", desc: "Federated Training loops utilizing latent float representations to calculate exact gradients." },
        ].map((item, i) => (
          <motion.div 
            key={i}
            style={{ background: 'var(--glass-bg)', borderLeft: '4px solid var(--accent-purple)', padding: '1.5rem', borderRadius: '0 8px 8px 0' }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: i * 0.1 }}
          >
            <h3 style={{ marginBottom: '0.5rem', color: 'var(--text-primary)' }}>{item.phase}</h3>
            <p style={{ color: 'var(--text-secondary)' }}>{item.desc}</p>
          </motion.div>
        ))}
      </div>
    </motion.div>
  )
}
