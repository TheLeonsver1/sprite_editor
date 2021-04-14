use crate::common::{input::marker_components::MainCamera, sprite_batch_experiment::BatchedDraw};

use super::input::resources::MouseWorldPosition;
use super::{
    data_components::RectFromTransform, events::PressedOnEntity, marker_components::Pickable,
};
use bevy::{prelude::*, utils::Instant};
pub fn clicked_pickable_entity_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut pressed_on_ent_events: EventWriter<PressedOnEntity>,
    mouse_world_position: Res<MouseWorldPosition>,
    q_pickable: Query<(Entity, &Transform, &RectFromTransform), With<Pickable>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right)
    {
        for (entity, transform, rect) in q_pickable.iter() {
            if mouse_world_position.position.x < transform.translation.x + rect.width as f32 / 2.
                && mouse_world_position.position.y
                    < transform.translation.y + rect.height as f32 / 2.
                && mouse_world_position.position.x
                    > transform.translation.x - rect.width as f32 / 2.
                && mouse_world_position.position.y
                    > transform.translation.y - rect.height as f32 / 2.
            {
                if mouse_button_input.just_pressed(MouseButton::Right) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Right,
                    })
                } else if mouse_button_input.just_pressed(MouseButton::Left) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Left,
                    })
                }
            }
        }
    }
}
pub fn naive_frustum_culling(
    main_camera_query: Query<&Transform, With<MainCamera>>,
    mut map_features: Query<(&mut Visible, &Transform), With<Pickable>>,
) {
    //let now = Instant::now();
    //let mut count = 0;
    if let Ok(camera_transform) = main_camera_query.single() {
        let camera_transform_z_zero = Vec3::new(
            camera_transform.translation.x,
            camera_transform.translation.y,
            0.0,
        );
        for (mut visible, transform) in map_features.iter_mut() {
            if transform.translation.distance(camera_transform_z_zero) > 1500.0 {
                visible.is_visible = false;
            } else {
                //count+=1;
                if !visible.is_visible {
                    visible.is_visible = true;
                }
            }
        }
        //println!("{:?}",camera_transform.translation);
    }

    //println!("{}",count);
    //println!("{}",(Instant::now()-now).as_micros());
}
pub fn naive_frustum_culling_batched(
    main_camera_query: Query<&Transform, With<MainCamera>>,
    mut map_features: Query<(&mut BatchedDraw, &mut Visible, &Transform), With<Pickable>>,
) {
    //let now = Instant::now();
    if let Ok(camera_transform) = main_camera_query.single() {
        for (mut batched_visible, mut visible, transform) in map_features.iter_mut() {
            if transform.translation.distance(camera_transform.translation) > 1500.0 {
                batched_visible.is_visible = false;
                visible.is_visible = false;
            } else {
                if !batched_visible.is_visible {
                    batched_visible.is_visible = true;
                    visible.is_visible = true;
                }
            }
        }
        //println!("{:?}",camera_transform.translation);
    }
    //println!("{}",(Instant::now()-now).as_micros());
}
