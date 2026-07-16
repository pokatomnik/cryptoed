# Cryptoed UI Architecture Guide

> Status: target architecture.
>
> This document describes the architecture that new UI code MUST follow and that
> existing UI code will be migrated to. The current implementation may not yet
> comply with every rule below.

## Purpose

This guide is written for both humans and coding agents. Read it before adding or
changing UI code under `src/editor/ui/`.

Normative words have the following meaning:

- **MUST**: required for architectural consistency.
- **MUST NOT**: prohibited.
- **SHOULD**: preferred unless there is a documented reason not to follow it.
- **MAY**: optional.

The main goals are:

- predictable event and data flow;
- isolated and reusable UI components;
- explicit component composition and focus handling;
- separation of UI, application logic, domain logic, and external effects;
- testable state transitions and rendering;
- files that remain easy to read and normally stay below 300 lines.

## Architectural Layers

```text
Runtime
  Terminal, EventStream, Tokio, drawing schedule, external effects
        |
        v
Application
  State, actions, update logic, effect requests, application outcome
        |
        v
UI component tree
  Event routing, focus, layout, component rendering
        |
        v
Ratatui widgets
  Block, Paragraph, TextArea, Scrollbar, List, Table, etc.
```

Dependency rules:

1. Domain code MUST NOT depend on Ratatui or Crossterm.
2. Runtime MUST NOT know about individual components or key bindings.
3. Components MUST NOT access the terminal directly.
4. Components MUST NOT perform file I/O, encryption, network access, or spawn
   uncontrolled background tasks.
5. A parent component owns its children and decides their layout and event-routing
   order.
6. A child component MUST NOT know about its siblings.

## What Is a UI Component?

A UI component is an application-level visual or interactive unit with a defined
lifecycle and an assigned rectangular area.

Examples:

- `EditorScreen`;
- `TextEditor`;
- `Header`;
- `Footer`;
- a save dialog;
- a file list or sidebar.

A component may:

- render application or component state;
- translate relevant `UiEvent` values into semantic actions;
- update the state that belongs to it;
- own children and compose them into a larger component.

A component is not the same as every Ratatui widget. `Block`, `Paragraph`,
`Scrollbar`, and `TextArea` are low-level widgets that components use internally.
A decorative widget does not need its own lifecycle component unless it gains
independent state, event handling, or reuse requirements.

Example:

```text
TextEditor component
├── Block widget
├── TextArea widget
└── Scrollbar widget
```

## Target Component Contract

The exact signatures may evolve during implementation, but all components SHOULD
follow this lifecycle:

```rust
pub(crate) trait Component<S, A> {
    fn init(&mut self, _state: &mut S) {}

    fn handle_event(
        &mut self,
        _event: &UiEvent,
        _state: &S,
    ) -> EventResult<A> {
        EventResult::Ignored
    }

    fn update(
        &mut self,
        _action: &A,
        _state: &mut S,
    ) -> UpdateResult<A> {
        UpdateResult::unchanged()
    }

    fn render(
        &mut self,
        frame: &mut ratatui::Frame<'_>,
        area: ratatui::layout::Rect,
        state: &S,
    );
}
```

Lifecycle methods:

- `init`: one-time initialization that does not perform external I/O.
- `handle_event`: translates an external event into a semantic action.
- `update`: mutates state in response to an action.
- `render`: draws the current state inside the assigned `Rect`.

Purely visual components normally implement only `render`; the other methods use
the default no-op implementations.

## Event Handling Result

Returning only `Option<Action>` is insufficient because a component may consume an
event without producing an action. Event propagation MUST therefore be explicit.

Target result type:

```rust
pub(crate) enum EventResult<A> {
    Ignored,
    Consumed,
    Action(A),
}
```

Meaning:

- `Ignored`: the parent MAY route the event elsewhere.
- `Consumed`: processing stops, but no action is emitted.
- `Action(action)`: processing stops and the action is queued.

This distinction is especially important for dialogs. For example, `Esc` may close
a dialog and MUST NOT also exit the application.

## Update Result

Updates SHOULD report whether rendering is required and whether another semantic
action should be processed.

Conceptual API:

```rust
pub(crate) struct UpdateResult<A> {
    pub redraw: bool,
    pub follow_up: Option<A>,
}
```

