Feature: Convert BPMN to MDX functional tests
  As a developer
  I want use-case orchestration between BPMN and MDX
  So that processes round-trip in-process

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
    When I parse the BPMN to definitions
    When I attempt to import the parsed definitions to MDX
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

  Scenario: Hello-world MDX compiles
    Given the hello-world MDX fixtures
    When I compile the MDX inputs to definitions
    Then the compiled task name is "Hello World"

  Scenario: MDX compiles and imports back
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    When I import the compiled definitions to MDX
    Then 6 MDX outputs are produced

  Scenario: Service task, script task, and gateway compile
    Given an MDX input set with a service task, a script task, and an exclusive gateway
    When I compile the MDX inputs to definitions
    Then the process has 1 service task, 1 script task, and 1 exclusive gateway

  Scenario: Condition expression survives import
    Given in-memory definitions whose sequence flow carries a condition expression
    When I import the parsed definitions to MDX
    Then the flow_1 MDX output contains the condition "amount > 100"

  Scenario: Process metadata MDX controls compiled definitions
    Given process metadata MDX inputs
    When I compile the process metadata inputs
    Then compiled definitions carry process and diagram metadata

  Scenario: Imports include process and definitions metadata
    Given the tdd BPMN fixture
    When I import it to MDX
    Then tdd import outputs include rich process metadata

  Scenario Outline: Imported fixture frontmatter is stable after compile roundtrip
    Given the <fixture> BPMN fixture
    When I import the fixture and compile-roundtrip its frontmatter
    Then imported frontmatter matches after compile roundtrip

    Examples:
      | fixture     |
      | hello-world |
      | blog-post   |
      | tdd         |
