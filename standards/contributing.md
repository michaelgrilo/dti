# Contributing Guidelines

## DTI Workflow Requirement

All contributions must follow the [Define-Test-Implement methodology](./methodology.md). Pull requests that skip phases will be asked to add the missing artifacts before review proceeds.

### 1. Define

- [ ] Fill out a [Definition](../templates/define.md) for the feature or fix
- [ ] List inputs, outputs, rules, and edge cases
- [ ] Resolve all open questions before moving on
- [ ] Include the completed definition in your PR (or link to it)

### 2. Test

- [ ] Derive a [Test Plan](../templates/test-plan.md) from your definition
- [ ] Map every success criterion to at least one test
- [ ] Write all tests before writing implementation code
- [ ] Confirm tests fail initially (they have nothing to pass against yet)

### 3. Implement

- [ ] Write the minimum code needed to make your tests pass
- [ ] Refactor only after all tests are green
- [ ] Do not change behavior during refactoring — rely on tests to catch regressions

## Development Workflow

- [ ] Fork the repository
- [ ] Create a feature branch from `main`
- [ ] Follow the DTI phases above for each unit of work
- [ ] Write clear commit messages using [Conventional Commits](./conventional_commits.md)
- [ ] Keep changes focused and atomic — one feature or fix per PR
- [ ] Update documentation alongside code changes

## Code Review Process

- [ ] Submit pull requests early for feedback
- [ ] PR description must reference the Definition artifact and Test Plan
- [ ] Ensure CI checks pass (linting, link checks)
- [ ] Address review comments promptly
- [ ] Update the [Changelog](../CHANGELOG.md) for user-facing changes

## Version Control

- Follow [semantic versioning](https://semver.org/) for releases
- Maintain the changelog for all significant changes
- Tag releases with code versions
- Archive outdated documentation appropriately

---

*See [DTI Methodology](./methodology.md) for the full workflow. See [Documentation Standards](./documentation.md) for writing guidelines.*
