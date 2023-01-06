pub mod ui;

use std::fs::{self, File};
use std::io::Read;
use std::sync::mpsc;
use std::{io, thread};
use std::time::{Duration, Instant};
use tui::layout::Rect;
use tui::style::Modifier;
use tui::widgets::{Paragraph, List, ListItem, ListState, Tabs};
use tui::
{
    Terminal,
    style::Style,
    backend::CrosstermBackend,
};
use crossterm::
{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode}
};
use tui_textarea::TextArea;
use ui::Mode;

fn main() -> Result<(), std::io::Error>
{
    enable_raw_mode().expect("failed to enable raw mode");

    let (tx, rx) = mpsc::channel::<Event>();
    let tick_rate = Duration::from_millis(200);
    thread::spawn
    (
        move ||
        {
            let mut last_tick = Instant::now();
            loop
            {
                if last_tick.elapsed() >= tick_rate
                {
                    last_tick = Instant::now();
                }

                if event::poll(tick_rate).unwrap()
                {
                    if let Event::Key(key) = event::read().unwrap()
                    {
                        tx.send(Event::Key(key)).unwrap();
                    }
                }
            }
        }
    );

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut typing_mode: Mode = Mode::NORMAL;

    let mut rom_dir_file: File = match File::open(".\\rom_dir.json")
    {
        Ok(file) => file,
        Err(_) =>
        {
            File::create(".\\rom_dir.json").unwrap();
            File::open(".\\rom_dir.json").unwrap()
        }
    };

    let mut rom_dir: String = String::new();

    rom_dir_file.read_to_string(&mut rom_dir).unwrap();

    rom_dir = match rom_dir.len()
    {
        0 => "C:\\Users\\Gavin".to_string(),
        _ => serde_json::from_str(&rom_dir).unwrap()
    };

        // "C:\\Users\\Gavin\\Desktop\\emulators\\ROMS".to_string();

    let mut titles: Vec<String> = ui::get_rom_dirs(&rom_dir);
    let mut tab_index: usize = 0;

    let mut rom_list_items: Vec<ListItem> = ui::get_roms_from_dir(&(rom_dir.clone() + &"\\".to_string() + &titles[tab_index]))
        .iter()
        .map(|n| ListItem::new(n.clone()))
        .collect::<Vec<ListItem>>();

    let mut rom_list_state: ListState = ListState::default();

    let mut text_editor: TextArea = ui::text_area(rom_dir.clone());

    loop
    {
        terminal.draw
        (
            |rect|
            {
                let vertical_panel: Vec<Rect> = ui::vertical_panels(rect.size());
                let horizontal_panel: Vec<Rect> = ui::horizontal_panels(vertical_panel[2]);
                let vertical_panel2: Vec<Rect> = ui::vertical_panels2(horizontal_panel[1]);

                let title: Paragraph = ui::title();

                let tabs: Tabs = ui::tabs(titles.clone()).select(tab_index);

                let rom_list: List = ui::roms_list(rom_list_items.clone());
                let controls: Paragraph = ui::controls();

                let text_area = text_editor.widget();

                rect.render_widget(title, vertical_panel[0]);
                rect.render_widget(tabs, vertical_panel[1]);
                rect.render_stateful_widget(rom_list.clone(), horizontal_panel[0], &mut rom_list_state);
                rect.render_widget(controls, vertical_panel2[0]);
                rect.render_widget(text_area, vertical_panel2[1]);

                if typing_mode == Mode::NORMAL
                {
                    let mode: Paragraph = ui::normal_mode();

                    rect.render_widget(mode, vertical_panel2[2]);
                }
                else
                {
                    let mode: Paragraph = ui::insert_mode();

                   rect.render_widget(mode, vertical_panel2[2]);
                }
            }
        )?;

        if typing_mode == Mode::NORMAL
        {
            match rx.recv().expect("failed to read mpsc")
            {
                Event::Key(key) => match key.code
                {
                    KeyCode::Char('q') =>
                    {
                        break;
                    },
                    KeyCode::Char('i') =>
                    {
                        text_editor = ui::text_area("".to_string());
                        text_editor.set_cursor_style
                        (
                            Style::default()
                                .add_modifier(Modifier::REVERSED)
                        );

                        typing_mode = Mode::INSERT;
                    },
                    KeyCode::Char('j') =>
                    {
                        let index: i32 = match rom_list_state.selected()
                        {
                            Some(index) =>
                            {
                                if index >= rom_list_items.len() - 1
                                {
                                    0
                                }
                                else
                                {
                                    (index + 1) as i32
                                }
                            }
                            None => 0
                        };

                        rom_list_state.select(if index < rom_list_items.len() as i32 { Some(index as usize) } else { None });
                    },
                    KeyCode::Char('k') =>
                    {
                        let index: i32 = match rom_list_state.selected()
                        {
                            Some(index) =>
                            {
                                if index <= 0
                                {
                                    (rom_list_items.len() - 1) as i32
                                }
                                else
                                {
                                    (index - 1) as i32
                                }
                            }
                            None => rom_list_items.len() as i32 - 1
                        };

                        rom_list_state.select(if index >= 0 { Some(index as usize) } else { None });
                    },
                    KeyCode::Char('h') =>
                    {
                        if tab_index == 0
                        {
                            tab_index = titles.len();
                        }

                        tab_index -= 1;

                        rom_list_items = ui::get_roms_from_dir(&(rom_dir.clone() + &"\\".to_string() + &titles[tab_index]))
                            .iter()
                            .map(|n| ListItem::new(n.clone()))
                            .collect::<Vec<ListItem>>();

                        rom_list_state.select(None);
                    },
                    KeyCode::Char('l') =>
                    {
                        if tab_index == titles.len() - 1
                        {
                            tab_index = 0;
                        }
                        else
                        {
                            tab_index += 1;
                        }

                        rom_list_items = ui::get_roms_from_dir(&(rom_dir.clone() + &"\\".to_string() + &titles[tab_index]))
                            .iter()
                            .map(|n| ListItem::new(n.clone()))
                            .collect::<Vec<ListItem>>();

                        rom_list_state.select(None);
                    },
                    KeyCode::Enter =>
                    {
                        if let Some(rom_index) = rom_list_state.selected()
                        {
                            let rom_to_run: String = format!("{}\\{}\\{}", rom_dir, titles[tab_index], (&ui::get_roms_from_dir(&(rom_dir.clone() + &"\\".to_string() + &titles[tab_index]))[rom_index])).to_string();

                            opener::open(std::path::Path::new(&rom_to_run)).expect(&format!("failed to open:\n{}", rom_to_run).to_string());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        else
        {
            match rx.recv().expect("failed to read mpsc")
            {
                Event::Key(key) => match key.code
                {
                    KeyCode::Esc =>
                    {
                        typing_mode = Mode::NORMAL;

                        text_editor = ui::text_area(rom_dir.clone());
                        text_editor.set_cursor_style
                        (
                            Style::default()
                                .add_modifier(Modifier::HIDDEN)
                        );
                    },
                    KeyCode::Enter =>
                    {
                        rom_dir = text_editor.into_lines().join("");

                        titles = ui::get_rom_dirs(&rom_dir);

                        text_editor = ui::text_area(rom_dir.clone());
                        text_editor.set_cursor_style
                        (
                            Style::default()
                                .add_modifier(Modifier::HIDDEN)
                        );

                        fs::write(".\\rom_dir.json", serde_json::to_string(&rom_dir).unwrap()).unwrap();

                        typing_mode = Mode::NORMAL;
                    },
                    _ => 
                    {
                        text_editor.input(key);
                    }
                },
                _ => {}
            }
        }
    }

    disable_raw_mode().expect("failed to exit raw mode");
    terminal.clear()?;

    return Ok(());
}
