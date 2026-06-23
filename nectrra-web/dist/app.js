const pages = {
    landing: `
        <div class="page slide-up">
            <div style="text-align: center; margin-top: 10vh; padding: 0 2rem;">
                <h1 style="font-size: 4rem; margin-bottom: 1rem; background: linear-gradient(90deg, #fff, var(--accent-neon)); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                    Sovereign Hyper-Sparse<br/>Swarm Intelligence
                </h1>
                <p style="color: var(--text-secondary); font-size: 1.2rem; max-width: 600px; margin: 0 auto 3rem auto; line-height: 1.6;">
                    Obsidian replaces capital-heavy datacenters with architectural ingenuity. Harness the power of a decentralized, 1-bit quantized neural network.
                </p>
                <button class="primary-btn" style="font-size: 1.2rem; padding: 1rem 3rem;" onclick="navigate('chat')">
                    Initialize Obsidian <i data-lucide="arrow-right" style="vertical-align: middle; margin-left: 10px;"></i>
                </button>
            </div>
            <div style="display: flex; justify-content: center; gap: 2rem; margin-top: 15vh; flex-wrap: wrap; padding: 0 2rem;">
                <div class="feature-card">
                    <i data-lucide="cpu" color="var(--accent-purple)" width="32" height="32"></i>
                    <h3 style="margin-top: 1rem; margin-bottom: 0.5rem;">Ternary Quantization</h3>
                    <p style="color: var(--text-secondary); font-size: 0.9rem; line-height: 1.5;">1-bit {-1, 0, 1} matrix math bypasses floating point ALUs completely.</p>
                </div>
                <div class="feature-card">
                    <i data-lucide="network" color="var(--accent-neon)" width="32" height="32"></i>
                    <h3 style="margin-top: 1rem; margin-bottom: 0.5rem;">Decentralized Swarm</h3>
                    <p style="color: var(--text-secondary); font-size: 0.9rem; line-height: 1.5;">Secure MCTS logic routing across a global P2P mesh network.</p>
                </div>
                <div class="feature-card">
                    <i data-lucide="zap" color="#fff" width="32" height="32"></i>
                    <h3 style="margin-top: 1rem; margin-bottom: 0.5rem;">Zero Capital</h3>
                    <p style="color: var(--text-secondary); font-size: 0.9rem; line-height: 1.5;">Training autonomously via RLAIF on consumer edge devices.</p>
                </div>
            </div>
        </div>
    `,
    architecture: `
        <div class="page slide-up" style="padding: 40px 4rem 4rem 4rem; max-width: 1000px; margin: 0 auto;">
            <h1 style="font-size: 3rem; margin-bottom: 2rem;">Architecture Breakdown</h1>
            <p style="color: var(--text-secondary); font-size: 1.2rem; margin-bottom: 3rem; line-height: 1.6;">
                The Nectrra ecosystem is built on 7 foundational pillars engineered natively in Rust. We bypassed standard matrix multiplications entirely, replacing them with extremely sparse mathematical operations executable directly on consumer edge devices.
            </p>
            <div style="display: flex; flex-direction: column; gap: 2rem;">
                <!-- Phases -->
                <div class="feature-card" style="width: 100%; border-left: 4px solid var(--accent-purple); border-radius: 0 8px 8px 0; padding: 1.5rem;">
                    <h3 style="margin-bottom: 0.5rem;">Phase 1 & 2: Compute Network & Quantization</h3>
                    <p style="color: var(--text-secondary);">A custom Libp2p mesh running 1-Bit Ternary weights {-1, 0, 1} and a State Space Model (SSM).</p>
                </div>
                <div class="feature-card" style="width: 100%; border-left: 4px solid var(--accent-neon); border-radius: 0 8px 8px 0; padding: 1.5rem;">
                    <h3 style="margin-bottom: 0.5rem;">Phase 3 & 4: Split-Brain Routing & Decentralized MCTS</h3>
                    <p style="color: var(--text-secondary);">The Local Reflex Engine seamlessly routes complex logic to thousands of idle Swarm nodes simultaneously.</p>
                </div>
                <div class="feature-card" style="width: 100%; border-left: 4px solid var(--accent-purple); border-radius: 0 8px 8px 0; padding: 1.5rem;">
                    <h3 style="margin-bottom: 0.5rem;">Phase 5, 6 & 7: Autonomous Federated Training</h3>
                    <p style="color: var(--text-secondary);">RLAIF data synthesis backed by Geometric Byzantine filtering and Straight-Through Estimator gradients.</p>
                </div>
            </div>
        </div>
    `,
    chat: `
        <div class="page fade-in">
            <div class="chat-container">
                <div class="chat-history" id="chat-history">
                    <div class="chat-bubble obsidian slide-up">
                        <i data-lucide="hexagon" color="var(--accent-neon)" width="16" height="16" style="margin-bottom:8px"></i><br/>
                        Node initialized. I am Obsidian. You are connected to the global MCTS Swarm. What impossible logic puzzle shall we solve today?
                    </div>
                </div>
                <div class="input-area slide-up" style="animation-delay: 0.2s;">
                    <input type="text" id="chat-input" placeholder="Ask Obsidian anything... (Math, Physics, Current Affairs)" onkeypress="if(event.key === 'Enter') sendMessage()">
                    <button class="send-btn" onclick="sendMessage()"><i data-lucide="send" width="20" height="20"></i></button>
                </div>
            </div>
        </div>
    `
};

