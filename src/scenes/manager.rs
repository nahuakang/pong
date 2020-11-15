use tetra::{Context, State};
use tetra::window;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Quit,
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new(initial_scene: Box<dyn Scene>) -> tetra::Result<Self> {
        Ok(SceneManager {
            scenes: vec![initial_scene],
        })
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(scene) => {
                    self.scenes.push(scene);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
                Transition::Quit => window::quit(ctx)
            }
            None => window::quit(ctx)
        }

        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(ctx)? {
                Transition::None => {},
                Transition::Push(scene) => {
                    self.scenes.push(scene);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
                Transition::Quit => window::quit(ctx),
            }
            None => window::quit(ctx)
        }

        Ok(())
    }
}
