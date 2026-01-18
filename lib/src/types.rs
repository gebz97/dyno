use std::collections::HashMap;

use serde_json::Value;

pub type Vars = HashMap<String, Value>;

#[derive(Debug)]
pub struct Host {
    pub id: String,
    pub address: String,
    pub vars: Vars,
    pub groups: Vec<String>,
}

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub vars: Vars,
    pub hosts: Vec<String>,
    pub children: Vec<String>,
}

#[derive(Debug)]
pub struct Inventory {
    pub hosts: HashMap<String, Host>,
    pub groups: HashMap<String, Group>,
}

#[derive(Debug)]
pub struct VarContext {
    pub host_vars: Vars,
    pub group_vars: Vec<Vars>,
    pub play_vars: Vars,
    pub extra_vars: Vars,
}

#[derive(Debug)]
pub struct Play {
    pub name: Option<String>,
    pub hosts: HostSelector,
    pub vars: Vars,
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub enum HostSelector {
    All,
    Host(String),
    Group(String),
    Pattern(String),
}

#[derive(Debug)]
pub struct Task {
    pub name: Option<String>,
    pub module: ModuleCall,
    pub when: Option<Expr>,
    pub register: Option<String>,
    pub changed_when: Option<Expr>,
    pub failed_when: Option<Expr>,
}

#[derive(Debug)]
pub struct ModuleCall {
    pub module: String,
    pub args: Vars,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Bool(bool),
    Number(f64),
    String(String),
    Var(VarRef),
    Not(Box<Expr>),

    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    Test {
        expr: Box<Expr>,
        test: TestOp,
    },

    FilterChain {
        base: Box<Expr>,
        chain: Vec<FilterInvocation>,
    },
}

#[derive(Debug, Clone)]
pub struct VarRef {
    pub root: String,
    pub path: Vec<VarPath>,
}

#[derive(Debug, Clone)]
pub enum VarPath {
    Attr(String),
    Index(Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy)]
pub enum TestOp {
    Defined,
    Undefined,
    None,
    True,
    False,
    Failed,
    Changed,
}

#[derive(Debug, Clone)]
pub struct FilterInvocation {
    pub name: String,
    pub args: Vec<Expr>,
}
