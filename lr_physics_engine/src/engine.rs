use lr_format_core::Track;
use lr_physics_grid::GridVersion;
use vector2d::Vector2Df;

use crate::{
    PhysicsMoment,
    entity_registry::{
        EntityId, EntityRegistry, EntityState, EntityTemplate, EntityTemplateBuilder,
        EntityTemplateId, Error, RemountVersion,
    },
    line_registry::{LineId, LineRegistry, PhysicsLine, PhysicsLineBuilder},
};

pub struct PhysicsEngine {
    line_registry: LineRegistry,
    entity_registry: EntityRegistry,
}

// Engine holds the public API for the entity and line registries
// Entity registry does most of the work
impl PhysicsEngine {
    pub fn new(grid_version: GridVersion) -> Self {
        PhysicsEngine {
            line_registry: LineRegistry::new(grid_version),
            entity_registry: EntityRegistry::new(),
        }
    }

    /// Provides a view of entities during a specific frame by simulating up to that frame
    pub fn view_frame(&mut self, frame: u32) -> Vec<EntityState> {
        self.view_moment(frame, PhysicsMoment::None)
    }

    /// Provides a view of entities during a specific moment by simulating up to that frame and moment
    pub fn view_moment(&mut self, frame: u32, moment: PhysicsMoment) -> Vec<EntityState> {
        self.entity_registry
            .compute_frame(frame, moment, &self.line_registry)
    }

    /// Changes the engine's grid version and reregisters all physics lines
    pub fn set_grid_version(&mut self, grid_version: GridVersion) {
        self.line_registry.set_grid_version(grid_version);
        self.entity_registry.clear_cache();
    }

    pub fn add_line(&mut self, line: PhysicsLine) -> LineId {
        let id = self.line_registry.add_line(line);
        self.entity_registry.clear_cache();
        id
    }

    pub fn get_line(&self, id: LineId) -> Option<&PhysicsLine> {
        self.line_registry.get_line(id)
    }

    pub fn replace_line(&mut self, id: LineId, new_line: PhysicsLine) {
        self.line_registry.replace_line(id, new_line);
        self.entity_registry.clear_cache();
    }

    pub fn remove_line(&mut self, id: LineId) {
        self.line_registry.remove_line(id);
        self.entity_registry.clear_cache();
    }

    pub fn register_entity_template(
        &mut self,
        entity_template: EntityTemplate,
    ) -> EntityTemplateId {
        self.entity_registry.add_entity_template(entity_template)
    }

    /// Adds a new entity given a template id
    ///
    /// Returns the id of the newly created entity if the template id is valid
    pub fn add_entity(&mut self, entity_template_id: EntityTemplateId) -> Option<EntityId> {
        self.entity_registry.create_entity(entity_template_id)
    }

    /// Gets an entity's initial offset from (0,0)
    ///
    /// Returns the offset if the entity id is valid
    pub fn get_entity_initial_offset(&self, entity_id: EntityId) -> Option<Vector2Df> {
        self.entity_registry.get_entity_initial_offset(entity_id)
    }

    /// Sets an entity's initial offset from (0,0)
    ///
    /// Errors if the entity id is invalid
    pub fn set_entity_initial_offset(
        &mut self,
        entity_id: EntityId,
        offset: Vector2Df,
    ) -> Result<(), Error> {
        self.entity_registry
            .set_entity_initial_offset(entity_id, offset)
    }

    /// Gets an entity's initial overall velocity
    ///
    /// Returns the velocity if the entity id is valid
    pub fn get_entity_initial_velocity(&self, entity_id: EntityId) -> Option<Vector2Df> {
        self.entity_registry.get_entity_initial_velocity(entity_id)
    }

    /// Sets an entity's initial velocity
    ///
    /// Errors if the entity id is invalid
    pub fn set_entity_initial_velocity(
        &mut self,
        entity_id: EntityId,
        velocity: Vector2Df,
    ) -> Result<(), Error> {
        self.entity_registry
            .set_entity_initial_velocity(entity_id, velocity)
    }

    /// Removes an entity instance from the registry
    ///
    /// Errors if the entity id is invalid
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Result<(), Error> {
        self.entity_registry.remove_entity(entity_id)
    }

    /// Completely clears the state cache of all entities in the registry
    pub fn clear_cache(&mut self) {
        self.entity_registry.clear_cache();
    }

    // TODO make these only visible by tests

    pub fn from_track(track: &Track, lra: bool) -> Self {
        let grid_version = match track.grid_version() {
            lr_format_core::GridVersion::V6_0 => GridVersion::V6_0,
            lr_format_core::GridVersion::V6_1 => GridVersion::V6_1,
            lr_format_core::GridVersion::V6_2 => GridVersion::V6_2,
        };

        let mut engine = PhysicsEngine::new(grid_version);

        for line in track.standard_lines() {
            let physics_line = PhysicsLineBuilder::new(line.endpoints())
                .flipped(line.flipped())
                .left_extension(line.left_extension())
                .right_extension(line.right_extension())
                .height(line.height())
                .acceleration_multiplier(line.multiplier())
                .build();
            engine.add_line(physics_line);
        }

        let template_none_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::None));
        let template_comv1_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV1));
        let template_comv2_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV2));
        let template_lra_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::LRA));

        for rider in track.riders() {
            let template_id = if lra {
                template_lra_id
            } else {
                match rider.remount_version() {
                    lr_format_core::RemountVersion::None => template_none_id,
                    lr_format_core::RemountVersion::ComV1 => template_comv1_id,
                    lr_format_core::RemountVersion::ComV2 => template_comv2_id,
                    lr_format_core::RemountVersion::LRA => template_lra_id,
                }
            };

            let entity_id = engine
                .add_entity(template_id)
                .expect("Template id should be valid");

            if let Some(offset) = rider.start_offset() {
                engine
                    .set_entity_initial_offset(entity_id, offset)
                    .expect("Entity id should be valid");
            }

            if let Some(velocity) = rider.start_velocity() {
                engine
                    .set_entity_initial_velocity(entity_id, velocity)
                    .expect("Entity id should be valid");
            }
        }

        engine
    }
}
