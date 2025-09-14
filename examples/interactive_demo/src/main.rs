use eframe::egui;
use lsph::{
    geometry::Point,
    map::LearnedHashMap,
    models::LinearModel,
};
use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
struct LSPHDemo {
    // Core LSPH data structure
    spatial_map: LearnedHashMap<LinearModel<f64>, f64>,
    
    // UI state
    points: Vec<Point<f64>>,
    point_colors: HashMap<usize, egui::Color32>,
    
    // Input fields
    input_x: String,
    input_y: String,
    input_value: String,
    
    // Search parameters
    search_x: String,
    search_y: String,
    search_radius: f32,
    
    // Visualization settings
    point_size: f32,
    show_grid: bool,
    
    // Demo modes
    demo_mode: DemoMode,
    auto_generate: bool,
    generation_speed: f32,
    
    // Search results
    nearest_neighbor: Option<Point<f64>>,
    range_results: Vec<Point<f64>>,
    
    // Statistics
    total_points: usize,
    last_search_time: Option<std::time::Duration>,
}

#[derive(Default, PartialEq)]
enum DemoMode {
    #[default]
    Manual,
    RandomGeneration,
    NearestNeighbor,
    RangeQuery,
}

impl LSPHDemo {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            spatial_map: LearnedHashMap::new(),
            point_size: 4.0,
            show_grid: true,
            search_radius: 50.0,
            generation_speed: 10.0,
            input_x: "0.5".to_string(),
            input_y: "0.5".to_string(),
            input_value: "1.0".to_string(),
            search_x: "0.5".to_string(),
            search_y: "0.5".to_string(),
            ..Default::default()
        }
    }
    
    fn add_point(&mut self, x: f64, y: f64, value: f64) {
        let point = Point::new(x, y);
        
        // Add to LSPH
        let _existing = self.spatial_map.insert(point);
        
        // Add to visualization
        let index = self.points.len();
        self.points.push(point);
        
        // Assign a color based on value
        let color = self.value_to_color(value);
        self.point_colors.insert(index, color);
        
        self.total_points += 1;
    }
    
    fn value_to_color(&self, value: f64) -> egui::Color32 {
        let normalized = (value.abs() % 10.0) / 10.0;
        let hue = normalized * 360.0;
        egui::Color32::from_rgb(
            (hue.sin() * 127.0 + 128.0) as u8,
            ((hue + 120.0).to_radians().sin() * 127.0 + 128.0) as u8,
            ((hue + 240.0).to_radians().sin() * 127.0 + 128.0) as u8,
        )
    }
    
    fn generate_random_points(&mut self, count: usize) {
        let mut rng = rand::rng();
        for _ in 0..count {
            let x = rng.random_range(0.0..1.0);
            let y = rng.random_range(0.0..1.0);
            let value = rng.random_range(-10.0..10.0);
            self.add_point(x, y, value);
        }
    }
    
    fn find_nearest_neighbor(&mut self, x: f64, y: f64) {
        let query_point = [x, y];
        let start = std::time::Instant::now();
        
        match self.spatial_map.nearest_neighbor(&query_point) {
            Some(point) => {
                self.nearest_neighbor = Some(point);
            }
            None => {
                self.nearest_neighbor = None;
            }
        }
        
        self.last_search_time = Some(start.elapsed());
    }
    
    fn range_query(&mut self, x: f64, y: f64, radius: f64) {
        let query_point = [x, y];
        let start = std::time::Instant::now();
        
        match self.spatial_map.radius_range(&query_point, radius) {
            Some(results) => {
                self.range_results = results;
            }
            None => {
                self.range_results.clear();
            }
        }
        
        self.last_search_time = Some(start.elapsed());
    }
    
    fn clear_all(&mut self) {
        self.spatial_map = LearnedHashMap::new();
        self.points.clear();
        self.point_colors.clear();
        self.nearest_neighbor = None;
        self.range_results.clear();
        self.total_points = 0;
        self.last_search_time = None;
    }
    
    fn canvas_to_world(&self, canvas_pos: egui::Pos2, canvas_rect: egui::Rect) -> (f64, f64) {
        let x = (canvas_pos.x - canvas_rect.min.x) / canvas_rect.width();
        let y = 1.0 - (canvas_pos.y - canvas_rect.min.y) / canvas_rect.height();
        (x as f64, y as f64)
    }
    
    fn world_to_canvas(&self, x: f64, y: f64, canvas_rect: egui::Rect) -> egui::Pos2 {
        let canvas_x = canvas_rect.min.x + (x as f32) * canvas_rect.width();
        let canvas_y = canvas_rect.min.y + (1.0 - y as f32) * canvas_rect.height();
        egui::Pos2::new(canvas_x, canvas_y)
    }
}

