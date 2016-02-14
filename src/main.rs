extern crate sdl;

use std::cmp::{min, max};
use std::env;
use std::fs::File;
use std::fmt;
use std::io::Read;
use std::str::FromStr;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

const WINDOW_WIDTH : usize = 800;
const WINDOW_HEIGHT : usize = 600;

#[derive(Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

fn get_default_map() -> String {
    r#"0 0 0 0 0 0 0 0 0
0 1 1 1 -1 1 1 1 0
0 1 2 2 2 2 2 1 0
0 1 2 0 0 0 2 1 0
0 -1 2 0 4 0 2 -1 0
0 1 2 0 0 0 2 1 0
0 1 2 2 2 2 2 1 0
0 1 1 1 -1 1 1 1 0
0 0 0 0 0 0 0 0 0"#.to_owned()
}

fn parse_map(map: String) -> Vec<Vec<i16>> {
    let mut ret = vec!();

    for line in map.split('\n') {
        ret.insert(0, line.split(' ').map(|x| i16::from_str(x).unwrap()).collect());
    }
    ret
}

fn draw_pixel(x: i16, y: i16, color: sdl::video::Color, screen: &sdl::video::Surface) {
    screen.fill_rect(Some(sdl::Rect {
        x: x,
        y: y,
        w: 1,
        h: 1,
    }), color);
}

fn draw_line(a: &Point, b: &Point, screen: &sdl::video::Surface, color: &sdl::video::Color) {
    let x_min = min(a.x, b.x);
    let x_max = max(a.x, b.x);
    let y_min = min(a.y, b.y) as f32;
    let y_max = max(a.y, b.y) as f32;
    let step = (y_max - y_min) / (x_max as f32 - x_min as f32);
    let mut y = a.y as f32;
    let y_count = if a.y < b.y { 1f32 } else if a.y == b.y { 0f32 } else { -1f32 };
    let x_count = if a.x < b.x { 1 } else { -1 };
    let mut x = a.x;

    loop {
        if x < x_min || x > x_max {
            break
        }
        if step == 0f32 {
            draw_pixel(x as i16, y as i16, *color, screen);
        } else {
            let mut tmp = 0f32;
            while tmp < step {
                tmp += 1f32;
                draw_pixel(x as i16, y as i16, *color, screen);
                y += y_count;
                if y > y_max || y < y_min {
                    break
                }
            }
        }
        x += x_count;
    }
}

fn draw_map(map: &[Vec<i16>], width_step: i16, height_step: i16, screen: &sdl::video::Surface) {
    //let mut x_orig = ((WINDOW_WIDTH - map[0].len() * width_step as usize - 10) / 2) as i16;
    let mut x_orig = ((WINDOW_HEIGHT - map.len() * height_step as usize - 10) / 2) as i16;
    let mut y_orig = (WINDOW_HEIGHT / 2 + 5) as i16;
    let mut prev_line = vec!();

    for line in map.iter() {
        let mut y = y_orig;
        let mut x = x_orig;
        let mut prev = None;
        let mut current_line = vec!();
        let color = sdl::video::Color::RGB(255, 255, 255);

        for (it, point) in line.iter().enumerate() {
            let current = Point::new(x as i16, y as i16 - point * height_step);
            if let Some(p) = prev {
                draw_line(&p, &current, screen, &color);
            }
            if prev_line.len() > 0 {
                draw_line(&prev_line[it], &current, screen, &color);
            }
            current_line.push(current);
            prev = Some(current);
            y += height_step;
            x += width_step;
        }
        prev_line = current_line.clone();
        x_orig += width_step;
        y_orig -= height_step;
    }
}

fn check_map(map: &[Vec<i16>]) -> Option<(i16, i16)> {
    let width = map[0].len();
    let height = map.len();

    if width != height {
        println!("Height must be equal to width!\nheight = {} and witdh = {}", height, width);
        return None;
    }
    for (pos, line) in map.iter().enumerate() {
        if line.len() != width {
            println!("Invalid width at line {}. Expected {}, found {}", pos, width, line.len());
            return None;
        }
    }
    //Some((((WINDOW_WIDTH - 10) / width as usize) as i16, ((WINDOW_HEIGHT - 10) / height as usize) as i16))
    Some((((WINDOW_HEIGHT - 10) / height as usize) as i16 / 2, ((WINDOW_HEIGHT - 10) / height as usize) as i16 / 2))
}

fn main() {
    let m = parse_map(match env::args().into_iter().collect::<Vec<String>>().get(1) {
        Some(f) => {
            let mut m = String::new();
            File::open(f).unwrap().read_to_string(&mut m).unwrap();
            m
        },
        None => get_default_map(),
    });

    match check_map(&m) {
        Some((w, h)) => {
            sdl::init(&[sdl::InitFlag::Video]);
            sdl::wm::set_caption("fdf", "fdf");

            let screen = match sdl::video::set_video_mode(WINDOW_WIDTH as isize, WINDOW_HEIGHT as isize, 32,
                                                          &[SurfaceFlag::HWSurface],
                                                          &[VideoFlag::DoubleBuf]) {
                Ok(s) => s,
                Err(err) => {
                    println!("failed to set video mode: {}", err);
                    return
                }
            };

            draw_map(&m, w, h, &screen);

            screen.flip();

            loop {
                match sdl::event::poll_event() {
                    Event::Quit => break,
                    Event::Key(k, _, _, _) if k == Key::Escape => break,
                    _ => {}
                }
            }

            sdl::quit();
        },
        None => {
            println!("Invalid map");
            return
        },
    }
}
