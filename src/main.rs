mod elements;
mod scenes;

use scenes::game::{GameScene, WINDOW_HEIGHT, WINDOW_WIDTH};
use scenes::manager::SceneManager;
use scenes::title::TitleScene;

use tetra::ContextBuilder;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| {
            let title_scene = TitleScene::new(ctx)?;
            Ok(SceneManager::new(Box::new(title_scene))?)
        })
}
