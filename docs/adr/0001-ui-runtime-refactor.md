# ADR 0001: UI/Runtime Refactor Boundaries (App + Widgets)

## Status
Accepted

## Date
2026-02-17

## Context
`openpad-app/src/app.rs` and `openpad-widgets/src/message_list.rs` had become high-churn integration points mixing orchestration, UI sync, domain decisions, and render/event internals. This made bug fixes risky (scroll behavior, tab orchestration, terminal layering) and slowed feature work.

## Baseline snapshot
- `openpad-app/src/app.rs`: 2291 lines (pre-split)
- `openpad-app/src/state/handlers.rs`: 947 lines
- `openpad-widgets/src/message_list.rs`: 1255 lines (pre-split)
- Validation commands:
- `cargo check -p openpad-app`
- `cargo test -p openpad-app`
- `cargo test -p openpad-widgets`

## Decision
Adopt incremental, compatibility-first modularization with explicit seams:

### App module boundaries
- `openpad-app/src/app/lifecycle.rs`
- startup/connect/load operations and project/session loaders
- `openpad-app/src/app/action_dispatch.rs`
- `AppAction` handling and dialog result routing
- `openpad-app/src/app/dock_controller.rs`
- center dock/tab lifecycle, dedupe/select/close, unsaved intent handling
- `openpad-app/src/app/ui_sync.rs`
- top strip/composer/editor strip synchronization and tab render refresh

### State boundaries
- `openpad-app/src/state/reducer.rs`
- pure state transitions + unit-testable helpers
- `openpad-app/src/state/effects.rs`
- explicit side-effect intents (`RequestSessionDiff`)
- `openpad-app/src/state/handlers.rs`
- compatibility wrapper for UI binding and effect execution

### Message list boundaries
- `openpad-widgets/src/message_list/mod.rs`
- script/template + public widget type + trait forwarding
- `openpad-widgets/src/message_list/model.rs`
- widget model helpers + explicit tail policy (`TailMode`)
- `openpad-widgets/src/message_list/events.rs`
- event/interaction handling
- `openpad-widgets/src/message_list/render.rs`
- draw pipeline and template binding
- `openpad-widgets/src/message_list/api.rs`
- `MessageListRef` public APIs

## Non-goals
- No `openpad-protocol` refactor in this phase.
- No UX redesign or feature expansion.
- No persistence changes for dock tabs/layout.

## Consequences
### Positive
- Smaller, focused files with clearer ownership.
- Easier unit testing for reducer and tail behavior.
- Reduced risk of accidental scroll/tail regressions from unrelated redraw paths.

### Negative
- More files/modules to navigate.
- Temporary duplication while compatibility wrappers remain.

## Migration order
1. Extract app modules (lifecycle/action/dock/ui sync).
2. Introduce reducer/effects seam and route selected action paths through reducer.
3. Split message list into model/events/render/api modules.
4. Add and expand regression tests around state transitions and tail policy.
5. Remove compatibility wrappers when coverage is sufficient.
