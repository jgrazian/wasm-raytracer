#![feature(test)]

use std::path::Path;

use wasm_raytracer::*;

fn main() {
    let mut r = Renderer::new(800);
    r.scene(PerlinSpheres {});
    r.render(100);
    r.write_image(Path::new(r"out.png"));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_raytrace(b: &mut Bencher) {
        let mut r = Renderer::new(10);
        r.scene(RandomScene {});

        b.iter(|| r.render(10));
    }
}
