# Define-Test-Implement (DTI) Documentation Toolkit

[![Docs Quality](https://github.com/michaelgrilo/dti/actions/workflows/docs-quality.yml/badge.svg)](https://github.com/michaelgrilo/dti/actions/workflows/docs-quality.yml)
[![Documentation](https://img.shields.io/badge/docs-standards-blue)](standards/documentation.md)

A structured workflow for AI-assisted development, built around the Define-Test-Implement loop. This toolkit provides the templates, standards, and AI prompts to put it into practice.

## 📚 Contents

- [The DTI Workflow](#the-dti-workflow)
  - [Phase 1 – Define ✅](#phase-1--define-)
  - [Phase 2 – Test 🧪](#phase-2--test-)
  - [Phase 3 – Implement ⚙️](#phase-3--implement-️)
- [Directory Structure](#directory-structure)
- [Start Here (15 Minutes)](#start-here-15-minutes)
- [Getting Started](#getting-started)
- [Standards](#standards)
- [Templates](#templates)
- [Contributing](#contributing)
- [Changelog](#changelog)

## The DTI Workflow

**Define-Test-Implement (DTI)** is a structured workflow that guides developers—especially AI-assisted beginners—from idea to working code in three clear phases:

1. **Define** ✅ – write down exactly what you need to build
2. **Test** 🧪 – write automated tests from your definition checklist
3. **Implement** ⚙️ – write the minimum code needed to make your tests pass

By separating concerns this way, DTI lowers cognitive load, keeps AI tools on track, and yields modular, maintainable software.

For canonical phase rules and PR expectations, see [DTI Methodology](standards/methodology.md).

### Phase 1 – Define ✅

*Write down exactly what your code needs to do.*

- Write a simple description of what the code should do, a checklist of what success looks like, or rough examples of how the code might be used.
- List what goes into the code (inputs), what should come out (outputs), what must always be true (rules), and any unusual situations to handle (edge-cases).
- Provide this definition to your AI assistant so it has a clear blueprint.

### Phase 2 – Test 🧪

*Write automated tests from the checklist you defined.*

- Turn each item from your checklist into a specific test
- Write tests that check both normal usage and edge cases
- Use these tests to keep AI-generated code on track and catch mistakes early

### Phase 3 – Implement ⚙️

*Write code until your tests pass, then refactor.*

- **Pass the Test:** Make the smallest change needed to pass the next failing test. Ask your AI assistant to help write basic code structure.
- **Verify:** Check that AI suggestions pass your tests before moving on.
- **Refactor:** Clean up the code only after all tests pass. Review the implementation for clarity, performance, and best practices *without changing its behavior*, relying on your tests to catch any regressions.

## Directory Structure

```
.
├── prompt.md                # AI assistant prompts for each DTI phase
├── TODO.md                  # Project backlog and next steps
│
├── standards/               # Documentation standards and guidelines
│   ├── methodology.md       # Canonical DTI workflow rules
│   ├── documentation.md     # Documentation requirements
│   ├── contributing.md      # Contribution guidelines
│   └── conventional_commits.md  # Commit message standards
│
└── templates/               # Documentation templates
    ├── define.md            # Requirements definition template
    ├── test-plan.md         # Test plan template
    ├── architecture.md      # System architecture template
    ├── overview.md          # Project overview template
    ├── setup.md             # Setup instructions template
    └── todo.md              # TODO list template
```

## Start Here (15 Minutes)

If you are new to DTI, follow this path first:

1. Read the worked example in order:
   - [Define artifact](examples/blobui-layer-transform/01_define.md)
   - [Test Plan artifact](examples/blobui-layer-transform/02_test-plan.md)
   - [Implementation artifact](examples/blobui-layer-transform/03_implementation.rs)
2. Copy the templates and start your own feature artifacts:

   ```bash
   cp templates/define.md define.md
   cp templates/test-plan.md test-plan.md
   ```

3. Use the phase prompts in [prompt.md](prompt.md) to drive your AI assistant through Define,
   then Test, then Implement.
4. Run the DTI structure check:

   ```bash
   ./scripts/check-dti.sh
   ```

## Getting Started

1. Clone this repository
2. Review the DTI Workflow in this README
3. Formulate your work using `templates/define.md` and `templates/test-plan.md`
4. Use the appropriate templates from the `templates/` directory for your documentation needs
5. Follow the contribution guidelines when submitting changes

## Standards

Our documentation follows several key standards:

- **Code Documentation**: Clear, concise comments with comprehensive API documentation
- **Markdown Standards**: Consistent heading hierarchy, proper formatting, and table of contents for longer documents
- **Best Practices**: Focus on clarity, include examples, and explain the reasoning behind decisions
- **Review Process**: All documentation changes undergo peer review for technical accuracy and style consistency

For detailed standards, see:
- [DTI Methodology](standards/methodology.md)
- [Documentation Standards](standards/documentation.md)
- [Conventional Commits](standards/conventional_commits.md)

## Templates

We provide several templates to ensure consistency:

- [Definition](templates/define.md) - For requirements gathering and documentation
- [Test Plan](templates/test-plan.md) - For mapping definitions to automated tests
- [Architecture Documentation](templates/architecture.md) - For system architecture documentation
- [Project Overview](templates/overview.md) - For high-level project descriptions
- [Setup Instructions](templates/setup.md) - For installation and configuration guides
- [TODO Lists](templates/todo.md) - For tracking documentation tasks

## Contributing

We welcome contributions! Please follow these steps:

1. Review our [Contributing Guidelines](standards/contributing.md) to understand the required DTI flow for PRs
2. Follow our [Documentation Standards](standards/documentation.md)
3. Use [Conventional Commits](standards/conventional_commits.md) for commit messages
4. Check our [Changelog](CHANGELOG.md) for recent updates
5. Submit a pull request for review

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of notable changes.

## License

This project is currently unlicensed. Add a `LICENSE` file to define reuse terms.
