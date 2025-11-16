#[cfg(test)]
mod tests {
    use geometry::Point;
    use lr_formatter_rs::{formats::json, track::GridVersion};
    use lr_physics::{
        AccelerationLine as PhysicsAccelerationLine, EngineBuilder, EngineState,
        GridVersion as PhysicsGridVersion, MountPhase, NormalLine as PhysicsNormalLine,
    };
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize)]
    struct EngineTestCaseEntity {
        points: Vec<(String, String, String, String)>,
        mount_state: Option<String>,
        sled_state: Option<String>,
        rider_state: Option<String>,
    }

    #[derive(Deserialize)]
    struct EngineTestCaseState {
        entities: Vec<EngineTestCaseEntity>,
    }

    #[derive(Deserialize)]
    struct EngineTestCase {
        test: String,
        frame: u32,
        file: String,
        state: EngineTestCaseState,
    }

    #[test]
    fn engine_fixtures() {
        let data =
            fs::read_to_string("tests/fixture_tests.json").expect("Failed to read JSON file");
        let test_cases: Vec<EngineTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        for test in test_cases {
            println!("Running test {}", test.test);

            let file_name = format!("tests/fixtures/{}.track.json", test.file);
            let file = fs::read(file_name).expect("Failed to read JSON file");
            let track = json::read(file).expect("Failed to parse track file");

            // TODO duplication across libraries
            let version = match track.metadata().grid_version() {
                GridVersion::V6_0 => PhysicsGridVersion::V6_0,
                GridVersion::V6_1 => PhysicsGridVersion::V6_1,
                GridVersion::V6_2 => PhysicsGridVersion::V6_2,
            };
            let mut engine = EngineBuilder::new(version).build();
            for line in track.line_group().acceleration_lines() {
                let acceleration_line = PhysicsAccelerationLine::new(
                    (
                        Point::new(line.x1(), line.y1()),
                        Point::new(line.x2(), line.y2()),
                    ),
                    line.flipped(),
                    line.left_extension(),
                    line.right_extension(),
                    line.multiplier().unwrap_or(1.0),
                );
                engine.create_line(Box::new(acceleration_line));
            }

            for line in track.line_group().standard_lines() {
                let normal_line = PhysicsNormalLine::new(
                    (
                        Point::new(line.x1(), line.y1()),
                        Point::new(line.x2(), line.y2()),
                    ),
                    line.flipped(),
                    line.left_extension(),
                    line.right_extension(),
                );
                engine.create_line(Box::new(normal_line));
            }

            let frame_data = engine.view_frame(test.frame);

            compare_states(frame_data, &test.state);
        }
    }

    fn compare_states(result_state: &EngineState, expected_state: &EngineTestCaseState) {
        todo!("compare states to check functionality")
    }
}
