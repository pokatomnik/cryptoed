use crate::editor::{
    action::{Action, EditorAction},
    effect::Effect,
    event::UiEvent,
    outcome::{EditorOutcome, FinishReason},
    state::EditorState,
    ui::{component::EventResult, screen::EditorScreen},
};

#[derive(Debug)]
pub(crate) struct ApplicationUpdate {
    pub redraw: bool,
    pub effects: Vec<Effect>,
    pub follow_up: Vec<Action>,
}

pub(crate) struct EditorApplication {
    state: EditorState,
    screen: EditorScreen,
}

impl EditorApplication {
    pub(crate) fn new(state: EditorState) -> Self {
        Self {
            state,
            screen: EditorScreen::new(),
        }
    }

    pub(crate) fn handle_event(&mut self, event: UiEvent) -> EventResult<Action> {
        self.screen.handle_event(&event, &self.state)
    }

    pub(crate) fn update(&mut self, action: Action) -> ApplicationUpdate {
        match action {
            Action::Editor(EditorAction::Input(_)) => {
                let result = self.screen.update(&action, &mut self.state);
                ApplicationUpdate {
                    redraw: result.redraw,
                    effects: Vec::new(),
                    follow_up: result.follow_up,
                }
            }
            Action::Save => ApplicationUpdate {
                redraw: false,
                effects: vec![Effect::Finish(EditorOutcome::new(
                    FinishReason::SaveAndExit,
                    self.state.buffer().text(),
                ))],
                follow_up: Vec::new(),
            },
            Action::Exit => ApplicationUpdate {
                redraw: false,
                effects: vec![Effect::Finish(EditorOutcome::new(
                    FinishReason::Exit,
                    self.state.buffer().text(),
                ))],
                follow_up: Vec::new(),
            },
        }
    }

    pub(crate) fn render(&self, frame: &mut ratatui::Frame<'_>) {
        self.screen.render(frame, &self.state);
    }
}