impl eframe::App for LSPHDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-generation in random mode
        if self.auto_generate && self.demo_mode == DemoMode::RandomGeneration {
            if ctx.input(|i| i.time) as f32 % (1.0 / self.generation_speed) < 0.016 {
                self.generate_random_points(1);
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🗺️ LSPH Interactive Demo");
            ui.separator();
            
            ui.horizontal(|ui| {
                // Left panel - Controls
                ui.vertical(|ui| {
                    // Responsive control panel width (25-35% of available width)
                    let available_width = ui.available_size().x;
                    let control_width = (available_width * 0.3).clamp(250.0, 400.0);
                    ui.set_width(control_width);
                    
                    // Demo mode selection
                    ui.group(|ui| {
                        ui.label("Demo Mode:");
                        ui.radio_value(&mut self.demo_mode, DemoMode::Manual, "Manual Point Addition");
                        ui.radio_value(&mut self.demo_mode, DemoMode::RandomGeneration, "Random Generation");
                        ui.radio_value(&mut self.demo_mode, DemoMode::NearestNeighbor, "Nearest Neighbor Search");
                        ui.radio_value(&mut self.demo_mode, DemoMode::RangeQuery, "Range Query");
                    });
                    
                    ui.separator();
                    
                    match self.demo_mode {
                        DemoMode::Manual => {
                            ui.group(|ui| {
                                ui.label("Add Point:");
                                ui.horizontal(|ui| {
                                    ui.label("X:");
                                    ui.text_edit_singleline(&mut self.input_x);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Y:");
                                    ui.text_edit_singleline(&mut self.input_y);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Value:");
                                    ui.text_edit_singleline(&mut self.input_value);
                                });
                                
                                if ui.button("Add Point").clicked() {
                                    if let (Ok(x), Ok(y), Ok(value)) = (
                                        self.input_x.parse::<f64>(),
                                        self.input_y.parse::<f64>(),
                                        self.input_value.parse::<f64>(),
                                    ) {
                                        if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
                                            self.add_point(x, y, value);
                                        }
                                    }
                                }
                            });
                        }
                        
                        DemoMode::RandomGeneration => {
                            ui.group(|ui| {
                                ui.label("Random Generation:");
                                ui.checkbox(&mut self.auto_generate, "Auto Generate");
                                ui.horizontal(|ui| {
                                    ui.label("Speed:");
                                    ui.add(egui::Slider::new(&mut self.generation_speed, 0.1..=20.0));
                                });
                                
                                if ui.button("Generate 10 Points").clicked() {
                                    self.generate_random_points(10);
                                }
                                if ui.button("Generate 100 Points").clicked() {
                                    self.generate_random_points(100);
                                }
                            });
                        }
                        
                        DemoMode::NearestNeighbor => {
                            ui.group(|ui| {
                                ui.label("Nearest Neighbor Search:");
                                ui.horizontal(|ui| {
                                    ui.label("Query X:");
                                    ui.text_edit_singleline(&mut self.search_x);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Query Y:");
                                    ui.text_edit_singleline(&mut self.search_y);
                                });
                                
                                if ui.button("Find Nearest").clicked() {
                                    if let (Ok(x), Ok(y)) = (
                                        self.search_x.parse::<f64>(),
                                        self.search_y.parse::<f64>(),
                                    ) {
                                        if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
                                            self.find_nearest_neighbor(x, y);
                                        }
                                    }
                                }
                                
                                if let Some(nn) = &self.nearest_neighbor {
                                    ui.label(format!("Nearest: ({:.3}, {:.3})", nn.x(), nn.y()));
                                }
                            });
                        }
                        
                        DemoMode::RangeQuery => {
                            ui.group(|ui| {
                                ui.label("Range Query:");
                                ui.horizontal(|ui| {
                                    ui.label("Center X:");
                                    ui.text_edit_singleline(&mut self.search_x);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Center Y:");
                                    ui.text_edit_singleline(&mut self.search_y);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Radius:");
                                    ui.add(egui::Slider::new(&mut self.search_radius, 0.01..=0.5));
                                });
                                
                                if ui.button("Search Range").clicked() {
                                    if let (Ok(x), Ok(y)) = (
                                        self.search_x.parse::<f64>(),
                                        self.search_y.parse::<f64>(),
                                    ) {
                                        if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
                                            self.range_query(x, y, self.search_radius as f64);
                                        }
                                    }
                                }
                                
                                ui.label(format!("Found: {} points", self.range_results.len()));
                            });
                        }
                    }
                    
                    ui.separator();
                    
                    // Visualization settings
                    ui.group(|ui| {
                        ui.label("Visualization:");
                        ui.checkbox(&mut self.show_grid, "Show Grid");
                        ui.horizontal(|ui| {
                            ui.label("Point Size:");
                            ui.add(egui::Slider::new(&mut self.point_size, 1.0..=10.0));
                        });
                    });
                    
                    ui.separator();
                    
                    // Statistics
                    ui.group(|ui| {
                        ui.label("Statistics:");
                        ui.label(format!("Total Points: {}", self.total_points));
                        if let Some(time) = self.last_search_time {
                            ui.label(format!("Last Search: {:.2}ms", time.as_secs_f64() * 1000.0));
                        }
                    });
                    
                    ui.separator();
                    
                    if ui.button("Clear All").clicked() {
                        self.clear_all();
                    }
                });
                
                ui.separator();
                
                // Right panel - Canvas
                ui.vertical(|ui| {
                    // Calculate responsive canvas size based on available space
                    let available_size = ui.available_size();
                    let canvas_width = available_size.x - 20.0; // Leave some margin
                    let canvas_height = available_size.y - 40.0; // Leave space for label
                    
                    // Maintain square aspect ratio for better spatial visualization
                    let canvas_size = canvas_width.min(canvas_height).max(200.0); // Minimum 200px
                    let canvas_vec = egui::Vec2::splat(canvas_size);
                    
                    let (response, painter) = ui.allocate_painter(canvas_vec, egui::Sense::click());
                    let canvas_rect = response.rect;
                    
                    // Handle canvas clicks
                    if response.clicked() {
                        if let Some(click_pos) = response.interact_pointer_pos() {
                            let (world_x, world_y) = self.canvas_to_world(click_pos, canvas_rect);
                            
                            match self.demo_mode {
                                DemoMode::Manual => {
                                    self.add_point(world_x, world_y, 1.0);
                                }
                                DemoMode::NearestNeighbor => {
                                    self.find_nearest_neighbor(world_x, world_y);
                                    self.search_x = format!("{:.3}", world_x);
                                    self.search_y = format!("{:.3}", world_y);
                                }
                                DemoMode::RangeQuery => {
                                    self.range_query(world_x, world_y, self.search_radius as f64);
                                    self.search_x = format!("{:.3}", world_x);
                                    self.search_y = format!("{:.3}", world_y);
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    // Draw background
                    painter.rect_filled(canvas_rect, 0.0, egui::Color32::WHITE);
                    painter.rect_stroke(canvas_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                    
                    // Draw grid
                    if self.show_grid {
                        let grid_color = egui::Color32::from_gray(230);
                        for i in 1..10 {
                            let x = canvas_rect.min.x + (i as f32 / 10.0) * canvas_rect.width();
                            let y = canvas_rect.min.y + (i as f32 / 10.0) * canvas_rect.height();
                            
                            painter.line_segment(
                                [egui::Pos2::new(x, canvas_rect.min.y), egui::Pos2::new(x, canvas_rect.max.y)],
                                egui::Stroke::new(0.5, grid_color),
                            );
                            painter.line_segment(
                                [egui::Pos2::new(canvas_rect.min.x, y), egui::Pos2::new(canvas_rect.max.x, y)],
                                egui::Stroke::new(0.5, grid_color),
                            );
                        }
                    }
                    
                    // Draw points with responsive sizing
                    let scale_factor = (canvas_rect.width() / 400.0).clamp(0.5, 2.0); // Scale relative to 400px baseline
                    let scaled_point_size = self.point_size * scale_factor;
                    
                    for (i, point) in self.points.iter().enumerate() {
                        let canvas_pos = self.world_to_canvas(point.x(), point.y(), canvas_rect);
                        let color = self.point_colors.get(&i).copied().unwrap_or(egui::Color32::BLUE);
                        painter.circle_filled(canvas_pos, scaled_point_size, color);
                    }
                    
                    // Draw search query point with responsive sizing
                    if self.demo_mode == DemoMode::NearestNeighbor || self.demo_mode == DemoMode::RangeQuery {
                        if let (Ok(x), Ok(y)) = (self.search_x.parse::<f64>(), self.search_y.parse::<f64>()) {
                            if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
                                let query_pos = self.world_to_canvas(x, y, canvas_rect);
                                let scaled_query_size = 8.0 * scale_factor;
                                let scaled_stroke_width = 2.0 * scale_factor.sqrt();
                                painter.circle_stroke(query_pos, scaled_query_size, egui::Stroke::new(scaled_stroke_width, egui::Color32::RED));
                                
                                // Draw range circle for range queries
                                if self.demo_mode == DemoMode::RangeQuery {
                                    // Scale radius proportionally to canvas size
                                    let radius_pixels = self.search_radius * canvas_rect.width().min(canvas_rect.height());
                                    painter.circle_stroke(
                                        query_pos,
                                        radius_pixels,
                                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 100)),
                                    );
                                }
                            }
                        }
                    }
                    
                    // Highlight nearest neighbor with responsive sizing
                    if let Some(nn) = &self.nearest_neighbor {
                        let nn_pos = self.world_to_canvas(nn.x(), nn.y(), canvas_rect);
                        let highlight_size = scaled_point_size + 3.0 * scale_factor;
                        let highlight_stroke = 2.0 * scale_factor.sqrt();
                        painter.circle_stroke(nn_pos, highlight_size, egui::Stroke::new(highlight_stroke, egui::Color32::GREEN));
                    }
                    
                    // Highlight range query results with responsive sizing
                    for result in &self.range_results {
                        let result_pos = self.world_to_canvas(result.x(), result.y(), canvas_rect);
                        let result_highlight_size = scaled_point_size + 2.0 * scale_factor;
                        let result_stroke = 1.5 * scale_factor.sqrt();
                        painter.circle_stroke(result_pos, result_highlight_size, egui::Stroke::new(result_stroke, egui::Color32::YELLOW));
                    }
                    
                    ui.label("💡 Click on the canvas to interact!");
                });
            });
        });
        
        // Request repaint for animations
        if self.auto_generate {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0]) // Larger default size
            .with_min_inner_size([600.0, 400.0]) // Minimum window size
            .with_resizable(true) // Allow resizing
            .with_title("LSPH Interactive Demo"),
        ..Default::default()
    };
    
    eframe::run_native(
        "LSPH Demo",
        options,
        Box::new(|cc| Box::new(LSPHDemo::new(cc))),
    )
}