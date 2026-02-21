# TODO List: DTI Documentation Toolkit

## Current Status

- ✅ Created DTI methodology and core standards
- ✅ Added Define and Test Plan templates (templates/define.md, templates/test-plan.md)
- ✅ Genericized todo.md and overview.md templates with placeholders
- ✅ Updated contributing.md to enforce the DTI workflow
- ✅ Expanded prompt.md with structured AI prompts for each DTI phase

## Next Steps

### 1. Connect Conventional Commits to DTI

- [ ] Add a project-specific section to [conventional_commits.md](standards/conventional_commits.md) mapping commit types to DTI phases (e.g., `docs:` for Define artifacts, `test:` for Test phase work, `feat:`/`fix:` for Implementation)
- [ ] Include examples of DTI-aware commit messages (e.g., `docs(define): add requirements for user authentication`)
- [ ] Trim or separate the verbatim spec text so project guidance is easy to find

### 2. Flesh Out Documentation Standards

- [ ] Add before/after examples to [documentation.md](standards/documentation.md) showing good vs. bad documentation (vague comment vs. clear one, bare function signature vs. one with usage examples)
- [ ] Expand the Best Practices section with concrete illustrations
- [ ] Connect documentation standards back to DTI — e.g., what makes a good Definition artifact, what makes a good test description

### 3. Add a License

- [ ] Choose a license (MIT or Apache 2.0 are natural fits for a shareable toolkit)
- [ ] Add a LICENSE file to the repository root
- [ ] Update the License section in [readme.md](readme.md)

### 4. Tighten the README Intro

- [ ] Rewrite the opening paragraph to lead with DTI as the differentiator, not "comprehensive documentation toolkit"
- [ ] Frame the repo as: DTI is a structured workflow for AI-assisted development; this toolkit provides the templates, standards, and prompts to use it
- [ ] Ensure the methodology is positioned as the foundational concept, not just one of several standards

### 5. Add a Worked Example

- [ ] Create an `examples/` directory
- [ ] Walk a small feature end-to-end through all three DTI phases: rough idea to completed definition, definition to test plan, test plan to passing implementation
- [ ] Use a simple, language-agnostic example (e.g., a string utility or data validator) so it's accessible to any audience
- [ ] Link to the example from the methodology, the README, and the prompt.md tips section

## Priority Items

1. Conventional commits mapping — low effort, high cohesion gain
2. Documentation standards examples — makes the standards actionable
3. License — legal prerequisite for anyone reusing the toolkit
4. README intro rewrite — first impression for new visitors
5. Worked example — highest effort but biggest impact for adoption

## Notes

- All new work should follow the [DTI methodology](standards/methodology.md).
- The generic [todo.md template](templates/todo.md) remains unchanged — this file is the project-specific backlog.

---

*See [DTI Methodology](standards/methodology.md) for the full workflow.*
