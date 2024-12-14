use regex::Regex;
use show_image::{create_window, event, exit, ImageInfo, ImageView};
use std::io;

const W: i64 = 101;
const H: i64 = 103;

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn simulate_steps(r: &Robot, steps: i64) -> Robot {
    Robot {
        p: (
            (r.p.0 + r.v.0 * steps % W + W) % W,
            (r.p.1 + r.v.1 * steps % H + H) % H,
        ),
        v: r.v,
    }
}

#[show_image::main]
fn main() {
    let mut robots: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>\-?\d+),(?<vy>\-?\d+)").unwrap();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let c = re.captures(line.as_str()).unwrap();
        robots.push(Robot {
            p: (c["px"].parse().unwrap(), c["py"].parse().unwrap()),
            v: (c["vx"].parse().unwrap(), c["vy"].parse().unwrap()),
        });
    }

    let window = create_window("image", Default::default()).unwrap();

    let padding = 10;
    let grid = 7;
    let mut step = 0;
    loop {
        let mut pixel_data: Vec<u8> = Vec::new();
        pixel_data.resize(((W + padding) * grid * (H + padding) * grid) as usize, 0);
        for iy in 0..grid {
            for ix in 0..grid {
                for r in robots.iter() {
                    let new_r = simulate_steps(&r, step + (iy * grid + ix));
                    pixel_data[((W + padding) * grid * (new_r.p.1 + ((H + padding) * iy))
                        + (new_r.p.0 + ix * (W + padding)))
                        as usize] = 255;
                }
            }
        }

        let image = ImageView::new(
            ImageInfo::mono8(((W + padding) * grid) as u32, ((H + padding) * grid) as u32),
            pixel_data.as_slice(),
        );

        println!("step: {step}");
        window.set_image("image", image).unwrap();

        for event in window.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                if !event.input.state.is_pressed() {
                    continue;
                }
                match event.input.key_code {
                    Some(event::VirtualKeyCode::Escape) => exit(1),
                    Some(event::VirtualKeyCode::Right) => {
                        step += grid * grid;
                        break;
                    }
                    Some(event::VirtualKeyCode::Left) => {
                        if step > 0 {
                            step -= grid * grid;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
