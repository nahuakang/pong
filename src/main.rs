mod elements;
mod game;

use game::{GameScene, WINDOW_HEIGHT, WINDOW_WIDTH};

use tetra::ContextBuilder;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameScene::new)
}
