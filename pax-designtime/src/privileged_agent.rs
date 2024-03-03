use std::net::SocketAddr;

use crate::{
    messages::{AgentMessage, ComponentSerializationRequest},
    orm::PaxManifestORM,
};
use anyhow::{anyhow, Result};
use ewebsock::{WsEvent, WsMessage};
use pax_manifest::ComponentDefinition;

pub struct PrivilegedAgentConnection {
    sender: ewebsock::WsSender,
    recver: ewebsock::WsReceiver,
}

impl PrivilegedAgentConnection {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let (sender, recver) = ewebsock::connect(format!("ws://{}/ws", addr))
            .map_err(|_| anyhow!("couldn't create socket connection"))?;
        Ok(Self {
            sender,
            recver: recver,
        })
    }

    pub fn send_component_update(&mut self, component: &ComponentDefinition) -> Result<()> {
        let component_bytes = rmp_serde::to_vec(&component)?;
        let msg_bytes = rmp_serde::to_vec(&AgentMessage::ComponentSerializationRequest(
            ComponentSerializationRequest { component_bytes },
        ))?;

        self.sender.send(ewebsock::WsMessage::Binary(msg_bytes));
        Ok(())
    }

    pub fn handle_recv(&mut self, manager: &mut PaxManifestORM) -> Result<()> {
        while let Some(event) = self.recver.try_recv() {
            match event {
                WsEvent::Message(ws_message) => {
                    if let WsMessage::Binary(msg_bytes) = ws_message {
                        let msg: AgentMessage = rmp_serde::from_slice(&msg_bytes)?;
                        match msg {
                            AgentMessage::UpdateTemplateRequest(resp) => {
                                manager
                                    .replace_template(resp.type_id, resp.new_template)
                                    .map_err(|e| anyhow!(e))?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
