
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    children: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Workflow {
    id: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskMessage {
    workflow_id: String,
    task: Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskStatus {
    workflow_id: String,
    task_id: String,
    status: String, // "finished" / "failed"
}

/// In-memory workflow state. Single-controller assumption.
struct WorkflowState {
    graph: DiGraph<Task, ()>,
    node_map: HashMap<String, NodeIndex>,
    indegree: HashMap<NodeIndex, usize>,
    completed: HashSet<NodeIndex>,
    total: usize,
}

type Registry = Arc<Mutex<HashMap<String, WorkflowState>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect (use with_tokio for lapin+tokio integration)
    let conn = Connection::connect(
        "amqp://admin:admin@gopher.gebz.local:5672/%2f",
        ConnectionProperties::default(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    // Ensure queues exist
    channel
        .queue_declare(
            "dyno-submit-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    channel
        .queue_declare(
            "dyno-work-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    channel
        .queue_declare(
            "dyno-task-status-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    channel
        .queue_declare(
            "dyno-workflow-status-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let registry: Registry = Arc::new(Mutex::new(HashMap::new()));

    // Spawn submit consumer (broker assigns consumer tag)
    {
        let ch = channel.clone();
        let registry = registry.clone();
        tokio::spawn(async move {
            let mut consumer = ch
                .basic_consume(
                    "dyno-submit-queue",
                    "", // broker-assigned tag
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("submit basic_consume");

            println!("Controller submit consumer running.");

            while let Some(delivery_result) = tokio_stream::StreamExt::next(&mut consumer).await {
                match delivery_result {
                    Ok(delivery) => {
                        // parse workflow
                        let payload = &delivery.data;
                        match serde_json::from_slice::<Workflow>(payload) {
                            Ok(wf) => {
                                println!("Got workflow {}", wf.id);
                                if let Err(e) =
                                    register_and_publish_initial(&ch, &registry, wf).await
                                {
                                    eprintln!("failed handling submit: {:?}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("invalid workflow JSON: {:?}", e);
                            }
                        }
                        // ack always to avoid redelivery in this example
                        if let Err(e) = delivery.ack(Default::default()).await {
                            eprintln!("failed to ack submit: {:?}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("submit consumer error: {:?}", e);
                    }
                }
            }
        });
    }

    // Spawn single global status consumer (fixed tag)
    {
        let ch = channel.clone();
        let registry = registry.clone();
        tokio::spawn(async move {
            let mut consumer = ch
                .basic_consume(
                    "dyno-task-status-queue",
                    "dyno-controller-status", // fixed tag; single controller process expected
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("status basic_consume");

            println!("Controller status consumer running.");

            while let Some(delivery_result) = tokio_stream::StreamExt::next(&mut consumer).await {
                match delivery_result {
                    Ok(delivery) => {
                        let payload = &delivery.data;
                        match serde_json::from_slice::<TaskStatus>(payload) {
                            Ok(status) => {
                                if let Err(e) = handle_status(&ch, &registry, status).await {
                                    eprintln!("failed to handle status: {:?}", e);
                                }
                            }
                            Err(e) => eprintln!("invalid TaskStatus JSON: {:?}", e),
                        }

                        if let Err(e) = delivery.ack(Default::default()).await {
                            eprintln!("failed to ack status: {:?}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("status consumer error: {:?}", e);
                    }
                }
            }
        });
    }

    // keep main alive
    println!("Controller ready.");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

/// Register workflow internal state and publish initial ready tasks.
/// This function builds the graph and the indegree map, inserts it into registry, and publishes tasks with indegree 0.
async fn register_and_publish_initial(
    ch: &Channel,
    registry: &Registry,
    wf: Workflow,
) -> anyhow::Result<()> {
    // Build graph
    let mut graph: DiGraph<Task, ()> = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for t in &wf.tasks {
        let idx = graph.add_node(t.clone());
        node_map.insert(t.id.clone(), idx);
    }
    for t in &wf.tasks {
        if let Some(&u) = node_map.get(&t.id) {
            for child in &t.children {
                if let Some(&v) = node_map.get(child) {
                    graph.add_edge(u, v, ());
                }
            }
        }
    }

    let indegree: HashMap<NodeIndex, usize> = graph
        .node_indices()
        .map(|i| (i, graph.neighbors_directed(i, Direction::Incoming).count()))
        .collect();

    let total = graph.node_count();
    let state = WorkflowState {
        graph,
        node_map,
        indegree,
        completed: HashSet::new(),
        total,
    };

    // Insert into registry
    {
        let mut reg = registry.lock().await;
        if reg.contains_key(&wf.id) {
            anyhow::bail!("workflow {} already exists", wf.id);
        }
        reg.insert(wf.id.clone(), state);
    }

    // Publish initial tasks (indegree == 0)
    // We must take a snapshot of which tasks to publish without holding registry lock during publish.
    let to_publish: Vec<TaskMessage> = {
        let reg = registry.lock().await;
        let s = reg.get(&wf.id).context("state present")?;
        s.indegree
            .iter()
            .filter_map(|(&idx, &deg)| {
                if deg == 0 {
                    Some(TaskMessage {
                        workflow_id: wf.id.clone(),
                        task: s.graph[idx].clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    };

    for msg in to_publish {
        publish_task(ch, &msg).await?;
        println!("Published initial task {}", msg.task.id);
    }

    Ok(())
}

/// Handle a TaskStatus from workers. Find workflow state, update indegree, and publish children that become ready.
/// Safety: do minimal work under lock; collect children to publish then release lock before publishing.
async fn handle_status(
    ch: &Channel,
    registry: &Registry,
    status: TaskStatus,
) -> anyhow::Result<()> {
    // find workflow and update its state
    let mut publish_list: Vec<TaskMessage> = Vec::new();
    {
        let mut reg = registry.lock().await;
        let state = match reg.get_mut(&status.workflow_id) {
            Some(s) => s,
            None => {
                eprintln!("no workflow state for {}", status.workflow_id);
                return Ok(());
            }
        };

        // treat only finished statuses here
        if status.status != "finished" {
            eprintln!(
                "task {} reported non-finished: {}",
                status.task_id, status.status
            );
            return Ok(());
        }

        // locate node index
        let &finished_idx = match state.node_map.get(&status.task_id) {
            Some(i) => i,
            None => {
                eprintln!(
                    "unknown task id {} for workflow {}",
                    status.task_id, status.workflow_id
                );
                return Ok(());
            }
        };

        // idempotent guard
        if state.completed.contains(&finished_idx) {
            return Ok(());
        }
        state.completed.insert(finished_idx);

        // decrement indegree of children and collect newly-ready children
        for child in state
            .graph
            .neighbors_directed(finished_idx, Direction::Outgoing)
        {
            if let Some(deg) = state.indegree.get_mut(&child) {
                if *deg == 0 {
                    eprintln!(
                        "warning: indegree already zero for child {} of {}",
                        state.graph[child].id, state.graph[finished_idx].id
                    );
                    continue;
                }
                *deg -= 1;
                if *deg == 0 && !state.completed.contains(&child) {
                    publish_list.push(TaskMessage {
                        workflow_id: status.workflow_id.clone(),
                        task: state.graph[child].clone(),
                    });
                }
            } else {
                eprintln!(
                    "missing indegree entry for child {:?}",
                    state.graph[child].id
                );
            }
        }

        // optional final cleanup: if completed == total, remove state
        if state.completed.len() == state.total {
            println!("workflow {} complete, cleaning up", status.workflow_id);
            // remove after releasing lock to avoid borrow issues: mark for removal
        }
    } // lock dropped here

    // publish all children outside lock
    for msg in publish_list {
        publish_task(ch, &msg).await?;
        println!("Published child task {}", msg.task.id);
    }

    // If workflow finished, remove its state (do separately to avoid holding lock earlier)
    {
        let mut reg = registry.lock().await;
        if let Some(s) = reg.get(&status.workflow_id) {
            if s.completed.len() == s.total {
                reg.remove(&status.workflow_id);
                // publish final workflow status if needed (not implemented here)
            }
        }
    }

    Ok(())
}

async fn publish_task(ch: &Channel, msg: &TaskMessage) -> anyhow::Result<()> {
    let payload = serde_json::to_vec(msg)?;
    ch.basic_publish(
        "",
        "dyno-work-queue",
        BasicPublishOptions::default(),
        &payload,
        BasicProperties::default(),
    )
    .await?
    .await?; // wait confirm
    Ok(())
}
