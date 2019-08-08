
extern crate nalgebra_glm as glm;


fn main() {

    let position = glm::vec2(0.0, 0.0);
    let velocity = glm::vec2(10.0, 0.0);
    let mut world = nphysics2d::world::World::<f32>::new();
    let gravity = glm::Vec2::y() * -9.8;
    world.set_gravity(gravity);
    world.integration_parameters_mut().dt = 1.0/60.0;

    let radius = 5.0;

    use ncollide2d::shape::{Ball, ShapeHandle};
    let shape = ShapeHandle::new(Ball::new(radius));

    let mut body_desc = {
        use nphysics2d::material::{BasicMaterial, MaterialHandle};
        use nphysics2d::object::{ColliderDesc, RigidBodyDesc};

        let collider_desc = ColliderDesc::new(shape.clone())
            .material(MaterialHandle::new(BasicMaterial::new(0.9, 0.0)))
            .collision_groups(
                ncollide2d::world::CollisionGroups::new()
                    .with_membership(&[0])
                    .with_blacklist(&[0]),
            )
            .name("bullet".to_string());

        RigidBodyDesc::new()
            .translation(position)
            .velocity(nphysics2d::math::Velocity::<f32>::new(velocity, 0.0))
            .mass(1000.0)
            .kinematic_rotation(false)
            .name("bullet".to_string())
    };

    for i in 0..100 {
        body_desc.build(&mut world);
    }

    for i in 0..100 {
        let t0 = std::time::Instant::now();
        world.step();
        let t1 = std::time::Instant::now();
        println!(
            "update physics took {}us",
            (t1 - t0).as_micros() as f64 / 1000.0
        );
    }

}
