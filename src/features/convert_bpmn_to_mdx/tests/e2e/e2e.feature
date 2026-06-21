Feature: Convert BPMN to MDX e2e tests
  As a user
  I want the detent CLI to compile, import, and validate
  So that I can work with BPMN and MDX from the shell

  Scenario: Compile help
    When I run detent with args "compile --help"
    Then the command succeeds
    And stdout contains "MDX files"

  Scenario: Compile defaults to current directory
    Given a fresh temp workspace
    And a minimal workflow in the temp workspace
    When I run detent with args "compile"
    Then the command succeeds

  Scenario: Compile writes output named after directory
    Given a fresh temp workspace
    And a minimal workflow in subdirectory "my_workflow"
    When I run detent with args "compile my_workflow"
    Then the command succeeds
    And the file "my_workflow.bpmn" exists in the temp workspace
    And the file "my_workflow.bpmn" contains "definitions"
    And the file "my_workflow.bpmn" contains "start_1"

  Scenario: Compile output flag overrides derived name
    Given a fresh temp workspace
    And a minimal workflow in subdirectory "my_workflow"
    And an output file path "custom.bpmn" in the temp workspace
    When I run detent with args "compile my_workflow --output custom.bpmn"
    Then the command succeeds
    And the file "custom.bpmn" exists in the temp workspace
    And the file "my_workflow.bpmn" does not exist in the temp workspace

  Scenario: Compile missing directory
    When I run detent with args "compile nonexistent-dir"
    Then the command fails
    And stderr contains "Failed to read"

  Scenario: Compile produces BPMN XML file
    Given a fresh temp workspace
    And an output file path "output.bpmn" in the temp workspace
    When I compile hello-world MDX to the output file
    Then the command succeeds
    And the file "output.bpmn" exists in the temp workspace
    And the file "output.bpmn" contains "definitions"
    And the file "output.bpmn" contains "process"

  Scenario: Compile output contains all elements
    Given a fresh temp workspace
    And an output file path "output.bpmn" in the temp workspace
    When I compile hello-world MDX to the output file
    Then the command succeeds
    And the file "output.bpmn" contains "_1E892844-423C-464F-ADC4-22F1EC73851B"
    And the file "output.bpmn" contains "_808AA40C-EAA1-40C4-A2DC-27000FBF1866"
    And the file "output.bpmn" contains "_D3F6E97D-7783-492C-98CE-57EC815D304C"

  Scenario: Compile writes to stdout by default
    Given a fresh temp workspace
    And a minimal workflow in the temp workspace
    When I compile all MDX files in the temp workspace to stdout
    Then the command succeeds
    And stdout contains "definitions"
    And stdout contains "start_1"

  Scenario: Compile ignores non-MDX files
    Given a fresh temp workspace
    And an output file path "output.bpmn" in the temp workspace
    When I compile hello-world MDX to the output file
    Then the command succeeds

  Scenario: Compile with individual files
    Given a fresh temp workspace
    And an output file path "output.bpmn" in the temp workspace
    When I compile hello-world MDX files individually to the output file
    Then the command succeeds
    And the file "output.bpmn" contains "definitions"
    And the file "output.bpmn" contains "process"

  Scenario: Compile rejects non-MDX file
    Given a fresh temp workspace
    And a non-MDX file "data.txt" in the temp workspace
    When I run detent with args "compile data.txt"
    Then the command fails
    And stderr contains "Not an .mdx file"

  Scenario: Compile tolerates dangling flow target
    Given a dangling-flow compile workspace
    When I run the compile CLI directly on the dangling workflow
    Then the command succeeds
    And the dangling target id appears in compiled output

  Scenario: Import help
    When I run detent with args "import --help"
    Then the command succeeds
    And stdout contains "BPMN XML file"

  Scenario: Import generates all expected files
    Given a fresh temp workspace
    When I import hello-world BPMN to the temp output directory
    Then the command succeeds
    And all expected hello-world MDX files exist

  Scenario: Import frontmatter matches reference
    Given a fresh temp workspace
    When I import hello-world BPMN to the temp output directory
    Then the command succeeds
    And imported hello-world frontmatter matches reference

  Scenario: Imported tdd fixture compiles
    Given a fresh temp workspace
    When I import tdd BPMN to the temp output directory
    Then the command succeeds
    And the temp workspace contains "Process_DeveloperWorkflow.mdx"
    When I compile the imported tdd MDX to BPMN in the temp workspace
    Then the command succeeds
    And roundtrip BPMN contains tdd task ids

  Scenario: Import missing BPMN file
    When I run detent with args "import nonexistent.bpmn"
    Then the command fails
    And stderr contains "Failed to read"

  Scenario: Import requires BPMN file
    When I run detent with args "import"
    Then the command fails

  Scenario: Validate help
    When I run detent with args "validate --help"
    Then the command succeeds
    And stdout contains "BPMN"

  Scenario: Validate BPMN file
    Given hello-world BPMN file path
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX start event
    Given hello-world MDX file path for "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX task
    Given hello-world MDX file path for "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX end event
    Given hello-world MDX file path for "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx"
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX sequence flow
    Given hello-world MDX file path for "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx"
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate multiple files
    Given a fresh temp workspace
    When I validate hello-world BPMN and start event MDX
    Then the command succeeds
    And stdout contains "✓" 2 times

  Scenario: Validate missing file
    When I run detent with args "validate nonexistent.bpmn"
    Then the command fails
    And stderr contains "✗"

  Scenario: Validate unknown extension
    When I validate hello-world task MDX and Cargo.toml
    Then the command fails
    And stderr contains "Unknown file type"

  Scenario: Validate requires files
    When I run detent with args "validate"
    Then the command fails

  Scenario: Validate tolerates dangling flow target
    Given a fresh temp workspace
    And a dangling-target hello-world BPMN in the temp workspace
    When I run validate directly on the dangling BPMN
    Then direct validate succeeds

  Scenario: Validate MDX exclusive gateway
    Given a fresh temp workspace
    And a temp exclusive gateway MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX parallel gateway
    Given a fresh temp workspace
    And a temp parallel gateway MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX service task
    Given a fresh temp workspace
    And a temp service task MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX script task
    Given a fresh temp workspace
    And a temp script task MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX manual task
    Given a fresh temp workspace
    And a temp manual task MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX user task
    Given a fresh temp workspace
    And a temp user task MDX file
    When I validate the prepared file with detent
    Then the command succeeds
    And stdout contains "✓"

  Scenario: Validate MDX rejects gateway without id
    Given a fresh temp workspace
    And a temp gateway MDX file without id
    When I validate the prepared file with detent
    Then the command fails
    And stderr contains "ExclusiveGateway must have an id"