function navigate(page) {
    const content = document.getElementById('page-content');
    content.innerHTML = pages[page];
    lucide.createIcons();
    window.scrollTo(0,0);
}

async function sendMessage() {
    const input = document.getElementById('chat-input');
    const msg = input.value.trim();
    if(!msg) return;
    
    const history = document.getElementById('chat-history');
    
    // Add User Message
    const userDiv = document.createElement('div');
    userDiv.className = 'chat-bubble user slide-up';
    userDiv.innerText = msg;
    history.appendChild(userDiv);
    
    input.value = '';
    history.scrollTop = history.scrollHeight;
    
    // Evaluate complexity to determine routing
    const isComplex = msg.length > 20 || msg.toLowerCase().includes('math') || msg.toLowerCase().includes('what') || msg.toLowerCase().includes('how') || msg.toLowerCase().includes('?');
    const loadingText = isComplex ? 'Routing query across global Swarm Nodes' : 'Processing via Local Ternary Reflex Engine';
    
    // Show correct thinking indicator
    const thinkingDiv = document.createElement('div');
    thinkingDiv.className = 'typing-indicator fade-in';
    thinkingDiv.id = 'thinking';
    thinkingDiv.innerHTML = '<span>' + loadingText + '</span><div class="dot"></div><div class="dot"></div><div class="dot"></div>';
    history.appendChild(thinkingDiv);
    history.scrollTop = history.scrollHeight;
    
    try {
        // We use the ACTUAL Rust daemon running locally!
        // This is the "Hard Way" - raw, unpolished tensor outputs.
        const response = await fetch('http://localhost:8080/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ prompt: msg })
        });
        
        let rawMarkdown = "";
        let actuallyRoutedToSwarm = false;
        
        if (response.ok) {
            const data = await response.json();
            rawMarkdown = data.answer;
            actuallyRoutedToSwarm = data.routed_to_swarm;
        } else {
            rawMarkdown = "My local rust daemon connection failed. Ensure `cargo run` is actively running on port 8080.";
        }

        // Convert the markdown response to HTML using Marked.js
        const parsedHtml = marked.parse(rawMarkdown);

        // Add prefix based on actual backend routing
        const prefix = actuallyRoutedToSwarm 
            ? "<b style='color: var(--accent-neon);'>[MCTS Swarm Output]</b><br/><br/>" 
            : "<b style='color: var(--text-secondary);'>[Local Ternary Reflex Output]</b><br/><br/>";

        finishMessage(prefix + parsedHtml);

    } catch (e) {
        finishMessage("<b style='color: #ff3333;'>[Local API Error]</b> Could not reach localhost:8080. Ensure the Obsidian Node is running.");
    }

    function finishMessage(finalHtml) {
        const thinkingEl = document.getElementById('thinking');
        if(thinkingEl) history.removeChild(thinkingEl);
        
        const obsidianDiv = document.createElement('div');
        obsidianDiv.className = 'chat-bubble obsidian slide-up';
        
        // Custom styling for markdown elements in the chat
        obsidianDiv.innerHTML = '<i data-lucide="hexagon" color="var(--accent-neon)" width="16" height="16" style="margin-bottom:8px"></i><br/>' + 
                                '<div class="markdown-body" style="color: #eee; font-family: \'Inter\', sans-serif;">' + finalHtml + '</div>';
        history.appendChild(obsidianDiv);
        lucide.createIcons();
        history.scrollTop = history.scrollHeight;
    }
}

// Init
window.onload = () => navigate('landing');
