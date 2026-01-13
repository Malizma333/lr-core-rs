## Physics Engine API

### Creating a New Engine
```rust
Engine::new(grid_version) -> Engine
```

### Timeline Viewing
```rust
engine.view_frame(frame) -> Vec<&EntityState>
// Similar to LRA (iteration, subiteration) captured in a "Moment"
engine.view_moment(frame, moment) -> Vec<&EntityState>
```

### Editing Lines
```rust
PhysicsLineBuilder::new().build() -> PhysicsLine
PhysicsLineBuilder::from(line: PhysicsLine) -> PhysicsLineBuilder
```

### Managing Lines
```rust
engine.add_line(line: PhysicsLine) -> LineId
engine.get_line(line_id: LineId) -> &PhysicsLine
engine.replace_line(line_id: LineId, new_line: PhysicsLine) -> ()
engine.remove_line(line_id: LineId) -> ()
```
