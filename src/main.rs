use macroquad::prelude::*;

// Local imports
mod player;
use player::player_main;

fn window_conf() -> Conf {
    Conf {
        window_title: "Minerust".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut player = player_main::Player{
        ..Default::default()
    };

    loop {
        
        player.movement();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Tab) {
            player.grabbed = !player.grabbed;
            set_cursor_grab(player.grabbed);
            show_mouse(!player.grabbed);
        }


        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
        draw_grid(20, 1.);

        // Load next frame on time
        next_frame().await
    }
}


