use bevy::{
    ecs::query::QueryData,
    render::camera::{Camera, Projection},
    transform::components::Transform,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableCameraOrthographicProjectionQuery {
    pub projection: &'static mut Projection,
    pub camera: &'static Camera,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableCameraTransformQuery {
    pub transform: &'static mut Transform,
    pub camera: &'static Camera,
}

#[derive(QueryData)]
pub struct CameraTransformOrthographicProjectionQuery {
    pub transform: &'static Transform,
    pub projection: &'static Projection,
    pub camera: &'static Camera,
}
