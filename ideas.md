## Questions
- How should version specific features (eg LRA Remount/bugfixes or .com Scarf) be plugins? custom logic extension?
- How to make sprites rendering over skeletons a plugin? (just add skeleton views)

## Timeline Engine Operations
- caching point states for each frame (basic clone)
- caching mount states for each frame (basic clone)
- removing from and adding to cache size whenever mounts change
- clearing front of cache whenever lines change
- swapping cached point states whenever frame info requested

## Engine Constraints
- Skeletons: <= 50
- Bones: <= 2,500
- Points: <= 2,500
- Mounts: <= 25
- Physics Lines <= 10,000
- Scenery Lines <= ~100,000,000

## Engine (Entity Registry) API
```rust
register_skeleton_type(?) -> () // some way of defining custom skeleton (see: custom skeleton def)
list_skeleton_templates() -> Vec<SkeletonTemplateId>
get_skeleton_template(skeleton_template_id) -> SkeletonTemplate 
add_skeleton(skeleton_template_id) -> ()
list_skeletons() -> Vec<SkeletonId>
get_skeleton(skeleton_id) -> Skeleton
remove_skeleton(skeleton_id) -> ()

register_mount_type(?) -> () // some way of defining mount between two skeletons
list_mount_templates() -> Vec<MountTemplateId>
get_mount_template(mount_template_id) -> MountTemplateId
add_mount(mount_template_id) -> ()
list_mounts() -> Vec
```

## Public Engine (Timeline) API
```rust
view_frame(frame) -> Vec<Skeleton>
view_moment(frame, moment) -> Vec<Skeleton>
get_gravity_at_time(frame) -> Vector2df // Overridable
get_enabled_skeletons_at_time(frame) -> Vec<bool> // Overridable
```

## Custom Skeleton Definition

API for building templates
```rust
let mut my_skeleton = registry.skeleton_template();

let p1 = my_skeleton.point(Vec2::new(0.0, 0.0));
let p2 = my_skeleton.point(Vec2::new(1.0, 0.0));
let p3 = my_skeleton.point(Vec2::new(1.0, 1.0));
let b1 = my_skeleton.bone(p1, p2);
let b2 = my_skeleton.bone(p2, p3);
let _j1 = my_skeleton.joint(b1, b2);

my_skeleton.build(); // add to the registry and take ownership of my_skeleton
```

## API for serializing and deserializing skeleton templates...? (probably best left as a plugin)
```rust
registry.add_skeleton_template(SkeletonTemplate {
    points: vec![
        Point { pos: vec2(0.0, 0.0), friction: 0.5 },
        Point { pos: vec2(1.0, 0.0), friction: 0.5 },
    ],
    bones: vec![(0, 1)],
    joints: vec![(0, 1)],
});

let serialized = registry.serialize_skeleton_template(skeleton_id);
```

implementation idea: btreemap with mount bone/joint diffs at dismount keyframes, then cumulatively construct by moving along diffs
```rust
0: [Add(id, id)]
10: [Add(id, id), Add(id, id), Remove(id, id)]
20: [Remove(id, id), Remove(id, id)]
```
