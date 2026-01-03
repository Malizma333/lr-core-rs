# Physics Engine

## Current Features

- Premade classic physics entities (normal and acceleration lines, rider and sled skeletons, and the rider-skeleton mount)
- Builders for custom line types, skeleton types, and mount types
- Timeline seeking (with iteration and subiteration views available)
- Timeline-based functions to modify physics parameter (such as gravity)
- Test fixtures borrowed from [lr-core-py](https://github.com/Malizma333/lr-core-py) to ensure compatibility with existing engines

# Engine API

## Creating a New Engine
```rust
// Allow for extending more engine parameters in the future (eg enable LRA compatibility fixes)
EngineBuilder::new().grid_version(version).build() -> Engine
```

## Timeline Viewing
```rust
engine.view_frame(frame) -> Vec<&Skeleton>
// Similar to LRA (iteration, subiteration) captured in a "Moment"
engine.view_moment(frame, moment) -> Vec<&Skeleton>
```

### Time-based Physics Parameters
```rust
// Overriding by passing defined functions
engine.define_gravity(get_gravity_at_time(frame) -> Vector2df)
engine.define_enabled_skeleton(get_enabled_skeletons_at_time(frame) -> Vec<bool>)
```

## Line Grid Modifications
```rust
engine.create_line(line) -> LineId
engine.update_line(line_id, new_line) -> ()
engine.delete_line(line_id) -> ()
```

## Entity Registry
```rust
let registry = engine.registry();

registry.register_skeleton_type(skeleton_template) -> SkeletonTemplateId
registry.create_skeleton(skeleton_template_id) -> SkeletonId
registry.delete_skeleton(skeleton_id) -> ()
```

### Skeleton Template Builder
```rust
let mut my_skeleton = registry.skeleton_template_builder();

let (my_skeleton, p1_id) = my_skeleton.point(Vec2df::new(0.0, 0.0)).contact().build();
let (my_skeleton, p2_id) = my_skeleton.point(Vec2df::new(1.0, 0.0)).friction(0.5).build();
let (my_skeleton, p3_id) = my_skeleton.point(Vec2df::new(1.0, 1.0)).build();
let (my_skeleton, b1_id) = my_skeleton.bone(p1_id, p2_id).repel().bias(1).build();
let (my_skeleton, b2_id) = my_skeleton.bone(p2_id, p3_id).adjustment_strength(1).build();
let (my_skeleton, j1_id) = my_skeleton.joint(b1_id, b2_id).build();

let skeleton_template_id = my_skeleton.build();
```

# Architecture

Four entity class types:
- Point
- Bone (Point-Point connections)
- Joint (Bone-Bone connections)
- Skeleton (contains Points, Bones, Joints)

Each has four sub-types:
- Builder (interface for building Templates)
- Template (reference for how to construct Entity)
- Entity (contains props populated by Template as well as helper functions operating on State)
- State (contains everything that needs to be copied across frames, plugged into entities for calculations)
