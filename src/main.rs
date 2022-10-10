#![allow(dead_code)]
#![allow(unused_imports)]
extern crate ggez;

mod key_mapper;
use crate::key_mapper::KeyMapper;
use crate::key_mapper::KeyEdge;

mod imgui_wrapper;
use crate::imgui_wrapper::ImGuiWrapper;

use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

// TODO: Follow this code example
// https://joetsoi.github.io/fix-your-timestep-rust-ggez/

pub struct GameState{
    pos_x: f32,
    pos_y: f32,
}

struct MainState {
    game_state: GameState,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    key_mapper: KeyMapper,
}

fn move_up(m: &mut GameState){
    m.pos_y -= 5.0;
}

fn move_down(m: &mut GameState){
    m.pos_y += 5.0;
}

fn move_left(m: &mut GameState){
    m.pos_x -= 5.0;
}

fn move_right(m: &mut GameState){
    m.pos_x += 5.0;
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let game_state = GameState{pos_x: 0.0,pos_y:0.0};
        let mut key_mapper = KeyMapper::new();
        key_mapper.insert(KeyCode::W, KeyMods::NONE, KeyEdge::HELD, "Move Up", move_up);
        key_mapper.insert(KeyCode::S, KeyMods::NONE, KeyEdge::HELD, "Move Down", move_down);
        key_mapper.insert(KeyCode::A, KeyMods::NONE, KeyEdge::HELD, "Move Left", move_left);
        key_mapper.insert(KeyCode::D, KeyMods::NONE, KeyEdge::HELD, "Move Right", move_right);
        for (key_name, fun_name) in &mut key_mapper{
            println!("Bound: {} to {}",fun_name,key_name);
        }
        let s = MainState {
            game_state,
            imgui_wrapper,
            hidpi_factor,
            key_mapper,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.key_mapper.update(&mut self.game_state, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(self.game_state.pos_x, self.game_state.pos_y),
                100.0,
                2.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

        // Render game ui
        {
            self.imgui_wrapper.render(ctx, self.hidpi_factor);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // Imgui update
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        // Imgui update
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
        self.game_state.pos_x = _x;
        self.game_state.pos_y = _y;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        // Imgui update
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.key_mapper.event(&mut self.game_state, keycode, keymods, KeyEdge::UP);
    }
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat:bool) {
        if !repeat {
            self.key_mapper.event(&mut self.game_state, keycode, keymods, KeyEdge::DOWN);
        }
    }
}

// Creates the window and event system
pub fn main() -> ggez::GameResult {
    let hidpi_factor: f32;
    {
        // Create a dummy window so we can get monitor scaling information
        let cb = ggez::ContextBuilder::new("", "");
        let (_ctx, events_loop) = &mut cb.build()?;
        hidpi_factor = events_loop.get_primary_monitor().get_hidpi_factor() as f32;
    }

    let cb = ggez::ContextBuilder::new("super_simple with imgui", "ggez")
        .window_setup(conf::WindowSetup::default().title("RoboFactory"))
        .window_mode(conf::WindowMode::default().dimensions(750.0, 500.0));
    let (ref mut ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx, hidpi_factor)?;
    event::run(ctx, event_loop, state)
}
