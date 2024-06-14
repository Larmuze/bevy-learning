use bevy::{math::bounding::*, prelude::*, color::palettes::basic::*};

pub struct BoundingPlugin;

impl Plugin for BoundingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_volumes)
            // .add_systems(PostUpdate, (
            //     //render_shapes,
            //     //,render_volumes
            // ).chain())
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
pub struct Volume(pub Aabb2d);

fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = Color::from(GRAY);
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(r, translation, rotation, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(t, translation, rotation, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(l, translation, rotation, color);
            }
            Shape::Ellipse(e) => {
                gizmos.primitive_2d(e, translation, rotation, color);
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
            Color::from(AQUA)
        } else {
            Color::from(TEAL)
        };
        gizmos.rect_2d(volume.center(), 0., volume.half_size() * 2., color);
    }
}
