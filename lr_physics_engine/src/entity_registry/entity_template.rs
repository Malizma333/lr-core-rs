use geometry::Point;
use std::collections::BTreeMap;

use crate::entity_registry::{
    EntityBone, EntityBoneBuilder, EntityJoint, EntityJointBuilder, EntityPoint,
    EntityPointBuilder, RemountVersion,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityPointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityBoneId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityJointId(usize);

pub struct EntityTemplate {
    points: BTreeMap<EntityPointId, EntityPoint>,
    bones: BTreeMap<EntityBoneId, EntityBone>,
    joints: BTreeMap<EntityJointId, EntityJoint>,
    dismounted_timer: u32,
    remounting_timer: u32,
    mounted_timer: u32,
    remount_version: RemountVersion,
}

impl EntityTemplate {
    pub(crate) fn points(&self) -> &BTreeMap<EntityPointId, EntityPoint> {
        &self.points
    }

    pub(crate) fn bones(&self) -> &BTreeMap<EntityBoneId, EntityBone> {
        &self.bones
    }

    pub(crate) fn joints(&self) -> &BTreeMap<EntityJointId, EntityJoint> {
        &self.joints
    }

    pub(crate) fn dismounted_timer(&self) -> u32 {
        self.dismounted_timer
    }

    pub(crate) fn remounting_timer(&self) -> u32 {
        self.remounting_timer
    }

    pub(crate) fn mounted_timer(&self) -> u32 {
        self.mounted_timer
    }

    pub(crate) fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }

    // TODO use computed graph for this
    pub(crate) fn sled_points(&self) -> Vec<&EntityPointId> {
        let x: Vec<&EntityPointId> = self.points.keys().collect();
        x.get(0..=3)
            .expect("Default riders should always have 4 points")
            .to_vec()
    }
}

pub struct EntityTemplateBuilder {
    points: BTreeMap<EntityPointId, EntityPointBuilder>,
    bones: BTreeMap<EntityBoneId, EntityBoneBuilder>,
    joints: BTreeMap<EntityJointId, EntityJointBuilder>,
    dismounted_timer: u32,
    remounting_timer: u32,
    mounted_timer: u32,
    remount_version: RemountVersion,
}

impl Default for EntityTemplateBuilder {
    fn default() -> Self {
        EntityTemplateBuilder::new()
    }
}

impl EntityTemplateBuilder {
    pub fn new() -> EntityTemplateBuilder {
        EntityTemplateBuilder {
            points: BTreeMap::new(),
            bones: BTreeMap::new(),
            joints: BTreeMap::new(),
            dismounted_timer: 0,
            remounting_timer: 0,
            mounted_timer: 0,
            remount_version: RemountVersion::None,
        }
    }

    pub(crate) fn add_point(&mut self, point: EntityPointBuilder) -> EntityPointId {
        let id = EntityPointId(self.points.len());
        self.points.insert(id, point);
        id
    }

    pub(crate) fn add_bone(&mut self, bone: EntityBoneBuilder) -> EntityBoneId {
        let id = EntityBoneId(self.bones.len());
        self.bones.insert(id, bone);
        id
    }

    pub(crate) fn add_joint(&mut self, joint: EntityJointBuilder) -> EntityJointId {
        let id = EntityJointId(self.joints.len());
        self.joints.insert(id, joint);
        id
    }

    pub fn dismounted_timer(mut self, duration: u32) -> Self {
        self.dismounted_timer = duration;
        self
    }

    pub fn remounting_timer(mut self, duration: u32) -> Self {
        self.remounting_timer = duration;
        self
    }

    pub fn mounted_timer(mut self, duration: u32) -> Self {
        self.mounted_timer = duration;
        self
    }

    pub fn remount_version(mut self, remount_version: RemountVersion) -> Self {
        self.remount_version = remount_version;
        self
    }

    pub fn build(self) -> EntityTemplate {
        let points = self
            .points
            .into_iter()
            .map(|x| (x.0, x.1.build()))
            .collect();
        let bones = self
            .bones
            .into_iter()
            .map(|x| (x.0, x.1.build(&points)))
            .collect();
        let joints = self
            .joints
            .into_iter()
            .map(|x| (x.0, x.1.build()))
            .collect();

        EntityTemplate {
            points,
            bones,
            joints,
            dismounted_timer: self.dismounted_timer,
            remounting_timer: self.remounting_timer,
            mounted_timer: self.mounted_timer,
            remount_version: self.remount_version,
        }
    }

