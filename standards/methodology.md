# Define-Test-Implement (DTI) Methodology

This is the canonical source for DTI workflow rules in this repository. If any other document
conflicts with this file, this file is the source of truth.

## Purpose

DTI separates three concerns so humans and AI assistants stay aligned:

1. `Define`: decide exactly what should happen.
2. `Test`: encode that definition as executable checks.
3. `Implement`: write the minimum code to satisfy those checks.

## Workflow Summary

| Phase | Goal | Required Artifact(s) | Exit Criteria |
|:------|:-----|:---------------------|:--------------|
| Define | Turn an idea into explicit requirements | `define.md` | Inputs/outputs/rules are clear, success criteria are testable, and open questions are resolved |
| Test | Translate requirements into tests | `test-plan.md` plus failing tests | Every criterion maps to at least one test, and tests fail before implementation |
| Implement | Make tests pass with minimal behavior | Implementation code and passing tests | All tests pass, then refactoring preserves behavior |

## Phase Rules

### 1. Define

- Produce a definition using `templates/define.md`.
- Capture inputs, outputs, invariants, success criteria, and edge cases.
- Keep success criteria observable and specific enough to test.
- Do not write implementation code in this phase.

### 2. Test

- Derive tests from the completed definition using `templates/test-plan.md`.
- Map every success criterion and rule to named tests.
- Include edge-case coverage from the definition.
- Ensure tests are written and failing before implementation starts.
- Do not write implementation code in this phase.

### 3. Implement

- Implement one failing test at a time, starting with the simplest.
- Make the smallest change needed to pass the current test.
- Do not add behavior that no test requires.
- Refactor only after all tests pass, without changing behavior.

## PR Requirements

Every PR should include or link to:

- A completed Definition artifact
- A completed Test Plan artifact
- Evidence that implementation follows the defined scope

See `.github/pull_request_template.md` and `standards/contributing.md` for process details.

## Tooling

- Phase prompts: `prompt.md`
- Artifact templates: `templates/define.md`, `templates/test-plan.md`
- Structure check: `./scripts/check-dti.sh`
