use cursive::{
    style::{Effect, Style},
    utils::markup::StyledString,
    view::{Resizable, Scrollable, ViewWrapper},
    views::{Panel, ResizedView, ScrollView, TextView},
    wrap_impl,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

pub(crate) struct PreviewView {
    panel: ResizedView<Panel<ScrollView<TextView>>>,
}

impl PreviewView {
    pub fn new(text: impl Into<String>) -> Self {
        let content = TextView::new(render_markdown(&text.into())).scrollable();
        let panel = Panel::new(content).title("Preview").full_screen();

        Self { panel }
    }
}

impl ViewWrapper for PreviewView {
    wrap_impl!(self.panel: ResizedView<Panel<ScrollView<TextView>>>);
}

#[derive(Default)]
struct MarkdownRenderer {
    output: StyledString,
    styles: Vec<Style>,
    lists: Vec<Option<u64>>,
    link_targets: Vec<String>,
    image_targets: Vec<String>,
    item_depth: usize,
    table_cell: Option<usize>,
}

impl MarkdownRenderer {
    fn render(mut self, text: &str) -> StyledString {
        let options =
            Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS;

        for event in Parser::new_ext(text, options) {
            self.handle_event(event);
        }

        self.output
    }

    fn handle_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.append(text.as_ref()),
            Event::Code(code) => {
                self.append_with_style(code.as_ref(), Style::from(Effect::Reverse))
            }
            Event::InlineMath(math) => self.append_with_style(math.as_ref(), Style::secondary()),
            Event::DisplayMath(math) => {
                self.ensure_newlines(2);
                self.append_with_style(math.as_ref(), Style::secondary());
                self.ensure_newlines(2);
            }
            Event::Html(html) | Event::InlineHtml(html) => self.append(html.as_ref()),
            Event::FootnoteReference(reference) => self.append(&format!("[{reference}]")),
            Event::SoftBreak | Event::HardBreak => self.ensure_newlines(1),
            Event::Rule => {
                self.ensure_newlines(2);
                self.append_with_style(
                    "----------------------------------------",
                    Style::tertiary(),
                );
                self.ensure_newlines(2);
            }
            Event::TaskListMarker(checked) => self.append(if checked { "[x] " } else { "[ ] " }),
        }
    }

    fn start_tag(&mut self, tag: Tag<'_>) {
        let style = match tag {
            Tag::Paragraph => Style::none(),
            Tag::Heading { level, .. } => {
                self.ensure_newlines(2);
                heading_style(level)
            }
            Tag::BlockQuote(_) => {
                self.ensure_newlines(2);
                self.append_with_style("> ", Style::tertiary());
                Style::tertiary().combine(Effect::Italic)
            }
            Tag::CodeBlock(_) => {
                self.ensure_newlines(2);
                Style::secondary()
            }
            Tag::List(first_item) => {
                self.ensure_newlines(if self.lists.is_empty() { 2 } else { 1 });
                self.lists.push(first_item);
                Style::none()
            }
            Tag::Item => {
                self.ensure_newlines(1);
                self.append(&"  ".repeat(self.lists.len().saturating_sub(1)));
                let marker = match self.lists.last_mut() {
                    Some(Some(number)) => {
                        let marker = format!("{number}. ");
                        *number += 1;
                        marker
                    }
                    _ => "- ".to_string(),
                };
                self.append_with_style(&marker, Style::title_secondary());
                self.item_depth += 1;
                Style::none()
            }
            Tag::Emphasis => Style::from(Effect::Italic),
            Tag::Strong => Style::from(Effect::Bold),
            Tag::Strikethrough => Style::from(Effect::Strikethrough),
            Tag::Link { dest_url, .. } => {
                self.link_targets.push(dest_url.into_string());
                Style::title_secondary().combine(Effect::Underline)
            }
            Tag::Image { dest_url, .. } => {
                self.image_targets.push(dest_url.into_string());
                self.append_with_style("Image: ", Style::tertiary());
                Style::from(Effect::Italic)
            }
            Tag::Table(_) => {
                self.ensure_newlines(2);
                Style::none()
            }
            Tag::TableHead => {
                self.ensure_newlines(1);
                self.table_cell = Some(0);
                Style::from(Effect::Bold)
            }
            Tag::TableRow => {
                self.ensure_newlines(1);
                self.table_cell = Some(0);
                Style::none()
            }
            Tag::TableCell => {
                if self.table_cell.is_some_and(|cell| cell > 0) {
                    self.append_with_style(" | ", Style::tertiary());
                }
                if let Some(cell) = &mut self.table_cell {
                    *cell += 1;
                }
                Style::none()
            }
            Tag::DefinitionListTitle => Style::from(Effect::Bold),
            Tag::DefinitionListDefinition => {
                self.append("  ");
                Style::none()
            }
            Tag::FootnoteDefinition(label) => {
                self.ensure_newlines(2);
                self.append_with_style(&format!("[{label}] "), Style::tertiary());
                Style::none()
            }
            Tag::Superscript
            | Tag::Subscript
            | Tag::HtmlBlock
            | Tag::MetadataBlock(_)
            | Tag::DefinitionList => Style::none(),
        };

        self.styles.push(style);
    }

    fn end_tag(&mut self, tag: TagEnd) {
        self.styles.pop();

        match tag {
            TagEnd::Paragraph => self.ensure_newlines(if self.item_depth > 0 { 1 } else { 2 }),
            TagEnd::Heading(_) | TagEnd::BlockQuote(_) | TagEnd::CodeBlock => {
                self.ensure_newlines(2);
            }
            TagEnd::List(_) => {
                self.lists.pop();
                self.ensure_newlines(if self.lists.is_empty() { 2 } else { 1 });
            }
            TagEnd::Item => {
                self.item_depth = self.item_depth.saturating_sub(1);
                self.ensure_newlines(1);
            }
            TagEnd::Link => {
                if let Some(target) = self.link_targets.pop() {
                    self.append_with_style(&format!(" <{target}>"), Style::tertiary());
                }
            }
            TagEnd::Image => {
                if let Some(target) = self.image_targets.pop() {
                    self.append_with_style(&format!(" <{target}>"), Style::tertiary());
                }
            }
            TagEnd::Table => self.ensure_newlines(2),
            TagEnd::TableHead | TagEnd::TableRow => {
                self.table_cell = None;
                self.ensure_newlines(1);
            }
            TagEnd::Emphasis
            | TagEnd::Strong
            | TagEnd::Strikethrough
            | TagEnd::TableCell
            | TagEnd::HtmlBlock
            | TagEnd::FootnoteDefinition
            | TagEnd::Superscript
            | TagEnd::Subscript
            | TagEnd::MetadataBlock(_)
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition => {}
        }
    }

    fn append(&mut self, text: &str) {
        self.output.append_styled(text, Style::merge(&self.styles));
    }

    fn append_with_style(&mut self, text: &str, style: Style) {
        self.output
            .append_styled(text, Style::merge(&self.styles).combine(style));
    }

    fn ensure_newlines(&mut self, count: usize) {
        if self.output.is_empty() {
            return;
        }

        let current = self
            .output
            .source()
            .chars()
            .rev()
            .take_while(|character| *character == '\n')
            .count();
        if current < count {
            self.output.append_plain("\n".repeat(count - current));
        }
    }
}

fn heading_style(level: HeadingLevel) -> Style {
    match level {
        HeadingLevel::H1 => Style::title_primary()
            .combine(Effect::Bold)
            .combine(Effect::Underline),
        HeadingLevel::H2 => Style::title_secondary().combine(Effect::Bold),
        HeadingLevel::H3 | HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => {
            Style::from(Effect::Bold)
        }
    }
}

fn render_markdown(text: &str) -> StyledString {
    MarkdownRenderer::default().render(text)
}

#[cfg(test)]
mod tests {
    use cursive::style::Style;

    use super::render_markdown;

    #[test]
    fn render_removes_markdown_syntax() {
        let rendered = render_markdown("# Heading\n\nText with **bold**, *italic* and `code`.");

        assert_eq!(
            rendered.source(),
            "Heading\n\nText with bold, italic and code.\n\n"
        );
        assert!(!rendered.source().contains("**"));
        assert!(!rendered.source().contains('`'));
    }

    #[test]
    fn render_applies_styles_to_markdown_spans() {
        let rendered = render_markdown("plain **bold** *italic*");
        let styled_spans = rendered
            .spans()
            .filter(|span| span.attr != &Style::none())
            .count();

        assert_eq!(styled_spans, 2);
    }
}
