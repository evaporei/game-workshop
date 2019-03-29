use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use std::error::Error;

struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const PLAYER_MOVEMENT_SPEED: i32 = 10;

fn update_player(player: &mut Player) {
    use Direction::*;
    match player.direction {
        Up => {
            player.position = player.position.offset(0, -player.speed);
        },
        Down => {
            player.position = player.position.offset(0, player.speed);
        },
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Right => {
            player.position = player.position.offset(player.speed, 0);
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", screen_width, screen_height)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 24, 40),
        speed: 0,
        direction: Direction::Right,
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = 0;
                },
                _ => {}
            }
        }

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        update_player(&mut player);

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/bardo.png")?;

        let screen_position = player.position + Point::new(screen_width as i32 / 2, screen_height as i32 / 2);

        let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

        canvas.copy(&texture, Some(player.sprite), Some(screen_rect))?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
