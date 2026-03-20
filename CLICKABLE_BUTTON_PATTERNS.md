# Clickable button patterns (brief)

## Goal
Keep button rendering separate from click behavior so teams can add features without rewriting input logic.

## 1) `ButtonId` + centralized action dispatch (recommended first step)
- Each button has stable metadata: `id`, `label`, `rect`, `enabled`.
- Click/focus only resolves to `ButtonId`.
- A dispatcher maps `ButtonId -> AppAction`.
- Main update loop handles `AppAction`.

Why: simple, testable, and avoids storing arbitrary closures in UI widgets.

## 2) Event bus for decoupling modules
- UI emits events like `UiEvent::ButtonClicked(ButtonId)`.
- Feature modules subscribe and convert events into domain actions.
- Keep bus synchronous at first; add async later only if needed.

Why: scales when features live in separate files/crates.

## 3) Command pattern for unknown future behavior
- Map `ButtonId -> Command` where `Command` has `execute(&mut AppContext)`.
- Commands can be swapped at runtime (useful for plugins/config-driven UIs).

Why: behavior is open-ended without changing core button code.

## 4) Recommended boundaries
- `ui/widgets/button.rs`: draw + hit-test only.
- `ui/input.rs`: mouse/keyboard -> `UiEvent`.
- `app/actions.rs`: `AppAction` definitions.
- `app/dispatch.rs`: event/action routing.
- `features/*`: business logic handlers.

## 5) Practical conventions
- Use typed IDs (`ButtonId`) instead of raw indexes.
- Keep `Rect` state owned by view layer; do not leak into business logic.
- Log all `UiEvent`/`AppAction` transitions for debugging.
- Unit test: hit-test, focus navigation, and dispatch mapping.

## Minimal evolution path
1. Replace index-based click handling with `ButtonId`.
2. Add `UiEvent::ButtonClicked(ButtonId)`.
3. Add dispatcher returning `AppAction`.
4. Let feature modules handle `AppAction`.
