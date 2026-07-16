use futures_util::StreamExt;

use crate::editor::event::UiEvent;

pub(crate) struct EventSource {
    stream: crossterm::event::EventStream,
}

impl EventSource {
    pub(crate) fn new() -> Self {
        Self {
            stream: crossterm::event::EventStream::new(),
        }
    }

    pub(crate) async fn next(&mut self) -> anyhow::Result<UiEvent> {
        loop {
            let Some(event) = self.stream.next().await else {
                anyhow::bail!("terminal event stream ended unexpectedly");
            };

            let event = event?;
            if let Some(ui_event) = UiEvent::from_crossterm(event) {
                return Ok(ui_event);
            }
        }
    }
}
