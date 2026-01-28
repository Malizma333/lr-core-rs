
```js
{
  "points": [
    {
      "initial_position": {"x": f64, "y": f64},
      "is_contact": bool,
      "contact_friction": f64,
      "air_friction": f64,
    },
    ...
  ],
  "bones": [
    {
      "points": [u32, u32],
      "bias": f64,
      "rest_length_factor": f64,
      "should_repel": bool,
      "should_attract": bool,
      "endurance": f64,
      "adjustment_strength": f64,
      "endurance_remount_factor": f64,
      "adjustment_strength_remount_factor": f64,
    },
    ...
  ],
  "joints": [
    {
      "bones": [u32, u32],
    },
    ...
  ],
  "frames_until_dismounted": u32,
  "frames_until_remounting": u32,
  "frames_until_mounted": u32,
  // None | Com V1 | Com V2 | LRA
  "remount_version": 0 | 1 | 2 | 3,
}
```