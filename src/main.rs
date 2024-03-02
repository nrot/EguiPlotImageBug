use egui::vec2;

struct TestImage {
    texture: egui::TextureHandle,
    name: String,
}

struct TestApp {
    textures: Vec<TestImage>,
}

fn generate_image()->egui::ColorImage{
    egui::ColorImage { size: [2048, 2048], pixels: (0..(2048*2048)).map(|i|{
        let x = i % 2048;
        let y = i / 2048;
        if (x/64) % 2 != (y/64) % 2{
            egui::Color32::WHITE
        } else {
            egui::Color32::BLACK
        }
    }).collect::<Vec<_>>() }
}

impl TestApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        
        let textures = (1..=10)
            .take(2)
            .map(|i| {
                let name = format!("image_{}", i);
                TestImage {
                    texture: cc.egui_ctx.load_texture(
                        name.clone(),
                        generate_image(),
                        egui::TextureOptions::NEAREST,
                    ),
                    name,
                }
            })
            .collect::<Vec<_>>();
        Self { textures }
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Plot").show(ctx, |ui| {
            egui_plot::Plot::new("Images").show(ui, |plot| {
                self.textures.iter().enumerate().for_each(|(i, img)| {
                    let t = &img.texture;
                    // let size = vec2(t.aspect_ratio(), 1.0);
                    let s2 = t.size_vec2();
                    let pi = egui_plot::PlotImage::new(
                        t,
                        egui_plot::PlotPoint {
                            x: s2.x as f64 * i as f64,
                            y: 0.0,
                        },
                        s2,
                    )
                    .rotate(std::f64::consts::PI / 2.0)
                    .name(img.name.clone());
                    plot.image(pi);
                });
            });
        });
    }
}

fn main() {
    let native_opt = eframe::NativeOptions {
        hardware_acceleration: eframe::HardwareAcceleration::Required,

        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Inspect",
        native_opt,
        Box::new(|cc| Box::new(TestApp::new(cc))),
    )
    .unwrap();
}
