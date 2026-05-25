#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub name: Option<String>,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Clone)]
pub struct ServiceTask {
    pub id: String,
}

#[derive(Clone)]
pub struct ScriptTask {
    pub id: String,
}
