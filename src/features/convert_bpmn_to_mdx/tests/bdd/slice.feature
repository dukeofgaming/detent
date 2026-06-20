Feature: Convert BPMN to MDX
  As a developer
  I want to round-trip between BPMN XML and MDX flow-element files
  So that I can author processes in MDX and emit valid BPMN

  Scenario: Hello-world imports to 10 MDX
    Given the hello-world BPMN fixture
    When I import it to MDX
    Then 10 MDX outputs are produced
    And each output contains a frontmatter block
    And each output contains its BPMN type

  Scenario: Minimal MDX compiles to process
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    Then a process is produced with 1 start event, 1 task, 1 end event, and 2 sequence flows

  Scenario: Empty input rejected
    Given no MDX inputs
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Missing type rejected
    Given an MDX input whose frontmatter omits the type field
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Dangling flow tolerated
    Given an MDX input set where a sequence flow targets a non-existent node
    When I compile the MDX inputs to definitions
    Then compilation succeeds
    And the resulting process contains the dangling target id

  Scenario: Missing frontmatter rejected
    Given an MDX input with no frontmatter
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Unknown type rejected
    Given an MDX input with unknown element type
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Import rejects no process
    Given a BPMN definition with no process
    When I attempt to import the BPMN to MDX
    Then import fails

  Scenario: Import filenames match IDs
    Given in-memory definitions with start, task, end, and two flows
    When I import the parsed definitions to MDX
    Then one output filename is "start_1.mdx"
    And one output filename is "task_1.mdx"
    And one output filename is "end_1.mdx"
    And one output filename is "flow_1.mdx"
    And one output filename is "flow_2.mdx"

  Scenario: Frontmatter has no XML artifacts
    Given in-memory definitions with start, task, end, and two flows
    When I import the parsed definitions to MDX
    Then no output contains '@' or '$text' in its frontmatter

  Scenario: Hello-world BPMN metadata
    Given the hello-world BPMN fixture
    When I parse the BPMN to definitions
    Then the process id is "hello_world"
    And the process name is "hello-world"
    And the process is executable and public
    And the process documentation is "This is a hello world activity"
    And the definitions were exported by "jBPM Process Modeler" version "2.0"
    And the definitions target namespace is "http://www.omg.org/bpmn20"

  Scenario: Hello-world MDX compiles
    Given the hello-world MDX fixtures
    When I compile the MDX inputs to definitions
    Then the compiled task name is "Hello World"

  Scenario: MDX compiles and imports back
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    Then importing the compiled definitions produces 5 MDX outputs

  Scenario: Service task, script task, and gateway compile
    Given an MDX input set with a service task, a script task, and an exclusive gateway
    When I compile the MDX inputs to definitions
    Then the process has 1 service task, 1 script task, and 1 exclusive gateway

  Scenario: Condition expression survives import
    Given in-memory definitions whose sequence flow carries a condition expression
    When I import the parsed definitions to MDX
    Then the flow_1 MDX output contains the condition "amount > 100"

  Scenario Outline: BPMN fixture imports to MDX
    Given the <fixture> BPMN fixture
    When I import it to MDX
    Then <count> MDX outputs are produced
    And each output contains a frontmatter block
    And each output contains its BPMN type

    Examples:
      | fixture      | count |
      | hello-world  | 10    |
      | blog-post    | 48    |
      | tdd          | 48    |
