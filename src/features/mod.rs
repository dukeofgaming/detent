#[cfg(test)]
mod tests {
    #[test]
    fn feature_slices_expose_expected_entrypoints() {
        let _ = std::any::type_name::<
            crate::features::convert_bpmn_to_mdx::use_cases::compile::CompileError,
        >();
        let _ = std::any::type_name::<
            crate::features::graph_validation::domain::graph::Graph<'static>,
        >();
    }
}

pub mod convert_bpmn_to_mdx;
pub mod graph_validation;
