# detent
A stepper workflow engine with BPMN support, written in Rust and compiled to WebAssembly for use in the browser.

## Design philosophy

- **Local First**: Many BPMN execution engines require infrastructure to run, detent is self-contained as a single binary that can run anywhere, including the browser via WebAssembly.

- **CLI-first**: Designed with a command-line interface in mind, making it easy to integrate into scripts and automation workflows.

- **Workflow

- **BPMN 2.0 Support**: Full support for BPMN 2.0 XML, including complex features like event subprocesses, compensation, and multi-instance tasks. Use your favorite BPMN modeling tool and export to XML for execution.
