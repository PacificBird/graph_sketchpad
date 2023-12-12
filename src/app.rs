use egui_plot::{Line, Plot, Points};

#[derive(PartialEq, Eq)]
pub enum Selection {
    Vertex,
    Edge,
    Delete,
}

pub struct TemplateApp {
    selected: Selection,
    graph: petgraph::graph::UnGraph<Vertex, Edge>,
    drag: Option<petgraph::graph::NodeIndex>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            selected: Selection::Vertex,
            graph: petgraph::graph::Graph::new_undirected(),
            drag: None,
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

        let panel_response = egui::SidePanel::left("side_panel")
            .show(ctx, |ui| {
                ui.selectable_value(&mut self.selected, Selection::Vertex, "V");
                ui.selectable_value(&mut self.selected, Selection::Edge, "E");
                ui.selectable_value(&mut self.selected, Selection::Delete, "D");
            })
            .response;

        let plot_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                let plt = Plot::new("plot");
                let plt = plt.allow_zoom([false, false]);
                let plt = plt.show_grid([false, false]);
                let plt = plt.show_axes([false, false]);
                let plt = plt.allow_scroll(false);
                let plt = plt.allow_drag(false);
                let plt = plt.allow_boxed_zoom(false);
                plt.show(ui, |plot_ui| {
                    for node_idx in self.graph.node_indices() {
                        plot_ui.points({
                            let node = self.graph.node_weight(node_idx).expect("No node weight");
                            node.to_points(plot_ui.transform())
                                .color(node.color)
                                .radius(5.0)
                        })
                    }
                    for edge_idx in self.graph.edge_indices() {
                        let (i1, i2) = self
                            .graph
                            .edge_endpoints(edge_idx)
                            .expect("There must be an edge at this index");
                        let pos1 = self
                            .graph
                            .node_weight(i1)
                            .expect("There must be a node at this index")
                            .to_point(plot_ui.transform());
                        let point1 = egui_plot::PlotPoint::new(pos1.x, pos1.y);
                        let pos2 = self
                            .graph
                            .node_weight(i2)
                            .expect("There must be a node at this index")
                            .to_point(plot_ui.transform());
                        let point2 = egui_plot::PlotPoint::new(pos2.x, pos2.y);
                        if i1 == i2 {
                            plot_ui.line(
                                egui_plot::Line::new(circle(pos1, 0.0005)).stroke(
                                    egui::Stroke::new(
                                        2.0,
                                        self.graph
                                            .edge_weight(edge_idx)
                                            .expect("There must be an edge at this index")
                                            .color,
                                    ),
                                ),
                            );
                        }
                        plot_ui.line(
                            egui_plot::Line::new(egui_plot::PlotPoints::Owned(vec![
                                point1, point2,
                            ]))
                            .stroke(egui::Stroke::new(
                                2.0,
                                self.graph
                                    .edge_weight(edge_idx)
                                    .expect("There must be an edge at this index")
                                    .color,
                            )),
                        );
                    }
                })
            })
            .response;
        let plot_response = plot_response.interact(egui::Sense::click().union(egui::Sense::drag()));

        match self.selected {
            Selection::Vertex => {
                if let Some(pos) = plot_response.interact_pointer_pos() {
                    if plot_response.clicked() {
                        println!("clicked!");
                        let idx = get_close_node(pos, &self.graph);
                        if self.drag.is_some() {
                            self.drag = None;
                        } else if let Some(i) = idx {
                            self.drag = Some(petgraph::graph::NodeIndex::new(i));
                        } else {
                            self.graph
                                .add_node(Vertex::new(pos, egui::Color32::from_white_alpha(255)));
                            println!("{}", self.graph.node_count());
                        }
                    }
                }
                if let (Some(i), Some(pos)) = (self.drag, plot_response.hover_pos()) {
                    self.graph
                        .node_weight_mut(i)
                        .expect("No node weight")
                        .move_to(pos);
                }
            }
            Selection::Edge => {
                if let Some(pos) = plot_response.interact_pointer_pos() {
                    println!("{:?}, {}", self.drag, self.graph.edge_count());
                    if plot_response.clicked() {
                        let idx = get_close_node(pos, &self.graph);
                        if let Some(i) = self.drag {
                            if let Some(i2) = idx {
                                self.graph.add_edge(
                                    i,
                                    petgraph::graph::NodeIndex::new(i2),
                                    Edge::new(egui::Color32::from_white_alpha(255)),
                                );
                            }
                            self.drag = None;
                        } else if let Some(i) = idx {
                            println!("new drag");
                            self.drag = Some(petgraph::graph::NodeIndex::new(i));
                        }
                    }
                }
            }
            Selection::Delete => {
                if let Some(pos) = plot_response.interact_pointer_pos() {
                    let idx = get_close_node(pos, &self.graph);
                    if plot_response.clicked() {
                        if let Some(i) = self.drag {
                            println!("no new drag");
                            if let Some(i2) = idx {
                                if let Some(e) =
                                    self.graph.find_edge(i, petgraph::graph::NodeIndex::new(i2))
                                {
                                    self.graph.remove_edge(e);
                                }
                            }
                            self.drag = None;
                        } else if let Some(i) = idx {
                            println!("new drag");
                            self.drag = Some(petgraph::graph::NodeIndex::new(i));
                        }
                    }
                    if plot_response.secondary_clicked() {
                        if let Some(i) = idx {
                            self.graph.remove_node(petgraph::graph::NodeIndex::new(i));
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pos: egui::Pos2,
    color: egui::Color32,
}
impl Vertex {
    fn new(pos: egui::Pos2, color: egui::Color32) -> Self {
        Self { pos, color }
    }
    fn to_points(self, transform: &egui_plot::PlotTransform) -> egui_plot::Points {
        egui_plot::Points::new(egui_plot::PlotPoints::Owned(vec![
            transform.value_from_position(self.pos)
        ]))
    }
    fn to_point(self, transform: &egui_plot::PlotTransform) -> egui_plot::PlotPoint {
        transform.value_from_position(self.pos)
    }
    // fn translate(&mut self, delta: egui::Vec2) {
    //     (*self).pos = self.pos + egui::Vec2::new(10.0, 10.0);
    // }
    fn move_to(&mut self, pos: egui::Pos2) {
        (*self).pos = pos;
    }
}
struct Edge {
    color: egui::Color32,
}
impl Edge {
    fn new(color: egui::Color32) -> Self {
        Self { color }
    }
}

fn get_close_node(
    pos: egui::Pos2,
    graph: &petgraph::graph::UnGraph<Vertex, Edge>,
) -> Option<usize> {
    graph.node_indices().position(|x| {
        graph
            .node_weight(x)
            .expect("No node weight")
            .pos
            .distance(pos)
            < 15.0
    })
}

fn circle(circ_pos: egui_plot::PlotPoint, radius: f32) -> egui_plot::PlotPoints {
    let points: Vec<egui_plot::PlotPoint> = (0..=16)
        .map(|z| {
            let r = radius;
            let offset = r / f32::sqrt(2.0);
            let step = 2.0 * std::f32::consts::PI / 16.0;
            egui_plot::PlotPoint::new(
                (circ_pos.x as f32 + offset) + (r * f32::cos(step * z as f32)),
                (circ_pos.y as f32 + offset) + (r * f32::sin(step * z as f32)),
            )
        })
        .map(|pos| egui_plot::PlotPoint::new(pos.x, pos.y))
        .collect();
    egui_plot::PlotPoints::Owned(points)
}
