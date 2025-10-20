use egui_code_editor::{CodeEditor, ColorTheme, Completer, Syntax};
//use egui_code_editor::{CodeEditor, Syntax, ColorTheme, TokenType};

// fn color(token: TokenType) -> Color {
//     match token {
//         TokenType::Comment(_) => Color::Grey37,
//         TokenType::Function => Color::Yellow3b,
//         TokenType::Keyword => Color::IndianRed1c,
//         TokenType::Literal => Color::NavajoWhite1,
//         TokenType::Numeric(_) => Color::MediumPurple,
//         TokenType::Punctuation(_) => Color::Orange3,
//         TokenType::Special => Color::Cyan,
//         TokenType::Str(_) => Color::Green,
//         TokenType::Type => Color::GreenYellow,
//         TokenType::Whitespace(_) => Color::White,
//         TokenType::Unknown => Color::Pink1,
//         TokenType::Hyperlink => Color::Blue,
//     }
// }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemoApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    completer: Completer,

    #[serde(skip)]
    code: String,

    #[serde(skip)]
    editor: CodeEditor,
}

impl Default for DemoApp {
    fn default() -> Self {
        let syntax = Syntax::rust();
        let completer = Completer::new_with_syntax(&syntax).with_user_words();
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            completer,
            code: "fn main() {\n    println!(\"Hello World!\");\n}".to_owned(),
            editor: CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GRUVBOX)
                //.with_syntax(completer.syntax)
                .with_numlines(true),
        }
    }
}

impl DemoApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for DemoApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe Demo");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_Demo/blob/main/",
                "Source code."
            ));

            ui.separator();

            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.code =
                            std::fs::read_to_string(path.display().to_string()).unwrap_or_default();
                    }
                }
                ui.separator();
            }


            self.editor
                .show_with_completer(ui, &mut self.code, &mut self.completer);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
