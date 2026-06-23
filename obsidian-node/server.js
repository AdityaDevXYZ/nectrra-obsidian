const http = require('http');

const PORT = 8080;

const server = http.createServer((req, res) => {
    // Enable CORS
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'POST, GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    if (req.url === '/query' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });

        req.on('end', () => {
            try {
                const payload = JSON.parse(body);
                const prompt = payload.prompt || '';
                
                // MOCK SPLIT-BRAIN ROUTING LOGIC (Mirrors Rust heuristic)
                const isComplex = prompt.length > 20 || prompt.toLowerCase().includes('math') || prompt.toLowerCase().includes('what') || prompt.toLowerCase().includes('how') || prompt.toLowerCase().includes('?');

                let answer = "";
                if (!isComplex) {
                    answer = `**Local Ternary Engine Output:**\nSparse Tensor Activation for query '${prompt}' yielded minimal semantic density. 1-Bit weights currently lack contextual mapping.\n\n*Training epochs required to understand this semantic branch: ~14,500.*`;
                } else {
                    answer = `**MCTS Swarm Output:**\nQuery '${prompt}' parsed and logic tree distributed across idle mesh nodes.\n\nMonte Carlo evaluation score: \`0.12\`.\n\n*Logic branch is currently unexplored. Global loss optimization via RLAIF pending.*`;
                }

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({
                    answer: answer,
                    routed_to_swarm: isComplex
                }));
            } catch (e) {
                res.writeHead(400);
                res.end('Bad Request');
            }
        });
    } else {
        res.writeHead(404);
        res.end('Not Found');
    }
});

server.listen(PORT, '0.0.0.0', () => {
    console.log(`[FALLBACK MODE] Web API Server listening on 0.0.0.0:${PORT}`);
    console.log(`Obsidian Node is running via Node.js fallback due to Termux Rustc architecture corruption.`);
});
