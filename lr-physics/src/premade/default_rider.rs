use vector2d::Vector2Df;

use crate::{engine::EngineVersion, entity::registry::EntityRegistry};

pub fn build_default_rider(registry: &mut EntityRegistry, version: EngineVersion) {
    let repel_length_factor = 0.5;
    let scarf_friction = match version {
        EngineVersion::Flash => 0.1,
        EngineVersion::Com => 0.2,
        EngineVersion::LRA => 0.1,
    };
    let mount_endurance = 0.057;
    let remount_endurance_factor = 2;
    let remount_strength_factor = match version {
        EngineVersion::Flash => 0.0,
        EngineVersion::Com => 0.1,
        EngineVersion::LRA => 0.5,
    };

    let sled_skeleton = registry.skeleton_template_builder();

    let (sled_skeleton, peg) = sled_skeleton
        .point(Vector2Df::new(0.0, 0.0))
        .contact()
        .contact_friction(0.8)
        .build();
    let (sled_skeleton, tail) = sled_skeleton
        .point(Vector2Df::new(0.0, 5.0))
        .contact()
        .build();
    let (sled_skeleton, nose) = sled_skeleton
        .point(Vector2Df::new(15.0, 5.0))
        .contact()
        .build();
    let (sled_skeleton, string) = sled_skeleton
        .point(Vector2Df::new(17.5, 0.0))
        .contact()
        .build();

    let (sled_skeleton, sled_back) = sled_skeleton.bone(peg, tail).build();
    let (sled_skeleton, _) = sled_skeleton.bone(tail, nose).build();
    let (sled_skeleton, _) = sled_skeleton.bone(nose, string).build();
    let (sled_skeleton, sled_front) = sled_skeleton.bone(string, peg).build();
    let (sled_skeleton, _) = sled_skeleton.bone(peg, nose).build();
    let (sled_skeleton, _) = sled_skeleton.bone(string, tail).build();

    let (sled_skeleton, _) = sled_skeleton.joint(sled_back, sled_front).build();

    sled_skeleton.build();

    let rider_skeleton = registry.skeleton_template_builder();
    let (rider_skeleton, butt) = rider_skeleton
        .point(Vector2Df::new(5.0, 0.0))
        .contact()
        .contact_friction(0.8)
        .build();
    let (rider_skeleton, shoulder) = rider_skeleton
        .point(Vector2Df::new(5.0, -5.5))
        .contact()
        .contact_friction(0.8)
        .build();
    let (rider_skeleton, right_hand) = rider_skeleton
        .point(Vector2Df::new(11.5, -5.0))
        .contact()
        .contact_friction(0.1)
        .build();
    let (rider_skeleton, left_hand) = rider_skeleton
        .point(Vector2Df::new(11.5, -5.0))
        .contact()
        .contact_friction(0.1)
        .build();
    let (rider_skeleton, left_foot) = rider_skeleton
        .point(Vector2Df::new(10.0, 5.0))
        .contact()
        .build();
    let (rider_skeleton, right_foot) = rider_skeleton
        .point(Vector2Df::new(10.0, 5.0))
        .contact()
        .build();
    let (rider_skeleton, scarf0) = rider_skeleton
        .point(Vector2Df::new(3.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf1) = rider_skeleton
        .point(Vector2Df::new(1.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf2) = rider_skeleton
        .point(Vector2Df::new(-1.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf3) = rider_skeleton
        .point(Vector2Df::new(-3.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf4) = rider_skeleton
        .point(Vector2Df::new(-5.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf5) = rider_skeleton
        .point(Vector2Df::new(-7.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (rider_skeleton, scarf6) = rider_skeleton
        .point(Vector2Df::new(-9.0, -5.5))
        .air_friction(scarf_friction)
        .build();

    let (rider_skeleton, torso) = rider_skeleton.bone(shoulder, butt).build();
    let (rider_skeleton, _) = rider_skeleton.bone(shoulder, left_hand).build();
    let (rider_skeleton, _) = rider_skeleton.bone(shoulder, right_hand).build();
    let (rider_skeleton, _) = rider_skeleton.bone(butt, left_foot).build();
    let (rider_skeleton, _) = rider_skeleton.bone(butt, right_foot).build();
    let (rider_skeleton, _) = rider_skeleton.bone(shoulder, right_hand).build();
    let (rider_skeleton, _) = rider_skeleton
        .bone(shoulder, left_foot)
        .repel()
        .initial_length_factor(repel_length_factor)
        .build();
    let (rider_skeleton, _) = rider_skeleton
        .bone(shoulder, right_foot)
        .repel()
        .initial_length_factor(repel_length_factor)
        .build();
    let (rider_skeleton, _) = rider_skeleton.bone(shoulder, scarf0).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf0, scarf1).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf1, scarf2).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf2, scarf3).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf3, scarf4).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf4, scarf5).bias(1.0).build();
    let (rider_skeleton, _) = rider_skeleton.bone(scarf5, scarf6).bias(1.0).build();

    rider_skeleton.build();
}
