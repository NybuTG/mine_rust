use macroquad::prelude::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

pub struct player {
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
    pub grabbed: bool
}

impl Default for player {
    fn default() -> Self {
        Self {
            x: 0.0,
            switch: false,
            bounds: 8.0,
            world_up: vec3(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            front: Vec3::ZERO,
            right: Vec3::ZERO.cross(vec3(0.0, 1.0, 0.0)),
            up: Vec3::ZERO,
            position: vec3(0.0, 1.0, 0.0),
            last_mouse_position: mouse_position().into(),
            grabbed: true,
        }
    }
}

impl player {

    pub fn player_init(&mut self) {
        set_cursor_grab(self.grabbed);
        show_mouse(false);
    }

    pub fn player_update(&mut self) {
            let delta = get_frame_time();

            if is_key_pressed(KeyCode::Tab) {
                self.grabbed = !self.grabbed;
                set_cursor_grab(self.grabbed);
                show_mouse(!self.grabbed);
            }

            if is_key_down(KeyCode::W) {
                self.position += self.front * MOVE_SPEED;
            }
            if is_key_down(KeyCode::S) {
                self.position -= self.front * MOVE_SPEED;
            }
            if is_key_down(KeyCode::A) {
                self.position -= self.right * MOVE_SPEED;
            }
            if is_key_down(KeyCode::D) {
                self.position += self.right * MOVE_SPEED;
            }

            let mouse_position: Vec2 = mouse_position().into();
            let mouse_delta = mouse_position - self.last_mouse_position;
            self.last_mouse_position = mouse_position;

            self.yaw += mouse_delta.x * delta * LOOK_SPEED;
            self.pitch += mouse_delta.y * delta * -LOOK_SPEED;

            self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
            self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

            self.front = vec3(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.right = self.front.cross(self.world_up).normalize();
            self.up = self.right.cross(self.front).normalize();

            self.x += if self.switch { 0.04 } else { -0.04 };
            if self.x >= self.bounds || self.x <= -self.bounds {
                self.switch = !self.switch;
            }


            // Going 3d!

            set_camera(&Camera3D {
                position: self.position,
                up: self.up,
                target: self.position + self.front,
                ..Default::default()
            });
    }
}
