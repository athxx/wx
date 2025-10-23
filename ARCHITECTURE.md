# Weixin Architecture & Refactor Plan

## Goals
- Preserve current behavior and UI.
- Introduce clearer layering and boundaries for future growth.
- Reduce duplication and centralize cross-cutting concerns.

## Target Layout
- src/core/
  - models.rs (re-export of existing models; future: split by domain)
  - events.rs (app-wide event types; re-export existing component events for now)
  - repos.rs (traits for data access: ContactsRepo, SessionsRepo)
- src/infra/
  - memory_repos.rs (in-memory implementations using current sample_data)
- src/ui/
  - app/ (existing: state.rs, view_builder.rs, event_handlers.rs)
  - components/ (existing components)
  - theme.rs (existing theme helper)
  - prelude.rs (future: common use imports)
  - constants.rs (future: UI constants)

Note: In this phase we avoid moving existing files. New modules re-export existing ones, so imports can migrate gradually without breaking functionality.

## Data Boundary
- Define repo traits in core::repos:
  - ContactsRepo: get_all() -> Vec<Contact>
  - SessionsRepo: get_messages(&Contact) -> Vec<Message>
- Provide infra::memory_repos implementations delegating to data::sample_data to keep identical data.
- WeixinApp depends on traits (Box<dyn ...>), enabling future persistence without app changes.

## Events Boundary
- core::events re-exports current event types from components, providing a unified import path now.
- Future: consolidate to app-level enums if needed (no behavior change planned here yet).

## Refactor Steps (Phase 1 — No behavior change)
1) Add new core/infra modules and traits with memory implementations.
2) Inject repos into WeixinApp and replace direct calls to sample_data with repo calls.
3) Keep all UI rendering and event logic unchanged.

## Future (Phase 2 — Optional, still behavior-preserving)
- Introduce newtypes ContactId/MessageId to avoid magic strings.
- Extract UI prelude and constants; deduplicate more UI builders.
- Gradually migrate imports to core::events and core::models.

## Coding Guidelines
- UI modules must not import infra; only core + gpui/gpui-component.
- App state composes repos via traits.
- Prefer small, pure helper functions for repeated UI fragments.

## Rollback Strategy
- Repos keep delegating to data::sample_data; switching back is trivial.
- No file moves in Phase 1; import paths remain valid.
