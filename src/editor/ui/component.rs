use ratatui::{Frame, layout::Rect};

use crate::editor::event::UiEvent;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EventResult<A> {
    Ignored,
    Consumed,
    Action(A),
}

#[derive(Debug, Clone)]
pub(crate) struct UpdateResult<A> {
    pub redraw: bool,
    pub follow_up: Vec<A>,
}

impl<A> UpdateResult<A> {
    pub(crate) fn none() -> Self {
        Self {
            redraw: false,
            follow_up: Vec::new(),
        }
    }

    pub(crate) fn redraw() -> Self {
        Self {
            redraw: true,
            follow_up: Vec::new(),
        }
    }
}

pub(crate) trait Component {
    type State;
    type Action;

    #[allow(dead_code)]
    fn init(&mut self, _state: &mut Self::State) {}

    fn handle_event(
        &mut self,
        _event: &UiEvent,
        _state: &Self::State,
    ) -> EventResult<Self::Action> {
        EventResult::Ignored
    }

    fn update(
        &mut self,
        _action: &Self::Action,
        _state: &mut Self::State,
    ) -> UpdateResult<Self::Action> {
        UpdateResult::none()
    }

    fn render(&self, frame: &mut Frame<'_>, area: Rect, state: &Self::State);
}
