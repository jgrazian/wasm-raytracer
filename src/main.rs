#![feature(test)]

use std::path::Path;
use wasm_raytracer::{RandomScene, Renderer};

fn main() {
    let mut r = Renderer::new(300, 200);
    r.scene(RandomScene {});
    r.render(10);
    r.write_image(Path::new(r"out.png"));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_raytrace(b: &mut Bencher) {
        let mut r = Renderer::new(10, 10);
        r.scene(RandomScene {});

        b.iter(|| r.render(10));
    }
}
