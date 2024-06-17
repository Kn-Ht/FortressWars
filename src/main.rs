#![allow(non_snake_case)]

mod game;
mod textures;

use game::Game;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum GameState {
    Menu,
    Game,
    Paused
}

#[macroquad::main("Fortress Wars")]
async fn main() { 
    let mut game = Game::new().await;
    let mut state = GameState::Game;

    loop {
        match state {
            GameState::Menu => game.menu(&mut state),
            GameState::Game => {
                game.update();
                game.ui();
                game.draw();
            },
            GameState::Paused => {
                game.ui();
                game.draw_paused();
            }
        }

        game.ui();
        game.draw();

        next_frame().await;
    }
}
