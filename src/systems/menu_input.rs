use hecs::World;
use macroquad::prelude::*;

use crate::scene::{MenuAction, MenuItem, MenuSelected, SceneKind, SceneManager};

pub fn system_menu_input(world: &mut World) {
    let key_down = is_key_pressed(KeyCode::Down);
    let key_up = is_key_pressed(KeyCode::Up);
    let key_confirm = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space);
    let just_pressed = is_mouse_button_pressed(MouseButton::Left);

    if !key_down && !key_up && !key_confirm && !just_pressed {
        return;
    }

    // Collect menu state into owned data so we can release the world borrow.
    let mut items: Vec<(hecs::Entity, usize, bool, MenuAction)> = world
        .query::<(&MenuItem, Option<&MenuSelected>)>()
        .iter()
        .map(|(e, (item, sel))| (e, item.index, sel.is_some(), item.action.clone()))
        .collect();
    items.sort_by_key(|&(_, idx, _, _)| idx);

    let count = items.len();
    let selected_pos = items.iter().position(|&(_, _, sel, _)| sel).unwrap_or(0);

    let new_pos = if key_down {
        (selected_pos + 1) % count
    } else if key_up {
        (selected_pos + count - 1) % count
    } else {
        selected_pos
    };

    if new_pos != selected_pos {
        let (old_entity, _, _, _) = items[selected_pos];
        let (new_entity, _, _, _) = items[new_pos];
        world.remove::<(MenuSelected,)>(old_entity).ok();
        world.insert(new_entity, (MenuSelected,)).ok();
    }

    if key_confirm || just_pressed{
        let action = &items[new_pos].3;
        let next = match action {
            MenuAction::StartLevel(n) => SceneKind::Level(*n),
            MenuAction::Quit => SceneKind::Quit,
        };
        let q = world.query_mut::<&mut SceneManager>();
        if let Some((_, mgr)) = q.into_iter().next() {
            mgr.next = Some(next);
        }
    }
}
