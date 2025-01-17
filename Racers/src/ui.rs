use macroquad::prelude::*;

pub struct Button {
    pub pressed: bool,
    hitbox: Rect,
    colour: Color,
    text: String,
}

pub struct Slider {
    pub value: i32,
    min: i32,
    max: i32,
    default: i32,

    pos: Vec2,

    pub selected: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: String, colour: Color) -> Self {
        Self {
            hitbox: Rect::new(x, y, width, height),
            pressed: false,
            text,
            colour,
        }
    }

    pub fn draw(&self) {
        let r = self.hitbox;
        let c = self.hitbox.center();
        draw_rectangle(r.x, r.y, r.w, r.h, self.colour);
        draw_text(&self.text, c.x, c.y, 40.0, BLACK);
    }

    pub fn check_pressed(&mut self) -> bool {
        if (is_mouse_button_pressed(MouseButton::Left)) {
            let mp = mouse_position();
            if self.hitbox.contains(mp.into()) {
                self.pressed = true;
                return true;
            }
        }
        return false;
    }

    pub fn reset(&mut self) {
        self.pressed = false;
    }
}

impl Slider {
    pub fn new(x: f32, y: f32, min: i32, max: i32, default: i32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            value: default,
            min,
            max,
            selected: false,
            default,
        }
    }

    pub fn draw(&self) {
        let width = 400.0; // sets length of slider to 400px
        let radius = 14.0; // sets radius of node to 5px
        let pos = self.pos;

        draw_line(pos.x, pos.y, pos.x + width, pos.y, 6.0, BLACK);
        let node_pos = Vec2::new(
            pos.x
                + width as f32 * (self.value as f32 - self.min as f32)
                    / (self.max as f32 - self.min as f32),
            pos.y,
        );

        draw_circle(node_pos.x, node_pos.y, radius, GRAY);
        draw_text(&format!("{}", self.min), pos.x - 25.0, pos.y, 20.0, BLACK);
        draw_text(
            &format!("{}", self.max),
            pos.x + width as f32 + 15.0,
            pos.y,
            20.0,
            BLACK,
        );
        draw_text(
            &format!("{}", self.value),
            node_pos.x,
            node_pos.y - 15.0,
            20.0,
            BLACK,
        );
    }

    pub fn update(&mut self) {
        let pos = self.pos;
        let radius = 14.0;
        let width = 400.0;
        let node_pos = Vec2::new(
            pos.x
                + width as f32 * (self.value as f32 - self.min as f32)
                    / (self.max as f32 - self.min as f32),
            pos.y,
        );
        let mp = Vec2::from(mouse_position());

        if !self.selected && is_mouse_button_down(MouseButton::Left) {
            if (mp - node_pos).length() <= radius {
                self.selected = true;
            }
        }

        if (self.selected && is_mouse_button_released(MouseButton::Left)) {
            self.selected = false;
        }

        if (self.selected) {
            // make node follow the mouse's x position
            // how many pixels is one value point worth
            let pixels_per_unit: f32 = width as f32 / (self.max as f32 - self.min as f32);

            let dx = mp.x - node_pos.x;
            let delta_value = dx / pixels_per_unit;
            self.value = clamp(self.value + delta_value as i32, self.min, self.max);
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default;
        self.selected = false;
    }
}
