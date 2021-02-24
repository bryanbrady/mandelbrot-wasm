mod utils;

extern crate wasm_timer;
use wasm_timer::Instant;

// use time::Instant;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const MAX_ITER: u32 = 1000;
const BAILOUT: f64 = 1000.0;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const PALETTE: [Color; 16] = [
    Color{r: 66,  g: 30,  b: 15,  a: 255},
    Color{r: 25,  g: 7,   b: 26,  a: 255},
    Color{r: 9,   g: 1,   b: 47,  a: 255},
    Color{r: 4,   g: 4,   b: 73,  a: 255},
    Color{r: 0,   g: 7,   b: 100, a: 255},
    Color{r: 12,  g: 44,  b: 138, a: 255},
    Color{r: 24,  g: 82,  b: 177, a: 255},
    Color{r: 57,  g: 125, b: 209, a: 255},
    Color{r: 134, g: 181, b: 229, a: 255},
    Color{r: 211, g: 236, b: 248, a: 255},
    Color{r: 241, g: 233, b: 191, a: 255},
    Color{r: 248, g: 201, b: 95,  a: 255},
    Color{r: 255, g: 170, b: 0,   a: 255},
    Color{r: 204, g: 128, b: 0,   a: 255},
    Color{r: 153, g: 87,  b: 0,   a: 255},
    Color{r: 106, g: 52,  b: 3,   a: 255}];

#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    look_at_x: f64,
    look_at_y: f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    zoom: f64,
    max_iter: u32,
    bailout: f64
}

fn color_lerp(a: &Color, b: &Color, t: f64) -> Color {
    let one_minus_t = 1.0 - t;
    Color {
        r: (one_minus_t * a.r as f64 + t * b.r as f64) as u8,
        g: (one_minus_t * a.g as f64 + t * b.g as f64) as u8,
        b: (one_minus_t * a.b as f64 + t * b.b as f64) as u8,
        a: (one_minus_t * a.a as f64 + t * b.a as f64) as u8,
    }
}

#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(w: u32, h: u32, x: f64, y: f64) -> Mandelbrot {
        utils::set_panic_hook();
        let zoom = 400.0;
        let x_min = x - (w as f64) / 2.0 / zoom;
        let x_max = x + (w as f64) / 2.0 / zoom;
        let y_min = y - (h as f64) / 2.0 / zoom;
        let y_max = y + (h as f64) / 2.0 / zoom;
        Mandelbrot {
            width: w,
            height: h,
            look_at_x: x,
            look_at_y: y,
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
            zoom: 400.0,
            max_iter: MAX_ITER,
            bailout: BAILOUT
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_max_iter(&mut self, iter: u32) {
        self.max_iter = iter;
    }

    pub fn set_bailout(&mut self, bailout: f64) {
        self.bailout = bailout;
    }

    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    pub fn set_camera(&mut self, client_x: u32, client_y: u32, zoom: f64) {
        self.zoom = zoom;
        let x = self.x_min + (client_x as f64 / self.width as f64) * (self.x_max - self.x_min);
        let y = self.y_min + (client_y as f64 / self.height as f64) * (self.y_max - self.y_min);
        self.look_at_x = x;
        self.look_at_y = y;
        self.x_min = x - (self.width as f64) / 2.0 / self.zoom;
        self.x_max = x + (self.width as f64) / 2.0 / self.zoom;
        self.y_min = y - (self.height as f64) / 2.0 / self.zoom;
        self.y_max = y + (self.height as f64) / 2.0 / self.zoom;
    }

    pub fn render(&self) -> Vec<u8> {
        let start = Instant::now();
        let mut data = Vec::new();

        let x_diff = self.x_max - self.x_min;
        let y_diff = self.y_max - self.y_min;

        for h in 0..self.height{
            for w in 0..self.width {
                let x0 = w as f64 / self.width as f64 * x_diff + self.x_min;
                let y0 = h as f64 / self.height as f64 * y_diff + self.y_min;
                let mut i = 0;
                let mut x = 0.0;
                let mut y = 0.0;
                let mut xx = 0.0;
                let mut yy = 0.0;
                while xx + yy <= self.bailout && i < self.max_iter {
                    y = 2.0 * x * y + y0;
                    x = xx - yy + x0;
                    xx = x * x;
                    yy = y * y;
                    i += 1;
                }
                if i == self.max_iter {
                    data.push(0);
                    data.push(0);
                    data.push(0);
                    data.push(255);
                } else {
                    let log_zn = f64::ln(xx + yy) / 2.0;
                    let mu = f64::ln(log_zn / std::f64::consts::LN_2) / std::f64::consts::LN_2;
                    let ii = (i + 1) as f64 - mu;
                    let color_idx = f64::floor(ii) as usize;
                    let a = &PALETTE[(color_idx) % PALETTE.len()];
                    let b = &PALETTE[(color_idx+1) % PALETTE.len()];
                    let c = color_lerp(a, b, ii - f64::floor(ii));
                    data.push(c.r);
                    data.push(c.g);
                    data.push(c.b);
                    data.push(c.a);
                }
            }
        }
        let end = Instant::now();
        console_log!("rendered in {:?} seconds", end - start);
        data
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let data = self.render();
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width, self.height)?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }
}
