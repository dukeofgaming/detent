//! Import command implementation - BPMN to MDX conversion

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use detent::bpmn::{parse_bpmn, Process, SequenceFlow};
use detent::mdx::{FlowFrontmatter, NodeFrontmatter, ProcessFrontmatter};

/// Run the import command
pub fn run(bpmn_file: PathBuf, output_directory: PathBuf) -> ExitCode {
    // Read and parse BPMN file
    let xml = match fs::read_to_string(&bpmn_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", bpmn_file.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let definitions = match parse_bpmn(&xml) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Failed to parse BPMN: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_directory) {
        eprintln!(
            "Failed to create output directory {}: {}",
            output_directory.display(),
            e
        );
        return ExitCode::FAILURE;
    }

    // Get process
    let process = match &definitions.process {
        Some(p) => p,
        None => {
            eprintln!("No process found in BPMN file");
            return ExitCode::FAILURE;
        }
    };

    // Generate MDX files
    let mut generated_count = 0;

    // Generate process MDX
    if let Err(e) = generate_process_mdx(process, &output_directory) {
        eprintln!("Failed to generate process MDX: {}", e);
        return ExitCode::FAILURE;
    }
    generated_count += 1;

    // Generate node MDX files
    for start_event in &process.start_events {
        if let Err(e) = generate_start_event_mdx(start_event, &output_directory) {
            eprintln!("Failed to generate startEvent MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for end_event in &process.end_events {
        if let Err(e) = generate_end_event_mdx(end_event, &output_directory) {
            eprintln!("Failed to generate endEvent MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for task in &process.tasks {
        if let Err(e) = generate_task_mdx(task, &output_directory) {
            eprintln!("Failed to generate task MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    // Generate sequence flow MDX files
    for flow in &process.sequence_flows {
        if let Err(e) = generate_sequence_flow_mdx(flow, &output_directory) {
            eprintln!("Failed to generate sequenceFlow MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    println!(
        "Generated {} MDX files in {}",
        generated_count,
        output_directory.display()
    );
    ExitCode::SUCCESS
}

/// Generate process MDX file
fn generate_process_mdx(process: &Process, output_dir: &PathBuf) -> Result<(), std::io::Error> {
    let frontmatter = ProcessFrontmatter {
        id: process.id.clone(),
        process_type: "bpmn:process".to_string(),
        name: process.name.clone(),
        is_executable: process.is_executable,
        process_type_attr: process.process_type.clone(),
        documentation: process.documentation.as_ref().map(|d| d.text.clone()),
        flow_elements: collect_flow_element_ids(process),
    };

    let yaml = serde_yaml::to_string(&frontmatter).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("YAML error: {}", e))
    })?;

    let mdx_content = format!("---\n{}---\n", yaml);

    let file_path = output_dir.join("_process.mdx");
    fs::write(file_path, mdx_content)
}

/// Collect all flow element IDs for the process frontmatter
fn collect_flow_element_ids(process: &Process) -> Vec<String> {
    let mut ids = Vec::new();

    for e in &process.start_events {
        ids.push(e.id.clone());
    }
    for e in &process.tasks {
        ids.push(e.id.clone());
    }
    for e in &process.end_events {
        ids.push(e.id.clone());
    }
    for e in &process.sequence_flows {
        ids.push(e.id.clone());
    }

    ids
}

/// Generate startEvent MDX file
fn generate_start_event_mdx(
    event: &detent::bpmn::StartEvent,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let frontmatter = NodeFrontmatter {
        id: event.id.clone(),
        node_type: "bpmn:startEvent".to_string(),
        name: event.name.clone(),
        incoming: vec![],
        outgoing: event.outgoing.clone(),
        documentation: event.documentation.as_ref().map(|d| d.text.clone()),
        implementation: None,
        service_ref: None,
        script_format: None,
        script: None,
        gateway_direction: None,
        default: None,
        conditions: None,
        io_spec: None,
        retry: None,
    };

    write_node_mdx(&event.id, &frontmatter, output_dir)
}

/// Generate endEvent MDX file
fn generate_end_event_mdx(
    event: &detent::bpmn::EndEvent,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let frontmatter = NodeFrontmatter {
        id: event.id.clone(),
        node_type: "bpmn:endEvent".to_string(),
        name: event.name.clone(),
        incoming: event.incoming.clone(),
        outgoing: vec![],
        documentation: event.documentation.as_ref().map(|d| d.text.clone()),
        implementation: None,
        service_ref: None,
        script_format: None,
        script: None,
        gateway_direction: None,
        default: None,
        conditions: None,
        io_spec: None,
        retry: None,
    };

    write_node_mdx(&event.id, &frontmatter, output_dir)
}

/// Generate task MDX file
fn generate_task_mdx(
    task: &detent::bpmn::Task,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let frontmatter = NodeFrontmatter {
        id: task.id.clone(),
        node_type: "bpmn:task".to_string(),
        name: task.name.clone(),
        incoming: task.incoming.clone(),
        outgoing: task.outgoing.clone(),
        documentation: task.documentation.as_ref().map(|d| d.text.clone()),
        implementation: None,
        service_ref: None,
        script_format: None,
        script: None,
        gateway_direction: None,
        default: None,
        conditions: None,
        io_spec: None,
        retry: None,
    };

    write_node_mdx(&task.id, &frontmatter, output_dir)
}

/// Generate sequenceFlow MDX file
fn generate_sequence_flow_mdx(
    flow: &SequenceFlow,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let frontmatter = FlowFrontmatter {
        id: flow.id.clone(),
        flow_type: "bpmn:sequenceFlow".to_string(),
        name: flow.name.clone(),
        source_ref: flow.source_ref.clone(),
        target_ref: flow.target_ref.clone(),
        condition_expression: flow.condition_expression.as_ref().map(|c| c.expression.clone()),
    };

    let yaml = serde_yaml::to_string(&frontmatter).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("YAML error: {}", e))
    })?;

    let mdx_content = format!("---\n{}---\n", yaml);

    let file_path = output_dir.join(format!("{}.mdx", flow.id));
    fs::write(file_path, mdx_content)
}

/// Write a node MDX file
fn write_node_mdx(
    id: &str,
    frontmatter: &NodeFrontmatter,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let yaml = serde_yaml::to_string(frontmatter).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("YAML error: {}", e))
    })?;

    let mdx_content = format!("---\n{}---\n", yaml);

    let file_path = output_dir.join(format!("{}.mdx", id));
    fs::write(file_path, mdx_content)
}
