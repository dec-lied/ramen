use tui::layout::{Layout, Direction, Constraint, Rect, Alignment};
use tui::style::Color;
use tui::text::{Spans, Span, Text};
use tui::widgets::{Paragraph, BorderType, List, ListItem, Tabs};
use tui::
{
    style::Style,
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;
use std::fs;

#[derive(PartialEq)]
pub enum Mode
{
    NORMAL,
    INSERT
}

pub fn vertical_panels(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Vertical)
        .constraints
        ([
            Constraint::Percentage(10), // title
            Constraint::Percentage(10), // tabs
            Constraint::Percentage(80)  // content
        ])
        .split(area);
}

pub fn horizontal_panels(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Horizontal)
        .constraints
        ([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(area);
}

pub fn vertical_panels2(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Vertical)
        .constraints
        ([
            Constraint::Percentage(75), // controls
            Constraint::Percentage(15), // roms dir
            Constraint::Percentage(10)  // mode
        ])
        .split(area);
}

pub fn title<'a>() -> Paragraph<'a>
{
    return Paragraph::new("ramen".to_string()).block
    (
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style
            (
                Style::default()
                    .fg(Color::White)
            )
    )
    .alignment(Alignment::Center);
}

pub fn tabs(titles: Vec<String>) -> Tabs<'static>
{
    return Tabs::new(titles.iter().cloned().map(Spans::from).collect())
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Consoles")
                .title_alignment(Alignment::Center)
        )
        .style
        (
            Style::default()
                .fg(Color::White)
        )
        .highlight_style
        (
            Style::default()
                .fg(Color::Yellow)
        )
        .divider(tui::symbols::line::VERTICAL);
}

pub fn roms_list<'a>(items: Vec<ListItem<'a>>) -> List<'a>
{
    return List::new(items)
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("ROMS")
                .title_alignment(Alignment::Center)
        )
        .style
        (
            Style::default()
                .fg(Color::White)
        )
        .highlight_symbol("-> ")
        .highlight_style
        (
            Style::default()
                .fg(Color::Cyan)
        )
}

pub fn controls<'a>() -> Paragraph<'a>
{
    let paragraph_string: Text = vec!
    [
        Spans::from(Span::styled("NORMAL MODE:", Style::default().fg(Color::Magenta))),
        Spans::from(Span::raw("h: switches cwd to next tab's console")),
        Spans::from(Span::raw("l: switches cwd to previous tab's console")),
        Spans::from(Span::raw("j: moves down in the ROMS list")),
        Spans::from(Span::raw("k: moves up in the ROMS list")),
        Spans::from(Span::raw("i: switches to insert mode")),
        Spans::from(Span::raw("q: quits the program")),
        Spans::from(Span::raw("enter: launches selected ROM")),
        Spans::from(Span::raw("")),
        Spans::from(Span::styled("INSERT MODE:", Style::default().fg(Color::Yellow))),
        Spans::from(Span::raw("esc: switches to normal mode, not keeping changes")),
        Spans::from(Span::raw("enter: saves changes, updating ROMS directory")),
        Spans::from(Span::raw("")),
        Spans::from(Span::styled("IMPORTANT NOTE:", Style::default().fg(Color::Red))),
        Spans::from(Span::raw("entering a directory with too many children or with too long of names will not display properly")),
        Spans::from(Span::raw("entering a directory that requires administrative priveleges will also not work")),
    ].into();

    return Paragraph::new(paragraph_string)
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Controls")
                .title_alignment(Alignment::Center)
        )
        .style
        (
            Style::default()
        )
        .alignment(Alignment::Left)
        .wrap(tui::widgets::Wrap{ trim: true })
}

pub fn text_area<'a>(rom_dir: String) -> TextArea<'a>
{
    let mut text_editor: TextArea = TextArea::new(vec![rom_dir]);

    text_editor.set_block
    (
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("ROMS directory")
            .title_alignment(Alignment::Center)
    );

    text_editor.set_style
    (
        Style::default()
    );

    text_editor.set_cursor_style
    (
        Style::default()
    );

    text_editor.set_cursor_line_style
    (
        Style::default()
    );

    return text_editor;
}

pub fn normal_mode<'a>() -> Paragraph<'a>
{
    return Paragraph::new("NORMAL mode")
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Mode")
                .title_alignment(Alignment::Center)
        )
        .style
        (
            Style::default()
                .fg(Color::Yellow)
        )
}

pub fn insert_mode<'a>() -> Paragraph<'a>
{
    return Paragraph::new("INSERT mode")
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Mode")
                .title_alignment(Alignment::Center)
        )
        .style
        (
            Style::default()
                .fg(Color::Magenta)
        )
}

pub fn get_rom_dirs(dir: &String) -> Vec<String>
{
    use std::path::PathBuf;

    return fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .map(|n| n.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .iter()
        .filter
        (
            |n|
            {
                PathBuf::from(n).is_dir()
            }
        )
        .map
        (
            |n|
            {
                let n_string = n.to_string();

                let splits: Vec<&str> = n_string.split('\\').collect();
                return splits[splits.len() - 1].to_string();
            }
        )
        .collect::<Vec<String>>()
}

pub fn get_roms_from_dir<'a>(dir: &String) -> Vec<String>
{
    use std::path::PathBuf;

    return fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .map(|n| n.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .iter()
        .filter
        (
            |n|
            {
                !PathBuf::from(n).is_dir()
            }
        )
        .map
        (
            |n|
            {
                let n_string = n.to_string();

                let splits: Vec<&str> = n_string.split('\\').collect();
                return splits[splits.len() - 1].to_string();
            }
        )
        .collect::<Vec<String>>()
        // .iter()
        // .map(|n| ListItem::new(n.clone()))
        // .collect::<Vec<ListItem>>()
}
