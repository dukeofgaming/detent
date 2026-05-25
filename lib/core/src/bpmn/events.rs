#[derive(Clone)]
pub struct StartEvent {
    pub id: String,
    pub name: Option<String>,
    pub outgoing: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Clone)]
pub struct EndEvent {
    pub id: String,
    pub name: Option<String>,
    pub incoming: Vec<String>,
    pub documentation: Option<String>,
}
