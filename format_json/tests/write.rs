#[cfg(test)]
mod tests {
    use std::fs;

    use format_core::track::{GridVersion, RGBColor, TrackBuilder, Vec2};

    #[test]
    fn write_metadata_features() {
        let mut track = TrackBuilder::default();
        track
            .metadata()
            .artist("bob")
            .description("this is a description")
            .duration(10)
            .grid_version(GridVersion::V6_0)
            .script("none")
            .start_position(Vec2::new(0.5, 1.5))
            .title("some title")
            .zero_velocity_start_riders(true);
        let result = format_json::write(&track.build().unwrap()).unwrap();
        let expected = fs::read("tests/metadata_features.json").unwrap();
        let result_string = String::from_utf8(result).unwrap();
        let expected_string = String::from_utf8(expected).unwrap();

        assert!(
            result_string == expected_string,
            "Metadata json should include expected features\n{}\n{}",
            result_string,
            expected_string
        )
    }

    #[test]
    fn write_all_metadata_features() {
        let mut track = TrackBuilder::default();
        track
            .metadata()
            .artist("bob")
            .audio_filename("this won't show")
            .audio_offset_until_start(1.5)
            .description("this is a description")
            .duration(10)
            .gravity_well_size(4.5)
            .grid_version(GridVersion::V6_0)
            .legacy_lra_fakie(true)
            .lra_remount(true)
            .remount_riders(true)
            .script("none")
            .start_background_color(RGBColor::new(0, 20, 40))
            .start_gravity(Vec2::new(10.0, 20.0))
            .start_line(0)
            .start_line_color(RGBColor::new(5, 6, 7))
            .start_position(Vec2::new(0.5, 1.5))
            .start_zoom(3.4)
            .title("some title")
            .zero_friction_riders(true)
            .zero_velocity_start_riders(true);
        let result = format_json::write(&track.build().unwrap()).unwrap();
        let expected = fs::read("tests/metadata_features.json").unwrap();
        let result_string = String::from_utf8(result).unwrap();
        let expected_string = String::from_utf8(expected).unwrap();

        assert!(
            result_string == expected_string,
            "Metadata json should not include extra features\n{}\n{}",
            result_string,
            expected_string
        )
    }
}
