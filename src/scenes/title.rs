use crate::scenes::game::GameScene;
use crate::scenes::manager::{Scene, Transition};
use crate::scenes::game::{WINDOW_HEIGHT, WINDOW_WIDTH};

use tetra::Context;
use tetra::graphics::{self, Color};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;

pub struct TitleScene {
    pub title: Text,
    pub pos: Vec2<f32>,
}

impl TitleScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let title = Text::new(
            "Welcome to PONG!",
            Font::vector(ctx, "./resources/DaysOne-Regular.ttf", 30.0)?,
        );

        println!("Text bounds are {:?}", title.get_bounds(ctx));

        Ok(TitleScene{
            title,
            pos: Vec2::new((WINDOW_WIDTH - 16.0 * 16.0) / 2.0, (WINDOW_HEIGHT - 30.0) / 2.0),
        })
    }
}

impl Scene for TitleScene {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.2, 0.2, 0.2));
        graphics::draw(ctx, &self.title, self.pos);

        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_released(ctx, Key::Enter) || input::is_key_released(ctx, Key::Space) {
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else if input::is_key_released(ctx, Key::Escape) {
            Ok(Transition::Quit)
        } else {
            Ok(Transition::None)
        }
    }
}
