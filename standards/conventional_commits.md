# Conventional Commits (Project Guidance)

This document defines how this repository applies Conventional Commits to DTI work.

For the full specification, see:

- https://www.conventionalcommits.org/en/v1.0.0/

## Format

Use:

`<type>[optional scope]: <description>`

Examples:

- `docs(define): add edge cases for token refresh flow`
- `test(auth): map all login criteria to unit tests`
- `feat(api): implement refresh token rotation`
- `fix(parser): handle empty CSV rows`

## DTI Mapping

Use commit types to reflect the DTI phase of the change:

- `docs`: Define artifacts and documentation-only changes
- `test`: Test plan changes and test code changes
- `feat`: new behavior added during Implement
- `fix`: bug fixes during Implement
- `refactor`: behavior-preserving cleanup after tests pass
- `chore`, `ci`, `build`, `perf`: infrastructure/tooling updates when applicable

## Scope Guidance

Use a short scope that identifies the work area:

- `define`, `test-plan`, `docs`, `templates`, `scripts`, `ci`, or a component name

Examples:

- `docs(test-plan): clarify coverage map instructions`
- `test(transform): add zero-aspect-ratio case`
- `refactor(renderer): simplify matrix composition`

## Breaking Changes

Mark breaking changes with either:

- `!` after type/scope, for example `feat(api)!: change auth response shape`
- `BREAKING CHANGE:` footer with migration notes

## Commit Quality Rules

- Keep commits focused: one logical change per commit.
- Write descriptions as behavior, not implementation details.
- Prefer multiple small commits over one mixed commit.
