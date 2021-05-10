use macroquad::prelude::*;


pub struct Player {
    pub move_speed: f32,
    pub mouse_speed: f32,
    pub x: f64,
    pub switch: bool,
    pub bounds: f64,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub position: Vec3,
    pub last_mouse_position: Vec2,
}

impl Player {

    pub fn get_camera(&self) -> Camera3D { 
        Camera3D {
            position: self.position,
            up: self.up,
            target: self.position + self.front,
            ..Default::default()
        }
    }

    /*
    This code handles movement:
    It both checks for keycodes and updates the camera created using `get_camera()`
    */
    pub fn movement(&mut self) { 
        let delta = get_frame_time();
        

        // Forward
        if is_key_down(KeyCode::W) {
            self.position += self.front * self.move_speed;

        }

        // Backwards 
        if is_key_down(KeyCode::S) {
            self.position -= self.front * self.move_speed;
        }

        // Left
        if is_key_down(KeyCode::A) {
            self.position -= self.right * self.move_speed;
        }

        // Right
        if is_key_down(KeyCode::D) {
            self.position += self.right * self.move_speed;

        }


        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;
        self.last_mouse_position = mouse_position;

        self.yaw += mouse_delta.x * delta * self.mouse_speed;
        self.pitch += mouse_delta.y * delta * -self.mouse_speed;

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

        self.front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();

        self.x += if self.switch {0.04} else { -0.04 };
        if self.x >= self.bounds || self.x <= -self.bounds {
           self.switch = !self.switch;
        }

        
    }
}


/* 
This block of code implements `Default` 
values for the vars create in
the struct called Player.
*/

impl Default for Player {
    fn default() -> Self {
        Self {
            move_speed: 0.15,
            mouse_speed: 0.1,

            x: 0.0,
            switch: false,
            bounds: 8.0,
            world_up: vec3(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,

            front: vec3(0.0, 0.0, 0.0).normalize(),
            right: vec3(0.0, 0.0, 0.0).normalize(),
            up: vec3(0.0, 0.0, 0.0),
            position: vec3(0.0, 1.0, 0.0),
            last_mouse_position: mouse_position().into(),
        }
    }
}

