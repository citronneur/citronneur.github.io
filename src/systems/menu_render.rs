use hecs::World;
use macroquad::prelude::*;

use crate::scene::{MenuItem, MenuSelected};

pub fn system_menu_render(world: &World) {
    let (w, h) = (screen_width(), screen_height());

    // Title
    let title = "GravityTouch";
    let title_size = 72_u16;
    let dim = measure_text(title, None, title_size, 1.0);
    draw_text(title, (w - dim.width) / 2.0, h * 0.30, title_size as f32, WHITE);

    // Subtitle
    let sub = "An ECS demo \u{2014} macroquad + hecs";
    let sub_dim = measure_text(sub, None, 22, 1.0);
    draw_text(sub, (w - sub_dim.width) / 2.0, h * 0.30 + 46.0, 22.0, GRAY);

    // Menu items sorted by index
    let mut items: Vec<(usize, String, bool)> = world
        .query::<(&MenuItem, Option<&MenuSelected>)>()
        .iter()
        .map(|(_, (item, sel))| (item.index, item.label.clone(), sel.is_some()))
        .collect();
    items.sort_by_key(|(idx, _, _)| *idx);

    let item_size: f32 = 34.0;
    let selected_size: f32 = 38.0;
    let spacing: f32 = 58.0;
    let menu_top = h * 0.52;

    for (i, (_, label, selected)) in items.iter().enumerate() {
        let size = if *selected { selected_size } else { item_size };
        let color = if *selected { YELLOW } else { Color::from_rgba(200, 200, 200, 255) };
        let y = menu_top + i as f32 * spacing;
        let dim = measure_text(label, None, size as u16, 1.0);
        let x = (w - dim.width) / 2.0;

        if *selected {
            draw_text(">", x - 28.0, y, size, YELLOW);
        }
        draw_text(label, x, y, size, color);
    }

    // Controls hint at bottom
    let hint = "\u{2191}\u{2193} Navigate    Enter / Space  Select";
    let hint_dim = measure_text(hint, None, 18, 1.0);
    draw_text(hint, (w - hint_dim.width) / 2.0, h - 24.0, 18.0, DARKGRAY);
}
