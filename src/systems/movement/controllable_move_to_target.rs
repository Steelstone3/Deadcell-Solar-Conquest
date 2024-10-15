use bevy::{
    math::{Quat, Vec3, Vec3Swizzles},
    prelude::{Query, Res, Transform},
    time::Time,
};

use crate::components::controllable::Movement;

pub fn controllable_move_to_target(
    time: Res<Time>,
    mut controllable_queries: Query<(&Movement, &mut Transform)>,
) {
    for mut controllable_query in controllable_queries.iter_mut() {
        if (controllable_query.1.translation - controllable_query.0.target_location).length() < 5.0
        {
            continue;
        }

        let current_location = controllable_query.1.translation;
        let mut target_location = controllable_query.0.target_location;
        //ensure moveable stays at current z index
        target_location.z = current_location.z;

        controllable_query.1.translation +=
            (controllable_query.0.target_location - current_location) * time.delta_seconds();

        let to_target = (target_location.xy() - current_location.xy()).normalize();
        let rotate_to_target = Quat::from_rotation_arc(Vec3::Y, to_target.extend(0.0));
        controllable_query.1.rotation = rotate_to_target;
    }
}
