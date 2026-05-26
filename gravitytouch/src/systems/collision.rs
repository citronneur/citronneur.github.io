use hecs::{Entity, World};
use macroquad::math::{Mat2, Vec2};

use crate::components::{CollideTag, Geometry, Position, Transformation};

pub fn system_collide(world: &mut World) {
    for (_, col) in world.query_mut::<&mut CollideTag>() {
        col.other = None;
    }

    let objects: Vec<(Entity, Position, Geometry, Option<Mat2>)> = world
        .query::<(&Position, &Geometry, &CollideTag, Option<&Transformation>)>()
        .iter()
        .map(|(e, (p, g, _, tf))| {
            (e, p.clone(), g.clone(), tf.map(|t| t.default * t.transformation))
        })
        .collect();

    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let (e1, p1, g1, tf1) = &objects[i];
            let (e2, p2, g2, tf2) = &objects[j];

            if !is_collide(p1, g1, *tf1, p2, g2, *tf2) {
                continue;
            }

            let mut o1 = world.query_one_mut::<&mut CollideTag>(*e1).unwrap();
            *o1 = CollideTag { other: Some(*e2) };
            let mut o2 = world.query_one_mut::<&mut CollideTag>(*e2).unwrap();
            *o2 = CollideTag { other: Some(*e1) };
        }
    }
}

pub fn is_collide(
    position_1: &Position, geometry_1: &Geometry, tf1: Option<Mat2>,
    position_2: &Position, geometry_2: &Geometry, tf2: Option<Mat2>,
) -> bool {
    match (geometry_1, geometry_2) {
        (Geometry::Circle(r1), Geometry::Circle(r2)) => {
            let dx = position_2.x - position_1.x;
            let dy = position_2.y - position_1.y;
            dx * dx + dy * dy < (r1 + r2) * (r1 + r2)
        }
        (Geometry::Circle(r), Geometry::Rectangle(w, h)) => {
            circle_rect_collide(position_1, *r, position_2, *w, *h, tf2)
        }
        (Geometry::Rectangle(w, h), Geometry::Circle(r)) => {
            circle_rect_collide(position_2, *r, position_1, *w, *h, tf1)
        }
        (Geometry::Rectangle(w1, h1), Geometry::Rectangle(w2, h2)) => {
            rect_rect_collide(position_1, *w1, *h1, tf1, position_2, *w2, *h2, tf2)
        }
        _ => panic!("Geometry collision pair not handled"),
    }
}

fn rect_rect_collide(
    p1: &Position, w1: f32, h1: f32, tf1: Option<Mat2>,
    p2: &Position, w2: f32, h2: f32, tf2: Option<Mat2>,
) -> bool {
    let mat1 = tf1.unwrap_or(Mat2::IDENTITY);
    let mat2 = tf2.unwrap_or(Mat2::IDENTITY);
    let center_diff = Vec2::new(p2.x - p1.x, p2.y - p1.y);
    let hw1 = w1 * 0.5;
    let hh1 = h1 * 0.5;
    let hw2 = w2 * 0.5;
    let hh2 = h2 * 0.5;

    // 4 SAT axes: local X and Y of each rectangle.
    let axes = [mat1.x_axis, mat1.y_axis, mat2.x_axis, mat2.y_axis];

    for axis in axes {
        let proj_center = center_diff.dot(axis).abs();
        let proj_r1 = hw1 * mat1.x_axis.dot(axis).abs() + hh1 * mat1.y_axis.dot(axis).abs();
        let proj_r2 = hw2 * mat2.x_axis.dot(axis).abs() + hh2 * mat2.y_axis.dot(axis).abs();
        if proj_center >= proj_r1 + proj_r2 {
            return false;
        }
    }
    true
}

fn circle_rect_collide(
    circle_pos: &Position, radius: f32,
    rect_pos: &Position, w: f32, h: f32,
    rect_tf: Option<Mat2>,
) -> bool {
    let delta = Vec2::new(circle_pos.x - rect_pos.x, circle_pos.y - rect_pos.y);
    // Move into rect's local space: inverse of an orthogonal matrix is its transpose.
    let local = match rect_tf {
        Some(mat) => mat.transpose() * delta,
        None => delta,
    };
    let closest = Vec2::new(local.x.clamp(-w * 0.5, w * 0.5), local.y.clamp(-h * 0.5, h * 0.5));
    let diff = local - closest;
    diff.length_squared() <= radius * radius
}
