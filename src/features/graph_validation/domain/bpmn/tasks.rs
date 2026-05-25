#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: String,
    pub name: Option<String>,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceTask {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptTask {
    pub id: String,
}
