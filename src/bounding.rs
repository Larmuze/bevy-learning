use bevy::{math::bounding::*, prelude::*, transform::commands};

use crate::{bullet::Bullet, enemies::Enemy};

pub struct BoundingPlugin;

impl Plugin for BoundingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_volumes)
            .add_systems(PostUpdate, (
                //render_shapes,
                (
                    aabb_cast_system
                )
                //,render_volumes
            ).chain())
        ;
    }
}

#[derive(Component)]
pub enum Shape {
    Rectangle(Rectangle),
    Ellipse(Ellipse),
    Triangle(Triangle2d),
    Line(Segment2d),
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersects(bool);

#[derive(Component, Deref, DerefMut)]
pub struct Volume(Aabb2d);

fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = Color::GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(*r, translation, rotation, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(*t, translation, rotation, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(*l, translation, rotation, color);
            }
            Shape::Ellipse(e) => {
                gizmos.primitive_2d(*e, translation, rotation, color);
            }
        }
    }
}

fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &Shape, &Transform),
        Or<(Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;

        let aabb = match shape {
            Shape::Rectangle(r) => r.aabb_2d(translation, rotation),
            Shape::Triangle(t) => t.aabb_2d(translation, rotation),
            Shape::Line(l) => l.aabb_2d(translation, rotation),
            Shape::Ellipse(c) => c.aabb_2d(translation, rotation),
        };
        commands.entity(entity).insert(Volume(aabb));
    }
}

fn render_volumes(mut gizmos: Gizmos, query: Query<(&Volume, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects {
            Color::CYAN
        } else {
            Color::ORANGE_RED
        };
        gizmos.rect_2d(volume.center(), 0., volume.half_size() * 2., color);
    }
}

fn aabb_cast_system(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Volume), With<Bullet>>,
    enemies_query: Query<(Entity, &Volume), With<Enemy>>,
) {
    for (bullet_entity, bullet_volume) in &bullets_query {
        let mut has_despawned = false;
        for (enemy_entity, enemy_volume) in &enemies_query {
            if enemy_volume.intersects(&bullet_volume.0) {
                commands.entity(enemy_entity).despawn();
                has_despawned = true;
                break;
            }
        }
        if has_despawned {
            commands.entity(bullet_entity).despawn();
            break;
        }
    }
}
