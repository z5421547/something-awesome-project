use crate::{clap::syntax, journal::PageMode,sections::{practice::*,learn::*}};
use anyhow::Result;
use egui::ImageButton;
#[allow(unused)]
use egui::{include_image, Button, Image, ImageSource, Pos2, Sense, TextBuffer, Ui, Vec2};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use egui_console::{ConsoleBuilder, ConsoleEvent, ConsoleWindow};

use egui_extras::{install_image_loaders, syntax_highlighting::CodeTheme};

use crate::{
    file::*,
    helpers::link,
    journal::{Day::*, Journal, Page},
    scenario::render_scenario,
};
use std::{collections::HashMap, sync::LazyLock};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct OpenWindows {
    pub console: bool,
    pub journal: bool,
    pub learning_page: bool,
    pub practice_page: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ConsoleDemo {
    console: ConsoleWindow,
    tools_open: bool,
    #[serde(skip)]
    journal: Journal,
    #[serde(skip)]
    files: HashMap<String, File>,
    #[serde(skip)]
    file_text: String,
    open_windows: OpenWindows,
    theme: CodeTheme,
    #[serde(skip)]
    learn_data: LearnData,
    #[serde(skip)]
    practice_data: PracticeData
}

impl Default for ConsoleDemo {
    fn default() -> Self {
        Self {
            console: ConsoleBuilder::new().prompt(">> ").history_size(20).build(),
            tools_open: false,
            journal: Journal::new(),
            files: HashMap::new(),
            file_text: "".to_string(),
            open_windows: OpenWindows::default(),
            theme: CodeTheme::default(),
            learn_data: LearnData::default(),
            practice_data: PracticeData::default()
        }
    }
}

static JOURNAL_PAGES: LazyLock<Vec<Page>> = LazyLock::new(|| {
    vec![
                Page{
                    title: "It's a good day".into(),
                    content: "Hello testing 1 2 3".into(),
                    date: (27,9,24),
                    day_of_week: Friday,
                    mode: PageMode::Lazy(0)
                },
                Page{
                    title: "It's not a good day".into(),
                    content: "Such sadness".into(),
                    date: (28,9,24),
                    day_of_week: Saturday,
                    mode: PageMode::Lazy(0)
                },
                Page{
                    title: "It's amazing".into(),
                    content: "Hello world".into(),
                    date: (29,9,24),
                    day_of_week: Sunday,
                    mode: PageMode::Lazy(0)
                },
                Page{
                    title: "'A' day".into(),
                    content: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                    date: (30,9,24),
                    day_of_week: Monday,
                    mode: PageMode::Lazy(0)
                }]
});

static QUIZZES : LazyLock<Vec<Quiz>> = LazyLock::new(|| {
    vec![
        Quiz::new("Encryption")
        .add_question("What is encryption", vec!["A technique for disguising or hiding information like in another message or an object","A technique for transforming a message such that it can only be decoded by authorized parties.","A technique for irreversibly transforming information."], 'b')
        .add_question("One time pad encryption can be broken with enough computational power", vec!["True","False"], 'b')
        .add_question("An asymmetric key is one that is used by both the sender and receiver", vec!["True","False"], 'b'),

        Quiz::new("Password Quiz")
        .add_question("What is hashing?",vec!["A technique to irreversible transform a piece of data.", "A way of cooking potatoes"], 'a')
        .add_question("What is a rainbow table?", vec![
            "A sophisicated piece of furniture that disguises information",
            "A precomputed table of hash values that can be used to identify values like passwords from their hashes"], 'b')
        .add_question("What does salting help with?", 
        vec!["It makes computing the hash harder","It misleads the attacker into thinking the hash isn't useful","It changes the original value to thawt rainbow tables"], 'c')
        .add_question("What's the hash of 171?", 
        vec!["6d7eaf2918c90f15bdd1e6dd78b6785e26a9d040e6e3bf85846cbc07e922","54183f4323f377b737433a1e98229eadfdc686f93bab057ecb612daa9402b5","17242aed0751adb88388d165183d0ebec8345e29638bfd688afdbf91deb"], 'a'),
        Quiz::new("Man in the middle attacks")
        .add_question("If someone steals your phone and sends someone a message is that man in the middle attack?", 
        vec!["Yes because like other man in the middle attacks the person on the other end is convinced you are sending the messages.","No because they are not intercepting the communication but are making it themselves"], 'b')
        .add_question("What makes them particularly dangerous?", 
        vec!["They can achieve their aim without you even knowing anything has happened",
        "Because they involve someone getting your data. "], 'a')
        ,
        Quiz::new("Social engineering")
        .add_question("What's so dangerous about it", vec![
            "It's got the word engineering in it.",
            "It exploits the human psyche to succeed",
            "It's not it's actually a trick question it isn't"
        ], 'b')
        .add_question("If someone hacks into your messages is that social engineering?", vec!["Yes because it involves human communication","No because it is merely the aquisition of your data"], 'b')
    ]
});

impl ConsoleDemo {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_image_loaders(&cc.egui_ctx);
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut res: ConsoleDemo =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            res.journal.pages = JOURNAL_PAGES.clone();
            res.practice_data.quizzes = QUIZZES.clone();
            return res;
        }

        Default::default()
    }

    pub fn render_journal(&mut self, ctx: &egui::Context) {
        if !self.open_windows.journal {
            return;
        }

        let window = egui::Window::new("Journal")
            .default_size(Vec2 { x: 300.0, y: 500.0 })
            .min_size(Vec2 { x: 300.0, y: 500.0 })
            .resizable(true);

        if let Some(Page {
            title,
            content,
            date: (day, month, year),
            day_of_week,
            mode,
        }) = self.journal.get_current_page()
        {
            let content = match mode {
                PageMode::Immediate => content,
                PageMode::Lazy(x) => {
                    self.journal.update();
                    content[..x / 5].into()
                }
                PageMode::Loaded => content,
            };

            window.open(&mut self.open_windows.journal).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.strong(title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(format!("{} - {day}/{month}/{year}", day_of_week.as_str()));
                    });
                });
                ui.separator();
                ui.add_space(10.0);
                ui.label(content);
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::ImageButton::new(include_image!(
                            "../assets/left_arrow.png"
                        )))
                        .clicked()
                    {
                        self.journal.prev_page();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui
                            .add(egui::ImageButton::new(include_image!(
                                "../assets/right_arrow.png"
                            )))
                            .clicked()
                        {
                            self.journal.next_page();
                        }
                    });
                });
            });
        } else {
            window.open(&mut self.open_windows.journal).show(ctx, |ui| {
                ui.label("");
            });
        }
    }

    #[allow(unused)]
    pub fn render_code_editor(&mut self, ctx: &egui::Context) {
        //     let theme = &mut self.theme;
        //     // egui_extras::syntax_highlighting::CodeTheme::from_memory(ctx);

        //     ui.collapsing("Theme", |ui| {
        //         ui.group(|ui| {
        //             theme.ui(ui);
        //             // println!("HI");
        //             theme.clone().store_in_memory(ui.ctx());
        //         });
        //     });

        // let language = "rs";

        // let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        //     let mut layout_job = egui_extras::syntax_highlighting::highlight(
        //         ui.ctx(),
        //         theme,
        //         string,
        //         language,
        //     );
        //     layout_job.wrap.max_width = wrap_width;
        //     ui.fonts(|f| f.layout_job(layout_job))
        // };
        // egui::ScrollArea::vertical().show(ui, |ui| {
        //     ui.add(
        //         egui::TextEdit::multiline(&mut self.file_text)
        //             .font(egui::TextStyle::Monospace) // for cursor height
        //             .code_editor()
        //             .desired_rows(10)
        //             .lock_focus(true)
        //             .desired_width(f32::INFINITY)
        //             .layouter(&mut layouter),
        //     );
        // });
    }

    pub fn render_app_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.set_min_size(Vec2{x:100.0,y:100.0});
            // let img: Image<'_> = Image::new(include_image!("../assets/txt.png"))
            //         .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
            let plus: Image<'_> = Image::new(include_image!("../assets/plus.jpeg"))
                    .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
            let q_mark: Image<'_> = Image::new(include_image!("../assets/qMark2.jpeg"))
                    .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
            let puzzle_piece: Image<'_> = Image::new(include_image!("../assets/puzzlePiece.jpeg"))
                    .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });

            let buttons = ["Learn the basics", "Practice", "Extra resources","Help"].into_iter().skip(1)
            .zip([puzzle_piece,plus,q_mark].into_iter().map(|img| ImageButton::new(img.clone())));

            for (title,button) in buttons {
                ui.vertical(|ui| {
                    ui.add(button);
                    ui.label(title);
                });
                ui.add_space(50.0);
            }

            // for title in ["Learn the basics", "Practice", "Extra resources","Help"] {
            //     let image_button = ImageButton::new(img.clone());
            //     ui.vertical(|ui| {
            //         ui.add(image_button);
            //         ui.label(title);
            //     });
            //     ui.add_space(50.0);
            // }
        });
    }

    /*
    Renders a scenario in which the main protagonist plans an attack on his past employer.
     */
    pub fn render_scenario(&mut self, ctx: &egui::Context) {
        let window = egui::Window::new("Scenario")
            .default_size(Vec2 { x: 300.0, y: 500.0 })
            .min_size(Vec2 { x: 300.0, y: 500.0 })
            .resizable(true);

        let title = "Scenario 1";
        let weakness = "This week Jerry plugged a USB killer into his computer again. I think this is the 3rd time he's done but he never learns. So corporate decided from that we all needed to have security seminar but I have a strong feeling he'll do it again";
        let (day, month, year, day_of_week) = (1, 1, 1, Monday);

        let idea = "I'll leave a USB programmed to work as a keylogger on his car tomorrow with the hope he might try it out.";

        let consequence = "It didn't work they seemed to installed a security patch to limit device access via USB connections.";

        let learning = "I can't just chuck ideas at the wall hoping one might work I'll need to do a little recon before I try next time.";

        window
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong(title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(format!("{} - {day}/{month}/{year}",day_of_week.as_str()));
                });
            });
            ui.separator();
            ui.add_space(10.0);
            for text in [weakness,idea,consequence,learning] {
                ui.label(text);
                ui.separator();
            }
            ui.collapsing("extra links", |ui| {
                link(ui,"https://en.wikipedia.org/wiki/Stuxnet");
                link(ui,"https://en.wikipedia.org/wiki/2008_malware_infection_of_the_United_States_Department_of_Defense")
            });
        });
    }
}

