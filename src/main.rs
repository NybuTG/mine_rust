use macroquad::prelude::*;

// Local imports
mod player;
use player::Player;


// Just window settings
fn window_conf() -> Conf {
    Conf {
        window_title: "Minerust".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    // "Create" a player
    // Sets up the necessary variables for a first person view
    let mut player = Player::default();

    loop {
        
        player.movement();

        // Quit game
        // TODO: Change into an enter menu type deal later
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        // Dont show the mouse
        show_mouse(false);

        // Dont lock cursor to the bounds of the windows
        set_cursor_grab(true);

        // Set background color
        // TODO: Change into image
        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));

        // TODO: Remove grid of course
        draw_grid(20, 1.);

        // TODO: (Probably). Might as well remove the get_camera method since it purely returns
        // some code and nothing more
        //
        // Reason to keep it:
        // Could be useful for a future implemenation of multiple views 
        // (3rd person from back and front namely)
        set_camera(&player.get_camera());

        // Load next frame on time
        next_frame().await
    }
}


