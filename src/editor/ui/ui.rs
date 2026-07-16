use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::editor::{document::EditorDocument, ui::text_editor::TextEditor};

pub(crate) fn draw(frame: &mut Frame<'_>, document: &EditorDocument, editor: &TextEditor) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .split(frame.area());

    render_header(frame, chunks[0], document);
    editor.render(frame, chunks[1], "Editor");
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, document: &EditorDocument) {
    let title = format!("Editing: {}", document.path_label());

    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(
            "cryptoed",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" — "),
        Span::raw(title),
    ]))
    .block(Block::bordered().borders(Borders::ALL))
    .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame<'_>, area: Rect) {
    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(
            "Ctrl+S",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" save/finish  "),
        Span::styled(
            "Ctrl+X",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" quit  "),
        Span::styled(
            "Esc",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" quit"),
    ]))
    .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
