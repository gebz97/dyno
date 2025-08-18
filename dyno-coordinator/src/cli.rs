use anyhow::{Context, Error};
use clap::Parser;
use std::fs;

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "dyno-coordinator")]
#[command(about = "The dyno Co-ordinator program", long_about = None)]
struct CliArgs {
    #[arg(long)]
    config: Option<String>,

    #[arg(long)]
    etcd_uri: Option<String>,

    #[arg(long)]
    amqp_uri: Option<String>,

    #[arg(long)]
    workflow_submit_queue: Option<String>,

    #[arg(long)]
    task_submit_queue: Option<String>,

    #[arg(long)]
    worker_queue: Option<String>,

    #[arg(long)]
    worker_status_queue: Option<String>,

    #[arg(long)]
    workflow_status_queue: Option<String>,

    #[arg(long)]
    task_status_queue: Option<String>,
}

// Define some default constants
const DEFAULT_ETCD_URI: &str = "http://127.0.0.1:2379";
const DEFAULT_AMQP_URI: &str = "amqp://127.0.0.1:5672";
const DEFAULT_WORKFLOW_SUBMIT_QUEUE: &str = "workflow.submit";
const DEFAULT_TASK_SUBMIT_QUEUE: &str = "task.submit";
const DEFAULT_WORKER_QUEUE: &str = "worker.queue";
const DEFAULT_WORKER_STATUS_QUEUE: &str = "worker.status";
const DEFAULT_WORKFLOW_STATUS_QUEUE: &str = "workflow.status";
const DEFAULT_TASK_STATUS_QUEUE: &str = "task.status";

pub fn init() -> Result<Config, Error> {
    let args = CliArgs::parse();

    // Load config file if provided
    let config_from_file: Config = if let Some(cfg_path) = args.config {
        let content = fs::read_to_string(&cfg_path)
            .with_context(|| format!("Failed to read config file: {}", cfg_path))?;
        serde_yaml::from_str(&content)
            .with_context(|| format!("Invalid config file format: {}", cfg_path))?
    } else {
        Config::default() // fallback to empty/default config
    };

    // Merge CLI args -> config file -> hardcoded defaults
    let config = Config {
        etcd_uri: args
            .etcd_uri
            .or(Some(config_from_file.etcd_uri))
            .unwrap_or_else(|| DEFAULT_ETCD_URI.to_string()),
        amqp_uri: args
            .amqp_uri
            .or(Some(config_from_file.amqp_uri))
            .unwrap_or_else(|| DEFAULT_AMQP_URI.to_string()),
        workflow_submit_queue: args
            .workflow_submit_queue
            .or(Some(config_from_file.workflow_submit_queue))
            .unwrap_or_else(|| DEFAULT_WORKFLOW_SUBMIT_QUEUE.to_string()),
        task_submit_queue: args
            .task_submit_queue
            .or(Some(config_from_file.task_submit_queue))
            .unwrap_or_else(|| DEFAULT_TASK_SUBMIT_QUEUE.to_string()),
        worker_queue: args
            .worker_queue
            .or(Some(config_from_file.worker_queue))
            .unwrap_or_else(|| DEFAULT_WORKER_QUEUE.to_string()),
        worker_status_queue: args
            .worker_status_queue
            .or(Some(config_from_file.worker_status_queue))
            .unwrap_or_else(|| DEFAULT_WORKER_STATUS_QUEUE.to_string()),
        workflow_status_queue: args
            .workflow_status_queue
            .or(Some(config_from_file.workflow_status_queue))
            .unwrap_or_else(|| DEFAULT_WORKFLOW_STATUS_QUEUE.to_string()),
        task_status_queue: args
            .task_status_queue
            .or(Some(config_from_file.task_status_queue))
            .unwrap_or_else(|| DEFAULT_TASK_STATUS_QUEUE.to_string()),
        ..config_from_file // preserve other fields if any
    };

    Ok(config)
}
