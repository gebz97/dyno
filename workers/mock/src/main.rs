use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    children: Vec<String>,
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

    let mut consumer = channel
        .basic_consume(
            "dyno-work-queue",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Worker ready, waiting for tasks…");

    while let Some(delivery_result) = consumer.next().await {
        let delivery = delivery_result?;
        let channel_clone = channel.clone();

        // spawn each task in parallel
        tokio::spawn(async move {
            let msg: Result<TaskMessage, _> = serde_json::from_slice(&delivery.data);
            match msg {
                Ok(task_msg) => {
                    println!("Worker picked up task {}", task_msg.task.id);
                    sleep(Duration::from_secs(2)).await;
                    println!("Worker finished task {}", task_msg.task.id);

                    let status = TaskStatus {
                        workflow_id: task_msg.workflow_id,
                        task_id: task_msg.task.id,
                        status: "finished".into(),
                    };
                    let payload = serde_json::to_vec(&status).unwrap();

                    // ✅ fixed publish confirm handling
                    match channel_clone
                        .basic_publish(
                            "",
                            "dyno-task-status-queue",
                            BasicPublishOptions::default(),
                            &payload,
                            BasicProperties::default(),
                        )
                        .await
                    {
                        Ok(conf) => {
                            if let Err(e) = conf.await {
                                eprintln!("Publish confirm failed: {:?}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to publish status: {:?}", e),
                    }

                    if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                        eprintln!("Ack failed: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to parse task: {:?}", e);
                    let _ = delivery.nack(BasicNackOptions::default()).await;
                }
            }
        });
    }

    Ok(())
}
