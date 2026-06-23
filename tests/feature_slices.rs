#[path = "../src/features/bpmn_mdx_transpiler/tests/world.rs"]
mod bpmn_mdx_transpiler;

#[cfg(feature = "graph-validation")]
#[path = "../src/features/graph_validation/tests/world.rs"]
mod graph_validation;

#[test]
fn bpmn_mdx_transpiler() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    assert!(!rt.block_on(bpmn_mdx_transpiler::run()));
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
