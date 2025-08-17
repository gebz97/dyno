use std::collections::HashMap;

use petgraph::prelude::NodeIndex;
use petgraph::graph::DiGraph;
use serde::{de::{value::Error, Error as _}, Deserialize, Serialize};

use crate::types::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub namespace: String,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
    pub tasks: Vec<Task>,
}


impl Workflow {
  pub fn to_digraph(&self) -> Result<DiGraph<Task, ()>, Error> {
    let mut dag: DiGraph<Task, ()> = DiGraph::new();

    // Map task ID -> NodeIndex. Adjust key field to your model.
    let mut idx: HashMap<String, NodeIndex> = HashMap::new();

    // 1) add all nodes
    for t in &self.tasks {
      let ni = dag.add_node(t.clone()); // Task: Clone
      if idx.insert(t.name.clone(), ni).is_some() {
        return Err(Error::custom(format!("duplicate task id {}", t.name)));
      }
    }

    // 2) add edges using NodeIndex lookups
    for t in &self.tasks {
      let u = *idx.get(&t.name).unwrap();
      for child_name in &t.children { // assume Vec<String>
        let v = *idx.get(child_name)
          .ok_or_else(|| Error::custom(format!("unknown child {}", child_name)))?;
        dag.add_edge(u, v, ());
      }
    }

    Ok(dag)
  }
}
