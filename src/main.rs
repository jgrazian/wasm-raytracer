use std::path::Path;
use wasm_raytracer::{RandomScene, Renderer};

fn main() {
    let mut r = Renderer::new(300, 200);
    r.scene(RandomScene {});
    r.render(10);
    r.write_image(Path::new(r"out.png"));
}