Do not add an action bus, dynamic registry, or multi-action allocation until the
application actually requires it. The application MAY later use a
`VecDeque<Action>` for action chaining.

## Event, Action, Effect, and Outcome

These concepts MUST remain separate.

### `UiEvent`

Describes something that happened outside the application:

```rust
pub(crate) enum UiEvent {
    Key(crossterm::event::KeyEvent),
    Paste(String),
    Resize { width: u16, height: u16 },
    FocusGained,
    FocusLost,
    Tick,
}
```

Crossterm types MAY exist in the UI/application boundary, but MUST NOT leak into
domain objects such as `Document`.

### `Action`

Describes semantic intent:

```rust
pub(crate) enum Action {
    Editor(EditorAction),
    Save,
    Exit,
    FocusNext,
}
```

Feature-specific actions SHOULD be grouped:

```rust
pub(crate) enum EditorAction {
    Input(crossterm::event::KeyEvent),
    Paste(String),
    ScrollUp,
    ScrollDown,
}
```

### `Effect`

Describes work outside pure state updates:

```rust
pub(crate) enum Effect {
    Finish(EditorOutcome),
    SaveFile { path: PathBuf, text: String },
}
```

The runtime or a dedicated effect handler executes effects. Components MUST NOT
execute them directly.

### `EditorOutcome`

Describes how the application completed and what data it produced. Save, exit, and
cancel SHOULD remain distinct intents even if they currently return similar data.

## Lifecycle for One Event

```text
EventStream receives terminal event
    -> runtime converts it to UiEvent
    -> root component routes UiEvent
    -> component returns EventResult<Action>
    -> application queues Action
    -> update mutates state
    -> update may request Effect or follow-up Action
    -> runtime executes Effect
    -> UI renders if redraw was requested
```

Rendering MUST NOT decide application behavior. Application behavior MUST be
represented by actions and state transitions.

## How to Write a New UI Component

1. Define the component's single responsibility.
2. Decide whether it is a lifecycle component or only an internal Ratatui widget.
3. Identify the smallest state type it needs.
4. Define a local action enum if the component is interactive.
5. Add a file under `ui/components/`.
6. Implement the common component contract.
7. Keep layout calculations local to the parent that owns the component.
8. Add update tests for behavior and `TestBackend` tests for important rendering.
9. Integrate the component into exactly one parent.

Suggested skeleton:

```rust
pub(crate) struct Sidebar;

pub(crate) enum SidebarAction {
    SelectNext,
    SelectPrevious,
    OpenSelected,
}

impl Component<SidebarState, SidebarAction> for Sidebar {
    fn handle_event(
        &mut self,
        event: &UiEvent,
        state: &SidebarState,
    ) -> EventResult<SidebarAction> {
        // Translate only sidebar-relevant events.
        EventResult::Ignored
    }

    fn update(
        &mut self,
        action: &SidebarAction,
        state: &mut SidebarState,
    ) -> UpdateResult<SidebarAction> {
        // Mutate only sidebar state.
        UpdateResult::unchanged()
    }

    fn render(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        state: &SidebarState,
    ) {
        // Render only inside area.
    }
}
```

## How to Add a Component to the Existing Tree

Components SHOULD be composed statically with concrete fields:

```rust
pub(crate) struct EditorScreen {
    header: Header,
    editor: TextEditor,
    footer: Footer,
    sidebar: Sidebar,
}
```

Do not introduce `Vec<Box<dyn Component>>` unless components are selected at
runtime, for example by a plugin system.

The parent component MUST:

1. own the child;
2. allocate the child's `Rect`;
3. route relevant events to the child;
4. map child actions into parent/application actions;
5. pass only the state required by the child;
6. render the child in the allocated area.

The child MUST NOT:

- calculate the full-screen layout;
- render outside its `Rect`;
- call sibling methods;
- change focus globally;
- exit the application directly.

## Passing Data to Components

Data SHOULD flow from parent/application state to children through explicit state
references or small view-state structures.

Preferred:

```rust
editor.render(frame, area, &state.editor);
header.render(frame, area, &state.document);
footer.render(frame, area, &state.status);
```

Avoid passing the entire `EditorApplication` to every component. Broad access makes
component dependencies implicit and encourages unrelated state mutations.

A component SHOULD receive:

- immutable state during event translation and rendering;
- mutable state only during `update`;
- constructor arguments for stable configuration;
- theme or keymap references through explicit configuration objects.