impl eframe::App for ConsoleDemo {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.set_max_size(Vec2{x:100.0,y:100.0});
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32") | true;
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut console_response: ConsoleEvent = ConsoleEvent::None;
            egui::Window::new("Console Window")
                .default_height(500.0)
                .resizable(true)
                .open(&mut self.open_windows.console)
                .show(ctx, |ui| {
                    console_response = self.console.draw(ui);
                });

            if let ConsoleEvent::Command(command) = console_response {
                let resp = match self.dispatch(&command, ctx) {
                    Err(e) => {
                        if let Some(original_error) = e.downcast_ref::<clap::error::Error>() {
                            format!("{}", original_error)
                        } else if e.backtrace().status()
                            == std::backtrace::BacktraceStatus::Captured
                        {
                            format!("{} {}", e, e.backtrace())
                        } else {
                            format!("{}", e)
                        }
                    }

                    Ok(string) => string, // continue
                };
                if !resp.is_empty() {
                    self.console.write(&resp);
                }
                self.console.prompt();
            }
            let mut tools_window = egui::Window::new("tools")
                .default_height(500.0)
                .default_width(300.0)
                .resizable(false);

            self.render_journal(ctx);
            // self.render_scenario(ctx);

            learn_button(ui,&mut self.open_windows);
            practice_button(ui, &mut self.open_windows);
            practice_window(ui, &mut self.practice_data,&mut self.open_windows);

