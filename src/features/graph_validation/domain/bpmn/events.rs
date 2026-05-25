#[derive(Debug, Clone, PartialEq)]
pub struct StartEvent {
    pub id: String,
    pub name: Option<String>,
    pub outgoing: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndEvent {
    pub id: String,
    pub name: Option<String>,
    pub incoming: Vec<String>,
    pub documentation: Option<String>,
}