Data needed only for display MAY be represented by a dedicated view state.

## Triggering Events and Actions

Components do not trigger terminal events. They translate incoming events into
semantic actions.

Preferred:

```text
KeyEvent Ctrl+S
    -> Action::Save
    -> application update
    -> Effect::SaveFile or Effect::Finish
```

Components MUST NOT:

- call `std::process::exit`;
- stop the main loop directly;
- write files;
- send arbitrary messages through hidden global channels;
- mutate unrelated application state.

A child returns a local action. Its parent maps it to the parent action type:

```text
SidebarAction::OpenSelected
    -> Action::OpenDocument(selected_path)
```

## Reacting to Data Changes

Ratatui uses immediate-mode rendering. Components do not subscribe to individual
fields. When state changes, the application requests a redraw and every visible
component renders the latest state.

Rules:

1. `update` changes state and sets `redraw = true` when visible output may change.
2. `render` reads the current state.
3. Components MUST NOT cache derived display data unless computation is expensive
   and invalidation is explicit.
4. A component MUST NOT poll application data during rendering.
5. Background task results MUST arrive as events/actions and update state before
   rendering.

## Component State vs Application Data State

Different state categories have different owners.

### Domain state

Examples:

- document path;
- encryption metadata;
- saved revision.

Domain state MUST NOT contain Ratatui or Crossterm types.

### Application state

Examples:

- current screen;
- save status;
- active dialog;
- focus;
- pending effects.

Application state controls behavior shared across components.

### Component or widget state

Examples:

- text cursor and selection;
- textarea viewport;
- list selection;
- scrollbar position;
- temporary dialog input.

Component state belongs either to the component or to a dedicated state object
owned by the application. It MUST have exactly one owner.

For text editing, introduce an `EditorBuffer` wrapper around `TextArea`. It SHOULD
be the single source of truth for editable content. Do not keep a synchronized copy
of the same text in `Document`.

Use these questions to choose the owner:

- Is the value meaningful without the UI? Put it in domain/application state.
- Is it only needed to render or navigate a widget? Put it in component state.
- Do multiple components need it? Lift it to their nearest common parent.
- Does saving or encryption need it? Expose it through an application-level state
  abstraction such as `EditorBuffer`, not through ad-hoc component access.

## Focus Management

Focus MUST be explicit once more than one interactive component exists.

Start with an enum, not a generic focus framework:

```rust
pub(crate) enum Focus {
    Editor,
    Sidebar,
    Dialog,
}
```

The root screen owns focus and routes events in this order:

1. active modal or overlay;
2. focused component;
3. screen-level bindings;
4. application-level bindings.

Emergency bindings such as forced termination MAY have a documented higher
priority.

Focus rules:

- children MAY return an action requesting focus;
- children MUST NOT mutate global focus directly;
- a hidden or disabled component MUST NOT retain focus;
- opening a modal stores previous focus and moves focus to the modal;
- closing a modal restores previous focus when possible;
- `Esc` is handled by the active modal before application exit;
- focus changes SHOULD request redraw.

## Nested Components and Overlays

A composite component follows the same contract as a leaf component.

`EditorScreen` may own:

```rust
pub(crate) enum Overlay {
    SaveDialog(SaveDialog),
    HelpDialog(HelpDialog),
}
```

When an overlay is active, the parent:

1. renders the base screen;
2. optionally clears or dims the overlay area;
3. renders the overlay last;
4. routes events to the overlay first;
5. stops propagation when the overlay consumes the event.

A dialog may itself own input fields and buttons. It becomes the parent responsible
for their layout, focus, and local event routing.

## Rendering Rules

A component's `render` method MUST:

- render only inside its assigned `Rect`;
- handle very small areas without panicking;
- avoid file, network, and encryption operations;
- avoid changing semantic application state;
- avoid deciding whether the application should exit or save;
- use Ratatui `Widget` or `StatefulWidget` internally where appropriate.

Render-time changes to strictly visual widget state may be acceptable when required
by Ratatui, but they MUST NOT change business behavior.

Shared colors, borders, and styles SHOULD move to a theme module once repetition
appears. Do not create a theme abstraction before there is real duplication.

## Key Bindings

Key matching and displayed help MUST come from the same keymap definition. Do not
hard-code `Ctrl+S` behavior in one file and footer text in another.

