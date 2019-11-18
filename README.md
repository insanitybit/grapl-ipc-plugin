# grapl-ipc-plugin
A plugin for Grapl that provides Inter Process Communication primitives

This repo contains two libraries;
* grapl-ipc-analyzer-plugin
* grapl-ipc-generator-plugin

### grapl-ipc-analyzer-plugin

The analyzer plugin defines two Python constructs, the
`IpcQuery` and the `IpcView`. These can be used to query
Grapl's master graph for suspicious inter-process communication.

Example: Querying for processes with IPC to `sshd`, indicating
potential ssh hijacking

```python
suspect_ipcs = (
    IpcQuery()
    .with_ipc_recipient(
        ProcessQuery()
        .with_process_name(eq="sshd")
    )
    .query(mg_client)
)  # type: List[IpcView]

for ipc in suspect_ipcs:
    print(ipc.get_ipc_creator().get_process_name())
```  

### grapl-ipc-generator-plugin

The generator plugin defines an Ipc node, which can be built from parsed data
in a Grapl subgraph generator.

In this example we generate a Grapl GraphDescription containing two ProcessDescription
nodes as well as an Ipc node conneting them.

```rust

use grapl_ipc_generator_plugin::{
    static_strategy as ipc_identity,
    IIpcNode,
    IpcNode,
    IPC_CREATOR,
    IPC_RECIPIENT
};

fn event_to_graph(data: &str) -> Result<GraphDescription> {
    let event = parse_from_data(&data)?;
    
    let mut graph = GraphDescription::new(event.event_time);

    let mut ipc_node = IpcNode::new(ipc_identity(), event.event_time);

    ipc_node
        .with_asset_id(event.hostname.to_owned())
        .with_src_pid(event.src_pid)
        .with_dst_pid(event.agent_pid)
        .with_ipc_type("UNIX_SOCKET");

    let creator: ProcessDescription = process_from_event(&event)?;
    let recipient: ProcessDescription = process_from_event(&event)?;
    
    graph.add_node(ipc_node.clone());
    graph.add_node(src_pid.clone());
    graph.add_node(ssh_agent_process.clone());

    graph.add_edge(
        IPC_CREATOR,
        ipc_node.clone_node_key(),
        src_pid.clone_key(),
    );

    graph.add_edge(
        IPC_RECIPIENT,
        ipc_node.clone_node_key(),
        ssh_agent_process.clone_key(),
    );

    Ok(ipc_node)
}
```