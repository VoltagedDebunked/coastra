use conrod_core::{widget, Colorable, Labelable, Sizeable, Widget};
use conrod_core::widget::Id;

pub struct Button {
    id: Id,
    label: String,
    color: conrod_core::Color,
    hover_color: conrod_core::Color,
    press_color: conrod_core::Color,
    label_color: conrod_core::Color,
    dimensions: [f64; 2],
}

impl Button {
    pub fn new(id: Id) -> Self {
        Button {
            id,
            label: String::new(),
            color: conrod_core::color::LIGHT_GREY,
            hover_color: conrod_core::color::LIGHT_BLUE,
            press_color: conrod_core::color::BLUE,
            label_color: conrod_core::color::BLACK,
            dimensions: [100.0, 50.0],
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn color(mut self, color: conrod_core::Color) -> Self {
        self.color = color;
        self
    }

    pub fn hover_color(mut self, color: conrod_core::Color) -> Self {
        self.hover_color = color;
        self
    }

    pub fn press_color(mut self, color: conrod_core::Color) -> Self {
        self.press_color = color;
        self
    }

    pub fn label_color(mut self, color: conrod_core::Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn dimensions(mut self, width: f64, height: f64) -> Self {
        self.dimensions = [width, height];
        self
    }

    pub fn set(&self, ui: &mut conrod_core::UiCell) -> Option<bool> {
        Some(widget::Button::new()
            .color(self.color)
            .hover_color(self.hover_color)
            .press_color(self.press_color)
            .label(&self.label)
            .label_color(self.label_color)
            .w_h(self.dimensions[0], self.dimensions[1])
            .set(self.id, ui)
            .was_clicked())
    }
}

pub struct Text {
    id: Id,
    text: String,
    color: conrod_core::Color,
    font_size: u32,
}

impl Text {
    pub fn new(id: Id) -> Self {
        Text {
            id,
            text: String::new(),
            color: conrod_core::color::BLACK,
            font_size: 16,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn color(mut self, color: conrod_core::Color) -> Self {
        self.color = color;
        self
    }

    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = size;
        self
    }

    pub fn set(&self, ui: &mut conrod_core::UiCell) {
        widget::Text::new(&self.text)
            .color(self.color)
            .font_size(self.font_size)
            .set(self.id, ui);
    }
}