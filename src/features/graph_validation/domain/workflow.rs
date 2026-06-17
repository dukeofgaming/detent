#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Start,
    End,
    Action,
    Gateway,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub id: String,
    pub nodes: Vec<Node>,
    pub flows: Vec<Flow>,
}
