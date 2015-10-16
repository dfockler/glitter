use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    Event,
    Key
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget
};

pub struct VerticalLayout {
    children: Vec<Box<Widget>>,
    spacing: usize
}

impl VerticalLayout {
    pub fn new() -> VerticalLayout {
        VerticalLayout {
            children: Vec::new(),
            spacing: 0
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget))
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing
    }
}

impl Drawable for VerticalLayout {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize) {
        let mut y_offset = 0;
        for child in self.children.iter() {
            let remaining_height = if y_offset < height {
                height - y_offset
            } else {
                // no remaining vertical space left,
                // so no need to draw remaining children
                break
            };
            child.draw_at(rb, x_pos, y_pos + y_offset, width, 0);
            y_offset += child.height() + self.spacing;
        }
    }

    fn width(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.width())
            .max()
            .unwrap_or(0)
    }

    fn height(&self) -> usize {
        let children: usize = self.children.iter().map(|child| child.height()).sum();
        let spacing = self.spacing * self.children.len();
        children + spacing
    }
}

impl EventReceiver for VerticalLayout {
    fn handle_event(&mut self, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(event);
        }

        true
    }
}