Bindings SHOULD be scoped:

- application-global bindings;
- screen bindings;
- focused-component bindings;
- modal bindings.

A binding may contain:

```rust
pub(crate) struct KeyBinding<A> {
    pub chord: KeyChord,
    pub action: A,
    pub label: &'static str,
    pub description: &'static str,
}
```

The event router uses `chord` and `action`; the footer uses `label` and
`description`.

## Async Work and External Effects

Components MUST NOT spawn Tokio tasks directly unless the component owns an
isolated resource with an explicit cancellation lifecycle. The normal flow is:

```text
Component action
    -> application update
    -> Effect request
    -> runtime starts async work
    -> completion UiEvent
    -> application update
    -> redraw
```

This keeps component behavior deterministic and prevents tasks from outliving the
UI that created them.

## Testing Requirements

### State/update tests

Test without a terminal:

- an event maps to the expected action;
- an action changes only the expected state;
- ignored events do not mutate state;
- save, exit, and cancel stay distinct;
- focus changes follow routing rules.

### Component rendering tests

Use Ratatui `TestBackend` for important visual behavior:

- correct rendering for normal and small areas;
- header and footer data;
- focused and unfocused styles;
- scrollbar visibility;
- modal layering.

Avoid broad snapshots for every detail. Prefer focused assertions for behavior that
is part of the component contract.

### Event propagation tests

Test that:

- focused components receive events;
- unfocused components do not consume typing events;
- modal events do not reach the background screen;
- global shortcuts do not get inserted into `TextArea`;
- `Esc` closes a modal before exiting the application.

## Target UI Directory

```text
ui/
├── UI-GUIDE.md
├── mod.rs
├── component.rs
├── screen.rs
├── keymap.rs
└── components/
    ├── mod.rs
    ├── header.rs
    ├── footer.rs
    └── text_editor.rs
```

Create additional modules such as `theme.rs`, `overlay.rs`, or
`components/text_editor/scrollbar.rs` only when the corresponding file has more
than one clear responsibility.

`mod.rs` files SHOULD contain declarations and narrow re-exports, not substantial
implementations.

## Anti-Patterns

Do not introduce:

- a single UI file containing all layouts and widgets;
- domain objects that match `KeyEvent`;
- duplicate copies of editable text;
- unconditional broadcasting of every event to every component;
- components that directly call siblings;
- components that own `Terminal`;
- business mutations during rendering;
- hidden global channels for component communication;
- `Arc<Mutex<_>>` as the default state-management mechanism;
- `Vec<Box<dyn Component>>` for a statically known component tree;
- abstractions created only to reduce line count without improving cohesion.

## Checklist for a New Component

- [ ] The responsibility is narrow and clearly named.
- [ ] The component is necessary; an internal Ratatui widget is not enough.
- [ ] State ownership is explicit and has one source of truth.
- [ ] Local actions are defined for interactive behavior.
- [ ] External effects are emitted, not executed.
- [ ] Event consumption is explicit.
- [ ] The component renders only inside its assigned `Rect`.
- [ ] The parent owns layout and focus routing.
- [ ] The component does not know its siblings.
- [ ] Small terminal areas do not panic.
- [ ] Relevant update and render tests exist.
- [ ] The production file remains below approximately 300 lines.

## Checklist for Integrating a Child

- [ ] Add the child as a concrete parent field.
- [ ] Add a child state field or a narrow state view.
- [ ] Allocate the child area in the parent layout.
- [ ] Route events according to focus and overlay priority.
- [ ] Map child actions to parent/application actions.
- [ ] Render the child after its background and before higher overlays.
- [ ] Update focus rules if the child is interactive.
- [ ] Update the shared keymap if the child adds bindings.
- [ ] Add propagation and integration tests.

## Instructions for Coding Agents

Before modifying UI code:

1. Read this document completely.
2. Inspect the root screen, the target component, its state, actions, and parent.
3. Preserve the event -> action -> update -> render direction.
4. Prefer static component composition.
5. Do not move business logic into `ui/`.
6. Do not add dependencies between sibling components.
7. Keep changes focused and update tests with behavior changes.
8. If a requested change conflicts with this guide, explain the conflict before
   implementing it.
9. Update this guide when an accepted architectural decision changes the component
   contract or data flow.
