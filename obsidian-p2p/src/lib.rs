use libp2p::{
    futures::StreamExt,
    mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
pub struct ObsidianBehaviour {
    pub mdns: mdns::tokio::Behaviour,
}

pub async fn start_p2p_node() -> Result<(), Box<dyn Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(ObsidianBehaviour { mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(std::time::Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Obsidian P2P Node Started. Local Peer ID: {}", swarm.local_peer_id());

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
            }
            SwarmEvent::Behaviour(ObsidianBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("mDNS discovered a new peer: {} at {}", peer_id, multiaddr);
                }
            }
            SwarmEvent::Behaviour(ObsidianBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("mDNS peer expired: {} at {}", peer_id, multiaddr);
                }
            }
            _ => {}
        }
    }
}
