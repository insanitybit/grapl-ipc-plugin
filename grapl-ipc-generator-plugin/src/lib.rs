use grapl_graph_descriptions::graph_description::*;
use grapl_graph_descriptions::*;
use grapl_graph_descriptions::graph_description::{Static, IdStrategy};

use derive_dynamic_node::{DynamicNode as GraplNode};

pub const IPC_CREATOR: &'static str = "ipc_creator";
pub const IPC_RECIPIENT: &'static str = "ipc_recipient";

#[derive(Clone, GraplNode)]
pub struct Ipc {
    hostname: String,
    src_pid: u64,
    dst_pid: u64,
    ipc_type: String,
}

pub fn static_strategy() -> IdStrategy {
    Static {
        primary_key_properties: vec![
                    "src_pid".to_string(), "dst_pid".to_string()
                ],
        primary_key_requires_asset_id: true,
    }.into()
}

impl IIpcNode for IpcNode {
    fn get_mut_dynamic_node(&mut self) -> &mut DynamicNode {
        &mut self.dynamic_node
    }
}


