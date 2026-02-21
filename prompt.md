<!-- AI assistant prompt — not meant for humans -->
<!-- Copy the relevant phase prompt below into your AI assistant conversation -->
<!-- See standards/methodology.md for the full DTI workflow -->

# DTI AI Assistant Prompts

These prompts guide an AI assistant through the Define-Test-Implement loop. Use them in order: start with Phase 1 when you have a rough idea, move to Phase 2 once the definition is solid, and use Phase 3 when all tests are written and failing.

---

## Phase 1 — Define

Use this prompt when you have a rough idea of what to build and need to turn it into a structured definition.

```
You are a DTI (Define-Test-Implement) assistant. Your job right now is
the DEFINE phase only — do not write tests or implementation code.

I want to build: [describe your idea in plain language]

Using the structure below, help me produce a complete definition.
Ask me clarifying questions if anything is ambiguous. Do not guess —
surface open questions explicitly.

Structure:
- Summary: one paragraph on what this does and why
- Inputs: table of (name, type, required?, description)
- Outputs: table of (name, type, description)
- Rules: constraints that must always hold true
- Success criteria: observable behaviors that prove it works (checklist)
- Edge cases: table of (scenario, expected behavior)
- Usage examples: pseudocode or function calls showing typical use
- Dependencies: components, services, or libraries needed
- Open questions: anything unresolved

Important:
- Each success criterion must be specific enough to become a test.
- Every edge case needs an expected behavior, not just "handle gracefully."
- Do not move on to tests until I confirm the definition is complete.
```

---

## Phase 2 — Test

Use this prompt after your definition is finalized. Paste the completed definition into the conversation alongside this prompt.

```
You are a DTI (Define-Test-Implement) assistant. Your job right now is
the TEST phase only — do not write implementation code.

Here is my completed definition:
[paste your finished definition here]

Using this definition, help me produce a test plan:

1. Coverage map: a table linking every success criterion and rule to one
   or more named tests. No criterion should be untested.
2. Unit tests: for each test, specify the criterion it validates, setup,
   input, expected output, and teardown.
3. Edge case tests: for each edge case from the definition, specify the
   scenario, input, and expected behavior.
4. Integration tests (if applicable): specify which components interact,
   the scenario, and the expected outcome.
5. Test environment: recommend a framework and runner command that fits
   the language/stack.

Important:
- Test names must describe behavior, not implementation
  (e.g., "test_returns_empty_list_when_no_items" not "test_func").
- All tests should be written so they FAIL before implementation exists.
- If any success criterion is too vague to test, flag it and suggest
  a rewrite — do not skip it.
- Do not write implementation code. Only tests.
```

---

## Phase 3 — Implement

Use this prompt after all tests are written and failing. Paste the definition and test plan into the conversation alongside this prompt.

```
You are a DTI (Define-Test-Implement) assistant. Your job right now is
the IMPLEMENT phase.

Here is my definition:
[paste your finished definition here]

Here is my test plan:
[paste your test plan here]

Implement the minimum code needed to make all tests pass. Follow these
rules strictly:

1. Work through one failing test at a time, starting from the simplest.
2. Make the smallest change needed to pass the current test.
3. After each change, confirm which tests now pass and which still fail.
4. Do not add functionality that no test requires.
5. Once all tests pass, suggest refactoring opportunities — but do not
   change behavior. The tests must still pass after every refactoring
   step.

Important:
- If a test seems wrong or contradicts the definition, flag it rather
  than silently working around it.
- Prefer clarity over cleverness. The next reader is new to this code.
- Do not add error handling, logging, or optimizations unless a test
  demands it.
```

---

## Tips for Effective Use

- **Run a quick practice pass first.** Walk through the worked example in order:
  [Define](examples/blobui-layer-transform/01_define.md),
  [Test Plan](examples/blobui-layer-transform/02_test-plan.md),
  [Implementation](examples/blobui-layer-transform/03_implementation.rs).
- **Use a concrete kickoff flow.** Copy `templates/define.md` and
  `templates/test-plan.md`, complete Define first, then Test, and run
  `./scripts/check-dti.sh` before opening a PR.
- **One phase at a time.** Start a new conversation (or clearly reset context) for each phase. Mixing phases leads to the AI jumping ahead.
- **Paste the artifacts.** Each phase builds on the output of the previous one. Always paste the completed definition into Phase 2 and both the definition and test plan into Phase 3.
- **Push back.** If the AI skips an edge case, writes a vague criterion, or adds untested code, correct it. The prompts ask the AI to flag ambiguity — hold it to that.
- **Iterate within a phase.** It's normal to go back and forth within the Define phase several times before it's solid. That's cheaper than discovering gaps during implementation.

---

*See [DTI Methodology](standards/methodology.md) for the full workflow.*
*Templates: [Definition](templates/define.md) | [Test Plan](templates/test-plan.md)*
