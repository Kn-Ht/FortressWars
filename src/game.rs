    use macroquad::prelude::*;

    use crate::{textures::Textures, GameState};

    pub struct Game {
        camera: Camera2D,
        textures: Textures,
    }

    const MAP_SIZE: Vec2 = vec2(2000.0, 100.0);

    impl Game {
        pub async fn new() -> Self {
            Self {
                camera: Camera2D {
                    target: MAP_SIZE / 2.0,
                    rotation: 0.0,
                    zoom: vec2(1.0, 1.0),
                    offset: Vec2::ZERO,
                    render_target: None,
                    viewport: None,
                },
                textures: Textures::load().await,
            }
        }
        pub fn menu(&mut self, state: &mut GameState) {
            egui_macroquad::ui(|ctx| {
                egui::CentralPanel::default()
                .show(ctx, |ui| {});
            });
        }
        pub fn ui(&mut self) {
            egui_macroquad::ui(|ctx| {

            });
        }
        pub fn draw_paused(&mut self) {

        }
        pub fn update(&mut self) {
            let dt = get_frame_time();
            const MOVE_SPEED: f32 = 100.0;

            let move_speed = MOVE_SPEED * dt;

            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                self.camera.target.x = (self.camera.target.x - move_speed).clamp(0.0, MAP_SIZE.x);
            }
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                self.camera.target.x = (self.camera.target.x + move_speed).clamp(0.0, MAP_SIZE.x);
            }
        }
        pub fn draw(&mut self) {
            clear_background(BLACK);

            set_camera(&self.camera);
            {

            }
            set_default_camera();

            // Draw UI and (if debug) FPS counter
            egui_macroquad::draw();
            if cfg!(debug_assertions) {
                draw_text(&format!("FPS: {}", get_fps()), 10., 20.0, 20.0, GREEN);
            }
        }
    }