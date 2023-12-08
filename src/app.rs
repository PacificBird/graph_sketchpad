use egui_plot::{Line, Plot, Points};

#[derive(PartialEq, Eq)]
pub enum Selection {
    Vertex,
    Edge,
}

pub struct TemplateApp {
    selected: Selection,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            selected: Selection::Vertex,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.selectable_value(&mut self.selected, Selection::Vertex, "V");
            ui.selectable_value(&mut self.selected, Selection::Edge, "E");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let plt = Plot::new("plot");
            let plt = plt.allow_zoom([false, false]);
            let plt = plt.show_grid([false, false]);
            let plt = plt.show_axes([false, false]);
            let plt = plt.allow_scroll(false);
            let plt = plt.allow_drag(false);
            let plt = plt.show_x(false);
            let plt = plt.show_y(false);
            plt.show(ui, |plot_ui| {})
        });
    }
}

struct Vertex {
    x: f64,
    y: f64,
    color: String,
}
struct Edge {
    color: String,
}

struct GraphPlot {
    graph: petgraph::graph::UnGraph<Vertex, Edge>,
}

impl GraphPlot {}
