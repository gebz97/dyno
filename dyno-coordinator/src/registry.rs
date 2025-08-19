#![allow(dead_code)]
use anyhow::Error;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Registry {
    pub step_state_map: Arc<RwLock<HashMap<String, String>>>,
    pub task_state_map: Arc<RwLock<HashMap<String, String>>>,
    pub workflow_state_map: Arc<RwLock<HashMap<String, String>>>,
}

impl Registry {
    pub fn init() -> Self {
        Registry {
            step_state_map: Arc::new(RwLock::new(HashMap::new())),
            task_state_map: Arc::new(RwLock::new(HashMap::new())),
            workflow_state_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn update_step(&self, key: String, value: String) -> Result<(), Error> {
        let mut map = self
            .step_state_map
            .write()
            .map_err(|_| Error::msg("step_state_map poisoned"))?;
        map.insert(key, value);
        Ok(())
    }

    pub fn update_task(&self, key: String, value: String) -> Result<(), Error> {
        let mut map = self
            .task_state_map
            .write()
            .map_err(|_| Error::msg("task_state_map poisoned"))?;
        map.insert(key, value);
        Ok(())
    }

    pub fn update_workflow(&self, key: String, value: String) -> Result<(), Error> {
        let mut map = self
            .workflow_state_map
            .write()
            .map_err(|_| Error::msg("workflow_state_map poisoned"))?;
        map.insert(key, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;
    use std::thread;
    use std::time::Duration;
    use tokio::sync::oneshot;

    #[test]
    fn concurrent_updates_to_different_keys() {
        let registry = Arc::new(Registry::init());
        registry.update_step("step1".into(), "0".into()).unwrap();
        registry.update_step("step2".into(), "0".into()).unwrap();

        let mut handles = vec![];

        for i in 0..10 {
            let reg_clone = Arc::clone(&registry);
            let key = if i % 2 == 0 {
                "step1".to_string()
            } else {
                "step2".to_string()
            };
            let handle = thread::spawn(move || {
                // read current value
                let current = {
                    let map = reg_clone.step_state_map.read().unwrap();
                    map.get(&key).unwrap().clone()
                };
                let new_val = (current.parse::<i32>().unwrap() + 1).to_string();
                reg_clone.update_step(key, new_val).unwrap();
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        let map = registry.step_state_map.read().unwrap();
        let v1 = map.get("step1").unwrap().clone();
        let v2 = map.get("step2").unwrap().clone();
        assert_eq!(v1, "5");
        assert_eq!(v2, "5");
    }

    #[tokio::test]
    async fn greedy_thread_blocks_same_key() {
        let registry = Registry::init();
        registry.update_step("step1".into(), "0".into()).unwrap();

        let (tx, rx) = oneshot::channel();

        // Hold the write lock on the map
        let reg_clone = registry.step_state_map.clone();
        let handle = tokio::task::spawn_blocking(move || {
            let mut map = reg_clone.write().unwrap();
            let entry = map.get_mut("step1").unwrap();
            *entry = "locked".into();
            tx.send(()).unwrap();
            std::thread::sleep(Duration::from_secs(3));
        });

        rx.await.unwrap();

        // While the writer holds the lock, we can't read
        assert!(
            registry.step_state_map.try_read().is_err(),
            "Contender must not acquire lock while greedy holds it"
        );

        handle.await.unwrap();
    }

    #[test]
    fn greedy_thread_doesnt_block_other_keys() {
        let registry = Arc::new(Registry::init());
        registry.update_step("step1".into(), "0".into()).unwrap();
        registry.update_step("step2".into(), "0".into()).unwrap();

        let barrier = Arc::new(Barrier::new(2));

        // Greedy writer blocks map
        let reg_clone = Arc::clone(&registry);
        let barrier_clone = Arc::clone(&barrier);
        let greedy_handle = thread::spawn(move || {
            let mut map = reg_clone.step_state_map.write().unwrap();
            let entry = map.get_mut("step1").unwrap();
            *entry = "0".into();
            barrier_clone.wait();
            std::thread::sleep(Duration::from_secs(2));
        });

        let reg_clone2 = Arc::clone(&registry);
        let barrier_clone2 = Arc::clone(&barrier);
        let updater_handle = thread::spawn(move || {
            barrier_clone2.wait();
            // Will block until greedy releases lock
            reg_clone2
                .update_step("step2".into(), "1".into())
                .unwrap();
        });

        greedy_handle.join().unwrap();
        updater_handle.join().unwrap();

        let map = registry.step_state_map.read().unwrap();
        let v1 = map.get("step1").unwrap().clone();
        let v2 = map.get("step2").unwrap().clone();
        assert_eq!(v1, "0");
        assert_eq!(v2, "1");
    }
}
