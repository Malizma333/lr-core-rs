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

registry.register_mount_type(mount_template) -> MountTemplateId
registry.create_mount(mount_template_id) -> MountId
registry.delete_mount(mount_id) -> ()
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

### Skeleton Mount Template Builder
```rust
let mut my_mount = registry.mount_template_builder();
let (skeleton_template1, skeleton_template2) = my_mount.link(skeleton_template_id1, skeleton_template_id2);

let p1_id = skeleton_template1.get_point(0);
let p2_id = skeleton_template2.get_point(1);
let b1_id = skeleton_template1.get_bone(0);
let b2_id = skeleton_template2.get_bone(1);

let (my_mount, mb1_id) = my_mount.bone(0, p1_id, p2_id).endurance(0.8).build();
let (my_mount, mb2_id) = my_mount.bone(1, p1_id, p2_id).endurance(0.5).build();
let (my_mount, mj1_id) = my_mount.joint(b1_id, b2_id).build();
let (my_mount, mj2_id) = my_mount.joint(mb1_id, b2_id).build();

let mount_template_id = my_mount.build();
```

- state management
  - points
    - position, velocity, previous position
  - skeletons
    - intact
  - mounts
    - mount bones, mount joints
      - connection between points, but props should remain throughout?
    - mount state, mount timers
- skeleton entity logic
  - what should mount logic be?
- engine frame simulation
- version-specific quirks
- line json format
  - generalize logic
