use super::Color;
use super::ColorSystem;

pub struct Character {
    size: [f64; 2],
    position: [f64; 3],
    texture: Option<web_sys::HtmlImageElement>,
    texture_is_changed: bool,
    background_color: Color,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: [1.0, 0.0],
            position: [0.0, 0.0, 0.0],
            texture: None,
            texture_is_changed: false,
            background_color: Color::from(0),
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn stretch_height(&mut self) {
        if let Some(texture) = &self.texture {
            let width = texture.width() as f64;
            let height = texture.height() as f64;
            self.size[1] = self.size[0] * height / width;
        }
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn bind_to_grid(&mut self) {
        let p = self.position;
        let p = [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0];
        self.position = [p[0], p[1], self.position[2]];
    }

    pub fn texture_image(&mut self) -> Option<&web_sys::HtmlImageElement> {
        if self.texture_is_changed {
            if let Some(texture) = &self.texture {
                Some(texture)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_is_focused(&mut self, is_focused: bool) {
        if is_focused {
            self.background_color = ColorSystem::gray_900(127);
        } else {
            self.background_color = Color::from(0);
        }
    }

    pub fn set_image(&mut self, image: web_sys::HtmlImageElement) {
        self.texture = Some(image);
        self.texture_is_changed = true;
    }

    pub fn rendered(&mut self) {
        self.texture_is_changed = false;
        self.set_is_focused(false);
    }
}

impl Clone for Character {
    fn clone(&self) -> Self {
        let mut clone = Self::new();

        clone.set_size(self.size.clone());
        clone.set_position(self.position.clone());
        if let Some(texture) = &self.texture {
            clone.set_image(texture.clone());
        }

        clone
    }
}
