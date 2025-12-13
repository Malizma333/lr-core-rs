Four entity class types:
- Point
- Bone
- Joint
- Skeleton

Each has four sub-types:
- Builder (interface for building templates)
- Template (reference for how to construct the entity itself)
- Entity (contains props populated by template as well as helper functions for functionality)
- State (contains everything that needs to be copied across frames, plugged into entities for calculations)

