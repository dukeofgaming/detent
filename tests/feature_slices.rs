#[path = "../src/features/convert_bpmn_to_mdx/tests/world.rs"]
mod convert_bpmn_to_mdx;

#[cfg(feature = "graph-validation")]
#[path = "../src/features/graph_validation/tests/world.rs"]
mod graph_validation;

#[test]
fn convert_bpmn_to_mdx() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    assert!(!rt.block_on(convert_bpmn_to_mdx::run()));
}

#[cfg(feature = "graph-validation")]
#[test]
fn graph_validation() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    assert!(!rt.block_on(graph_validation::run()));
}
