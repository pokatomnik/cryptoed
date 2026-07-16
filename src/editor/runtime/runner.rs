use std::collections::VecDeque;

use crate::editor::{
    application::EditorApplication,
    effect::Effect,
    outcome::EditorOutcome,
    runtime::{event_source::EventSource, terminal::TerminalSession},
    ui::component::EventResult,
};

pub(crate) async fn run(application: EditorApplication) -> anyhow::Result<EditorOutcome> {
    let mut runtime = Runtime::new(application)?;
    runtime.run().await
}

struct Runtime {
    application: EditorApplication,
    terminal: TerminalSession,
    events: EventSource,
}

impl Runtime {
    fn new(application: EditorApplication) -> anyhow::Result<Self> {
        Ok(Self {
            application,
            terminal: TerminalSession::enter()?,
            events: EventSource::new(),
        })
    }

    async fn run(&mut self) -> anyhow::Result<EditorOutcome> {
        self.redraw()?;

        loop {
            let event = self.events.next().await?;
            let mut actions = VecDeque::new();
            let mut redraw = false;

            if let EventResult::Action(action) = self.application.handle_event(event) {
                actions.push_back(action);
            }

            while let Some(action) = actions.pop_front() {
                let update = self.application.update(action);
                redraw |= update.redraw;
                actions.extend(update.follow_up);

                for effect in update.effects {
                    if let Some(outcome) = self.handle_effect(effect).await? {
                        if redraw {
                            self.redraw()?;
                        }
                        return Ok(outcome);
                    }
                }
            }

            if redraw {
                self.redraw()?;
            }
        }
    }

    async fn handle_effect(&mut self, effect: Effect) -> anyhow::Result<Option<EditorOutcome>> {
        match effect {
            Effect::Finish(outcome) => Ok(Some(outcome)),
        }
    }

    fn redraw(&mut self) -> anyhow::Result<()> {
        self.terminal.draw(|frame| self.application.render(frame))
    }
}
