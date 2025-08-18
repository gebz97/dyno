use std::time::Duration;

use etcd_client::ConnectOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // AMQP Configurations
    pub amqp_uri: String,
    pub workflow_submit_queue: String,
    pub task_submit_queue: String,
    pub worker_queue: String,
    pub worker_status_queue: String,
    pub workflow_status_queue: String,
    pub task_status_queue: String,

    // ETCD Configurations
    pub etcd_uri: String,
    pub etcd_user: Option<(String, String)>,
    /// HTTP2 keep-alive: (keep_alive_interval, keep_alive_timeout)
    pub keep_alive: Option<(Duration, Duration)>,
    /// Whether send keep alive pings even there are no active streams.
    pub keep_alive_while_idle: bool,
    /// Apply a timeout to each gRPC request.
    pub timeout: Option<Duration>,
    /// Apply a timeout to connecting to the endpoint.
    pub connect_timeout: Option<Duration>,
    /// TCP keepalive.
    pub tcp_keepalive: Option<Duration>,
    //#[cfg(feature = "tls")]
    //tls: Option<TlsOptions>,
    //#[cfg(feature = "tls-openssl")]
    //otls: Option<OpenSslResult<OpenSslConnector>>,
    /// Require a leader to be present for the operation to complete.
    pub require_leader: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            amqp_uri: String::from("amqp://guest:guest@localhost:5672"),
            workflow_submit_queue: String::from("worflow.submit"),
            task_submit_queue: String::from("task.submit"),
            worker_queue: String::from("worker.queue"),
            worker_status_queue: String::from("worker.status"),
            workflow_status_queue: String::from("workflow.status"),
            task_status_queue: String::from("task.status"),
            etcd_uri: String::from("http://localhost:2379"),
            etcd_user: None,
            keep_alive: None,
            keep_alive_while_idle: true,
            timeout: None,
            connect_timeout: Some(Duration::from_secs(60)),
            tcp_keepalive: None,
            require_leader: false,
        }
    }

    pub fn get_etcd_options(&self) -> ConnectOptions {
        let mut options = ConnectOptions::new();

        // Set user if provided
        if let Some((ref username, ref password)) = self.etcd_user {
            options = options.with_user(username, password);
        }

        // Set connect timeout
        options = options.with_connect_timeout(self.connect_timeout.unwrap());

        // Set TCP keepalive
        if let Some(tcp) = self.tcp_keepalive {
            options = options.with_tcp_keepalive(tcp);
        }

        // Set gRPC keep-alive if provided
        if let Some((interval, timeout)) = self.keep_alive {
            options = options.with_keep_alive(interval, timeout);
        }

        // Set keep-alive while idle
        options = options.with_keep_alive_while_idle(self.keep_alive_while_idle);

        // Set require leader
        options = options.with_require_leader(self.require_leader);

        options
    }
}
