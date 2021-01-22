use std::f64::consts::PI;
use std::io::Read;

use termion::async_stdin;

use field::Field;
use player::Player;

use crate::screen::Screen;

mod field;
mod player;
mod screen;

const WIDTH: usize = 144;
const HEIGHT: usize = 38;
const FOV: f64 = PI / 4.0;
const DEPTH_OF_VIEW: f64 = 16.0;

fn main() {
    let field = Field::new(vec![
        "#####################",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#........#..........#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "#...................#",
        "##########..........#",
        "#...................#",
        "#...................#",
        "#####################",
    ]);

    let mut player = Player {
        x: 11.0,
        y: 5.0,
        angle: 0.0,
    };
    let mut screen = Screen::new();
    let mut stdin = async_stdin().bytes();
    draw(&mut screen, &player, &field);

    loop {
        match stdin.next() {
            Some(Ok(b'a')) => player.turn_left(),
            Some(Ok(b'd')) => player.turn_right(),
            Some(Ok(b'w')) => {
                player.move_forward();
                if field.get(player.x as usize, player.y as usize) == Some(&'#') {
                    player.move_backwards();
                }
            }
            Some(Ok(b's')) => {
                player.move_backwards();
                if field.get(player.x as usize, player.y as usize) == Some(&'#') {
                    player.move_forward();
                }
            }
            Some(Ok(b'q')) => break,
            _ => continue,
        }

        draw(&mut screen, &player, &field);
    }

    screen.move_to(WIDTH, HEIGHT);
    screen.reset();
}

fn draw(screen: &mut Screen, camera: &Player, field: &Field) {
    for x in 0..WIDTH {
        let angle = calculate_angle(x, camera.angle);
        let dist = calculate_wall_distance(field, camera, angle);
        let ceil = calculate_ceiling_offset(dist);

        for y in 0..HEIGHT {
            screen.move_to(x + 1, y + 1);

            if y < ceil {
                screen.draw_square(0, 0, 128);
            } else if y < HEIGHT - ceil {
                screen.draw_square((161.0 - 10.0 * dist) as u8, 0, 0);
            } else {
                let floor_dist = y as f64 - HEIGHT as f64 / 2.0;
                screen.draw_square(0, (13.0 * floor_dist) as u8, 0);
            }
        }
    }
}

fn calculate_angle(x: usize, camera_angle: f64) -> f64 {
    (camera_angle - FOV / 2.0) + (x as f64) / (WIDTH as f64) * FOV
}

fn calculate_wall_distance(field: &Field, player: &Player, angle: f64) -> f64 {
    let mut dist = 0.0;
    let (eye_x, eye_y) = angle.sin_cos();

    while dist < DEPTH_OF_VIEW {
        dist += 0.1;
        let test_x = player.x + eye_x * dist;
        let test_y = player.y + eye_y * dist;
        if test_x < 0.0 || test_y < 0.0 {
            return DEPTH_OF_VIEW;
        }
        match field.get(test_x as usize, test_y as usize) {
            None => return DEPTH_OF_VIEW,
            Some('#') => return dist,
            Some(_) => {}
        }
    }
    DEPTH_OF_VIEW
}

fn calculate_ceiling_offset(dist: f64) -> usize {
    if dist < 2.0 {
        0
    } else {
        HEIGHT / 2 - HEIGHT / dist as usize
    }
}
