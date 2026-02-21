# Test Plan: [Component/Feature Name]

*Derived from: [define.md](./define.md) for [Component/Feature Name]*

## Coverage Map

Each row links a success criterion from the Define phase to one or more concrete tests. No criterion should be left untested; no test should exist without a corresponding criterion.

| # | Success Criterion | Test Name | Status |
|:--|:------------------|:----------|:-------|
| 1 | [Criterion from define.md] | [test_descriptive_name] | [ ] |
| 2 | | | [ ] |
| 3 | | | [ ] |

## Unit Tests

Tests for individual behaviors in isolation.

### [test_descriptive_name]

- **Criterion:** [Which success criterion or rule this validates]
- **Setup:** [Preconditions, fixtures, or mocks needed]
- **Input:** [What is passed in]
- **Expected output:** [What should come back]
- **Teardown:** [Cleanup steps, if any]

### [test_descriptive_name]

- **Criterion:** [Which success criterion or rule this validates]
- **Setup:**
- **Input:**
- **Expected output:**
- **Teardown:**

## Edge Case Tests

Tests derived from the edge cases identified in the Define phase.

### [test_edge_case_name]

- **Scenario:** [The edge case from define.md]
- **Input:** [Boundary or unusual input]
- **Expected behavior:** [How the component should respond]

### [test_edge_case_name]

- **Scenario:**
- **Input:**
- **Expected behavior:**

## Integration Tests

Tests that verify this component works correctly with its dependencies.

### [test_integration_name]

- **Components involved:** [Which pieces interact]
- **Setup:** [Environment, services, or data required]
- **Scenario:** [What happens end-to-end]
- **Expected outcome:** [Observable result]

## Test Environment

- **Framework:** [e.g., pytest, Jest, Go testing]
- **Runner command:** `[e.g., npm test, pytest tests/]`
- **Fixtures or seed data:** [Location or description]

## Notes

- All tests should be written and failing *before* any implementation begins.
- Test names should describe behavior, not implementation (e.g., `test_returns_empty_list_when_no_items` not `test_function_x`).

---

*Once every success criterion has a corresponding test and all tests are written, proceed to the Implement phase.*
*See [DTI Methodology](../standards/methodology.md) for the full workflow.*
