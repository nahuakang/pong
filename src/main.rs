use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    paddle_texture: Texture,
    paddle_position: Vec2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;
        let paddle_position =
            Vec2::new(16.0, (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0);
        Ok(GameState {
            paddle_texture,
            paddle_position,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Clear to a specific background color
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        // Draw texture
        graphics::draw(ctx, &self.paddle_texture, self.paddle_position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
            self.paddle_position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.paddle_position.y += PADDLE_SPEED;
        }

        Ok(())
    }
}
