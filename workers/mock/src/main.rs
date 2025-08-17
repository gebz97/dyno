// workers/mock/src/main.rs
use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: String,
    children: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TaskMessage {
    workflow_id: String,
    task: Task,
}

#[derive(Debug, Deserialize, Serialize)]
struct TaskStatus {
    workflow_id: String,
    task_id: String,
    status: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = Connection::connect(
        "amqp://admin:admin@gopher.gebz.local:5672/%2f",
        ConnectionProperties::default(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    // ensure queues exist
    channel
        .queue_declare("dyno-work-queue", QueueDeclareOptions::default(), FieldTable::default())
        .await?;
    channel
        .queue_declare("dyno-task-status-queue", QueueDeclareOptions::default(), FieldTable::default())
        .await?;

    let mut consumer = channel
        .basic_consume(
            "dyno-work-queue",
            "", // let broker assign unique consumer tag for MPMC
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Worker ready, waiting for tasks…");

    while let Some(delivery_result) = tokio_stream::StreamExt::next(&mut consumer).await {
        match delivery_result {
            Ok(delivery) => {
                let payload = &delivery.data;
                let parsed = serde_json::from_slice::<TaskMessage>(payload);
                match parsed {
                    Ok(msg) => {
                        println!("Worker picked up task {} (wf {})", msg.task.id, msg.workflow_id);

                        // simulate work
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                        println!("Worker finished task {}", msg.task.id);

                        // publish status
                        let status = TaskStatus {
                            workflow_id: msg.workflow_id,
                            task_id: msg.task.id,
                            status: "finished".to_string(),
                        };
                        let payload = serde_json::to_vec(&status)?;
                        channel
                            .basic_publish(
                                "",
                                "dyno-task-status-queue",
                                BasicPublishOptions::default(),
                                &payload,
                                BasicProperties::default(),
                            )
                            .await?
                            .await?; // wait confirm
                    }
                    Err(e) => {
                        eprintln!("failed to parse TaskMessage: {:?}", e);
                    }
                }

                if let Err(e) = delivery.ack(Default::default()).await {
                    eprintln!("failed to ack delivery: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("consumer error: {:?}", e);
            }
        }
    }

    Ok(())
}