            if self.open_windows.learning_page {
                learn_window(ui,&mut self.learn_data,&mut self.open_windows);
            }
            // learn_button(ui,&mut self.open_windows);

            // self.render_app_buttons(ui);

            let mouse_response = ui.interact_bg(Sense::click());

            if mouse_response.clicked_by(egui::PointerButton::Secondary) {
                if self.tools_open {
                    self.tools_open = false;
                } else if let Some(pos) = ctx.pointer_interact_pos() {
                    tools_window = tools_window.current_pos(pos);
                    self.tools_open = true;
                }
            } else if mouse_response.clicked_by(egui::PointerButton::Primary) {
                self.tools_open = false;
            }

            if self.tools_open {
                tools_window.show(ctx, |tools_ui| {
                    tools_ui.menu_button("open", |ui| {
                        if !self.open_windows.console && ui.button("console").clicked() {
                            self.open_windows.console = true;
                            self.tools_open = false;
                        }

                        if ui.button("journal").clicked() {
                            self.open_windows.journal = true;
                        }
                    });

                    tools_ui.menu_button("create", |ui| {
                        if ui.button("file").clicked() {
                            println!("Not yet implemented");
                            // let mut file_label = "untitled".to_string();
                            // if self.files.contains_key(&file_label){
                            //     let mut n = 1;
                            //     while self.files.contains_key(&(file_label.clone()+n.to_string().as_str())){
                            //         n+=1;
                            //     }
                            //     file_label += n.to_string().as_str();
                            // }
                            // self.files.insert(file_label.clone(), File::new(FileIcon::Text,file_label));
                        }
                    });
                });
            }
        });
    }
}
impl ConsoleDemo {
    pub fn dispatch(&mut self, line: &str, ctx: &egui::Context) -> Result<String> {
        let args = line.split_whitespace();
        // parse with clap
        let matches = syntax().try_get_matches_from(args)?;
        // execute the command
        match matches.subcommand() {
            Some(("cd", args)) => {
                let dir = args.get_one::<String>("directory").unwrap();
                std::env::set_current_dir(dir)?;
                let cwd = std::env::current_dir()?;
                Ok(format!("Current working directory: {}", cwd.display()))
            }
            Some(("dark", _)) => {
                //  let ctx = egui::Context::default();
                ctx.set_visuals(egui::Visuals::dark());
                Ok("Dark mode enabled".to_string())
            }
            Some(("light", _)) => {
                //   let ctx = egui::Context::default();
                ctx.set_visuals(egui::Visuals::light());
                Ok("Light mode enabled".to_string())
            }
            Some(("quit", _)) => {
                //   let ctx = egui::Context::default();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                Ok("Bye".to_string())
            }
            Some(("clear_screen", _)) => {
                self.console.clear();
                Ok("".to_string())
            }
            Some(("dir", args)) => {
                let filter = if let Some(filter) = args.get_one::<String>("filter") {
                    filter.clone()
                } else {
                    "".to_string()
                };
                let entries = std::fs::read_dir(".")?;
                let mut result = String::new();
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.display().to_string().contains(filter.as_str()) {
                        result.push_str(&format!("{}\n", path.display()));
                    }
                }
                Ok(result)
            }
            Some(("history", _)) => {
                let history = self.console.get_history();
                let mut result = String::new();
                for (i, line) in history.iter().enumerate() {
                    result.push_str(&format!("{}: {}\n", i, line));
                }
                Ok(result)
            }
            Some(("clear_history", _)) => {
                self.console.clear_history();
                Ok("".to_string())
            }
            Some(("kill", args)) => {
                if let Some(arg) = args.get_one::<String>("application") {
                    match arg.as_str() {
                        "console" => self.open_windows.console = false,
                        "journal" => self.open_windows.journal = false,
                        _ => {}
                    };
                    Ok("".into())
                } else {
                    Err(anyhow::Error::msg("No application given"))
                }
            }
            _ => Ok("Unknown command".to_string()),
        }
    }
}
