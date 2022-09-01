// the hsv struct and its implementations

#[derive(Debug)]
pub struct Hsv {
    hue: f32, //0 - 360
    sat: f32, //0 - 1
    val: f32, //0 - 1
}

impl Hsv {
    pub fn new(h: f32, s: f32, v: f32) -> Hsv {
        Hsv {
            hue: h.abs()%360.0,
            sat: s.abs()%1.0,
            val: v.abs()%1.0,
        }
    }
    pub fn get_components(&self) -> (f32, f32, f32) {
        (self.hue, self.sat, self.val)
    }
}