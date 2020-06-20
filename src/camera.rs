use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    fn new() -> Camera {
        let vh = 2.0;
        let vw = 2.0 * (16.0 / 9.0);
        let fl = 1.0;
        let orig = Vec3::zero();
        let hor = Vec3::new(vw, 0.0, 0.0);
        let vert = Vec3::new(0.0, vh, 0.0);
        let ll = orig - hor / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, fl);
        Camera {
            viewport_height: vh,
            viewport_width: vw,
            focal_length: fl,
            origin: orig,
            horizontal: hor,
            vertical: vert,
            lower_left_corner: ll,
        }
    }
}