    // Known bug: Default riders of different remount versions are not able to
    // cross-remount with each other because they come from different templates,
    // even though they normally would in linerider.com. This is such a niche case
    // that it's probably not worth fixing.
    // TODO Maybe we could solve this with computing graph isomorphism?
    /// Builds the original bosh skeleton
    pub fn default_rider(version: RemountVersion) -> EntityTemplate {
        let repel_length_factor = 0.5;
        let scarf_friction = 0.1;
        let mount_endurance = 0.057;
        let remount_endurance_factor = 2.0;
        let remount_strength_factor = match version {
            RemountVersion::None => 0.0,
            RemountVersion::ComV1 | RemountVersion::ComV2 => 0.1,
            RemountVersion::LRA => 0.5,
        };
        // Adjustment strength when remounting affects all bones in LRA
        let unbreakable_remount_strength_factor = match version {
            RemountVersion::LRA => 0.5,
            _ => 1.0,
        };

        // Remount version also affects physics processing order, which is why it's needed internally
        let mut skeleton = Self::new().remount_version(version);

        skeleton = match version {
            RemountVersion::None => skeleton,
            _ => skeleton
                .dismounted_timer(30)
                .remounting_timer(3)
                .mounted_timer(3),
        };

        let peg = skeleton.add_point(
            EntityPointBuilder::new(Point::new(0.0, 0.0))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let tail =
            skeleton.add_point(EntityPointBuilder::new(Point::new(0.0, 5.0)).is_contact(true));
        let nose =
            skeleton.add_point(EntityPointBuilder::new(Point::new(15.0, 5.0)).is_contact(true));
        let string =
            skeleton.add_point(EntityPointBuilder::new(Point::new(17.5, 0.0)).is_contact(true));
        let butt = skeleton.add_point(
            EntityPointBuilder::new(Point::new(5.0, 0.0))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let shoulder = skeleton.add_point(
            EntityPointBuilder::new(Point::new(5.0, -5.5))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let right_hand = skeleton.add_point(
            EntityPointBuilder::new(Point::new(11.5, -5.0))
                .is_contact(true)
                .contact_friction(0.1),
        );
        let left_hand = skeleton.add_point(
            EntityPointBuilder::new(Point::new(11.5, -5.0))
                .is_contact(true)
                .contact_friction(0.1),
        );
        let left_foot =
            skeleton.add_point(EntityPointBuilder::new(Point::new(10.0, 5.0)).is_contact(true));
        let right_foot =
            skeleton.add_point(EntityPointBuilder::new(Point::new(10.0, 5.0)).is_contact(true));
        let scarf0 = skeleton
            .add_point(EntityPointBuilder::new(Point::new(3.0, -5.5)).air_friction(scarf_friction));
        let scarf1 = skeleton
            .add_point(EntityPointBuilder::new(Point::new(1.0, -5.5)).air_friction(scarf_friction));
        let scarf2 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-1.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf3 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-3.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf4 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-5.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf5 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-7.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf6 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-9.0, -5.5)).air_friction(scarf_friction),
        );

        let sled_back = skeleton.add_bone(
            EntityBoneBuilder::new((peg, tail))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((tail, nose))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((nose, string))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        let sled_front = skeleton.add_bone(
            EntityBoneBuilder::new((string, peg))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((peg, nose))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((string, tail))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((peg, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((tail, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((nose, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        let torso = skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, butt))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, left_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((butt, left_foot))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((butt, right_foot))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, peg))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((left_hand, string))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((right_hand, string))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((left_foot, nose))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((right_foot, nose))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, left_foot))
                .repel(true)
                .initial_length_factor(repel_length_factor)
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_foot))
                .repel(true)
                .initial_length_factor(repel_length_factor)
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(EntityBoneBuilder::new((shoulder, scarf0)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf0, scarf1)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf1, scarf2)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf2, scarf3)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf3, scarf4)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf4, scarf5)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf5, scarf6)).bias(1.0));

        skeleton.add_joint(EntityJointBuilder::new(sled_back, sled_front));
        skeleton.add_joint(EntityJointBuilder::new(torso, sled_front).is_mount(true));

        skeleton.build()
    }
}

impl From<EntityTemplate> for EntityTemplateBuilder {
    fn from(skeleton: EntityTemplate) -> Self {
        Self {
            points: skeleton
                .points
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            bones: skeleton
                .bones
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            joints: skeleton
                .joints
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            dismounted_timer: skeleton.dismounted_timer,
            remounting_timer: skeleton.remounting_timer,
            mounted_timer: skeleton.mounted_timer,
            remount_version: skeleton.remount_version,
        }
    }
}
