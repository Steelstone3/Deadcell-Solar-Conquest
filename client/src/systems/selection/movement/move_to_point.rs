use std::ops::Add;

use bevy::{
    math::{Quat, Vec3, Vec3Swizzles},
    prelude::{Query, Res, Transform},
    time::Time,
};

use crate::components::user_interface::controllable::Movement;

// TODO add to queries
pub fn move_to_point(
    time: Res<Time>,
    mut controllable_queries: Query<(&mut Movement, &mut Transform)>,
) {
    for mut controllable_query in controllable_queries.iter_mut() {
        let distance_to_target =
            (controllable_query.1.translation - controllable_query.0.target_location).length();

        if distance_to_target <= 5.0 {
            continue;
        }

        let current_transform = *controllable_query.1;
        let mut target_location = controllable_query.0.target_location;
        //ensure moveable stays at current z index
        target_location.z = current_transform.translation.z;

        let to_target = (target_location.xy() - current_transform.translation.xy()).normalize();
        let rotate_to_target = Quat::from_rotation_arc(Vec3::Y, to_target.extend(0.0));
        controllable_query.1.rotation = controllable_query.1.rotation.lerp(rotate_to_target, 0.05);

        if distance_to_target <= 100.0 {
            controllable_query.0.current_speed =
                (current_transform.translation - target_location).length();
        } else if controllable_query.0.current_speed < controllable_query.0.maximum_speed {
            controllable_query.0.current_speed = controllable_query
                .0
                .current_speed
                .add(5.0)
                .clamp(0.0, controllable_query.0.maximum_speed);
        }

        controllable_query.1.translation +=
            (controllable_query.0.current_speed * current_transform.up()) * time.delta_secs();
    }
}
