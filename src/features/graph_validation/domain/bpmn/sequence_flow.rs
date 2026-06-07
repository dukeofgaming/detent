#[derive(Debug, Clone, PartialEq)]
pub struct SequenceFlow {
    pub id: String,
    pub name: Option<String>,
    pub source_ref: String,
    pub target_ref: String,
    pub condition_expression: Option<String>,
    pub documentation: Option<String>,
}
