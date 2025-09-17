use geometry::{Line, Point, Rectangle};
use std::collections::{BTreeSet, HashMap, HashSet};
use vector2d::{Vector2Df, Vector2Di};

use crate::grid::grid_cell::{CellKey, GridCell};

pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}

pub type LineId = u32;

pub struct Grid {
    version: GridVersion,
    cell_size: u32,
    cells: HashMap<CellKey, BTreeSet<LineId>>,
}

impl Grid {
    pub fn new(version: GridVersion, cell_size: u32) -> Grid {
        Grid {
            version,
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn register(&mut self, line_id: LineId, position: &GridCell) {
        let cell_key = position.get_key();
        self.cells
            .entry(cell_key)
            .or_insert_with(BTreeSet::new)
            .insert(line_id);
    }

    fn unregister(&mut self, line_id: LineId, position: &GridCell) {
        let cell_key = position.get_key();
        if let Some(cell) = self.cells.get_mut(&cell_key) {
            cell.remove(&line_id);
        }
    }

    pub fn add_line(&mut self, id: LineId, endpoints: &Line) {
        let cell_positions = self.get_cell_positions_along(&endpoints);
        for position in cell_positions {
            self.register(id, &position);
        }
    }

    pub fn remove_line(&mut self, id: LineId, endpoints: &Line) {
        let cell_positions = self.get_cell_positions_along(&endpoints);
        for position in cell_positions {
            self.unregister(id, &position);
        }
    }

    pub fn move_line(&mut self, id: LineId, old_endpoints: &Line, new_endpoints: &Line) {
        let cell_positions = self.get_cell_positions_along(&old_endpoints);
        for position in cell_positions {
            self.unregister(id, &position);
        }

        let new_cell_positions = self.get_cell_positions_along(&new_endpoints);
        for position in new_cell_positions {
            self.register(id, &position);
        }
    }

    fn get_next_position(&self, current_position: Point, endpoints: &Line) -> Point {
        let current_cell = GridCell::new(current_position, self.cell_size);
        let endpoint_vector = endpoints.get_vector();

        let mut delta_x = if endpoint_vector.x() > 0.0 {
            f64::from(self.cell_size) - current_cell.remainder().x()
        } else {
            -1.0 - current_cell.remainder().x()
        };

        let mut delta_y = if endpoint_vector.y() > 0.0 {
            f64::from(self.cell_size) - current_cell.remainder().y()
        } else {
            -1.0 - current_cell.remainder().y()
        };

        if matches!(self.version, GridVersion::V6_2) {
            if current_cell.position().x() < 0 {
                delta_x = if endpoint_vector.x() > 0.0 {
                    f64::from(self.cell_size) + current_cell.remainder().x()
                } else {
                    -(f64::from(self.cell_size) + current_cell.remainder().x())
                }
            }
            if current_cell.position().y() < 0 {
                delta_y = if endpoint_vector.y() > 0.0 {
                    f64::from(self.cell_size) + current_cell.remainder().y()
                } else {
                    -(f64::from(self.cell_size) + current_cell.remainder().y())
                }
            }
        }

        if endpoint_vector.x() == 0.0 {
            Point::new(current_position.x(), current_position.y() + delta_y)
        } else if endpoint_vector.y() == 0.0 {
            Point::new(current_position.x() + delta_x, current_position.y())
        } else if matches!(self.version, GridVersion::V6_1) {
            let slope = endpoint_vector.y() / endpoint_vector.x();
            let y_intercept = endpoints.p0().y() - slope * endpoints.p0().x();
            let next_x = ((current_position.y() + delta_y - y_intercept) / slope).round();
            let next_y = (slope * (current_position.x() + delta_x) + y_intercept).round();
            if (next_y - current_position.y()).abs() < delta_y.abs() {
                Point::new(current_position.x() + delta_x, next_y)
            } else if (next_y - current_position.y()).abs() == delta_y.abs() {
                Point::new(
                    current_position.x() + delta_x,
                    current_position.y() + delta_y,
                )
            } else {
                Point::new(next_x, current_position.y() + delta_y)
            }
        } else {
            let y_based_delta_x = delta_y * (endpoint_vector.x() / endpoint_vector.y());
            let x_based_delta_y = delta_x * (endpoint_vector.y() / endpoint_vector.x());
            let next_x = current_position.x() + y_based_delta_x;
            let next_y = current_position.y() + x_based_delta_y;
            if x_based_delta_y.abs() < delta_y.abs() {
                Point::new(current_position.x() + delta_x, next_y)
            } else if x_based_delta_y.abs() == delta_y.abs() {
                Point::new(
                    current_position.x() + delta_x,
                    current_position.y() + delta_y,
                )
            } else {
                Point::new(next_x, current_position.y() + delta_y)
            }
        }
    }

    fn get_cell_positions_along(&self, endpoints: &Line) -> Vec<GridCell> {
        let initial_cell = GridCell::new(endpoints.p0(), self.cell_size);
        let final_cell = GridCell::new(endpoints.p1(), self.cell_size);

        if endpoints.p0() == endpoints.p1() || initial_cell.position() == final_cell.position() {
            return vec![initial_cell];
        }

        let mut cells: Vec<GridCell> = Vec::new();
        let lower_bound_x = initial_cell.position().x().min(final_cell.position().x());
        let upper_bound_x = initial_cell.position().x().max(final_cell.position().x());
        let lower_bound_y = initial_cell.position().y().min(final_cell.position().y());
        let upper_bound_y = initial_cell.position().y().max(final_cell.position().y());
        let mut current_position_along_line = endpoints.p0();
        let mut current_cell = initial_cell;
        let line_vector = endpoints.get_vector();
        let line_normal = line_vector.rotate_ccw() * (1.0 / line_vector.length());

        if matches!(self.version, GridVersion::V6_0) {
            let line_halfway = 0.5 * Vector2Df::new(line_vector.x().abs(), line_vector.y().abs());
            let line_midpoint = endpoints.p0() + 0.5 * line_vector;
            let absolute_normal = Vector2Df::new(line_normal.x().abs(), line_normal.y().abs());
            for cell_x in lower_bound_x..upper_bound_x + 1 {
                for cell_y in lower_bound_y..upper_bound_y + 1 {
                    let current_position_in_box = f64::from(self.cell_size)
                        * Vector2Df::new(f64::from(cell_x) + 0.5, f64::from(cell_y) + 0.5);
                    let next_cell_position = GridCell::new(current_position_in_box, self.cell_size);
                    let distance_between_centers = line_midpoint - current_position_in_box;
                    let distance_from_cell_center =
                        Vector2Df::dot(absolute_normal, *next_cell_position.remainder());
                    let cell_overlap_into_hitbox = Vector2Df::dot(
                        distance_from_cell_center * Vector2Df::one(),
                        absolute_normal,
                    );
                    let normal_distance_between_centers =
                        Vector2Df::dot(line_normal, distance_between_centers);
                    let distance_from_line = (normal_distance_between_centers * line_normal.x())
                        .abs()
                        + (normal_distance_between_centers * line_normal.y()).abs();
                    if line_halfway.x() + next_cell_position.remainder().x()
                        >= distance_between_centers.x().abs()
                        && line_halfway.y() + next_cell_position.remainder().y()
                            >= distance_between_centers.y().abs()
                        && cell_overlap_into_hitbox >= distance_from_line
                    {
                        cells.push(next_cell_position);
                    }
                }
            }
        } else {
            while lower_bound_x <= current_cell.position().x()
                && current_cell.position().x() <= upper_bound_x
                && lower_bound_y <= current_cell.position().y()
                && current_cell.position().y() <= upper_bound_y
            {
                current_position_along_line =
                    self.get_next_position(current_position_along_line, endpoints);
                let next_cell = GridCell::new(current_position_along_line, self.cell_size);
                if next_cell.position() == current_cell.position() {
                    break;
                } else {
                    cells.push(current_cell);
                    current_cell = next_cell;
                }
            }
        }

        cells
    }

    fn get_lines_between_grid_cells(&self, cell1: &GridCell, cell2: &GridCell) -> HashSet<u32> {
        let lower = Vector2Di::new(
            cell1.position().x().min(cell2.position().x()),
            cell1.position().y().min(cell2.position().y()),
        );
        let upper = Vector2Di::new(
            cell1.position().x().max(cell2.position().x()),
            cell1.position().y().max(cell2.position().y()),
        );
        let mut lines_between: HashSet<u32> = HashSet::new();
        for cell_x in lower.x()..upper.x() + 1 {
            for cell_y in lower.y()..upper.y() + 1 {
                let key = GridCell::new(
                    Point::new(f64::from(cell_x), f64::from(cell_y)),
                    self.cell_size,
                )
                .get_key();
                if let Some(cell) = self.cells.get(&key) {
                    lines_between.extend(cell.iter());
                }
            }
        }
        lines_between
    }

    /** Finds all of the lines that lie near a rectangular region and returns their ids */
    pub fn select_lines_near_rect(&self, rectangle: &Rectangle) -> Vec<u32> {
        let lower_cell = GridCell::new(rectangle.bottom_left(), self.cell_size);
        let upper_cell = GridCell::new(rectangle.top_right(), self.cell_size);
        let mut lines_included: Vec<u32> = Vec::new();

        for id in self.get_lines_between_grid_cells(&lower_cell, &upper_cell) {
            lines_included.push(id);
        }

        lines_included
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, grid_cell::GridCell, line_grid::LineId};
    use geometry::{Circle, Line, Point, Rectangle};
    use serde::Deserialize;
    use std::{collections::HashMap, fs};
    use vector2d::Vector2Df;

    #[derive(Deserialize)]
    struct GridTestCase {
        name: String,
        input: (f64, f64, f64, f64),
        expected: Vec<(i32, i32)>,
    }

    #[test]
    fn add_move_remove_lines() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);
        let line0 = Line::new(Point::zero(), Point::one());
        let line1 = Line::new(Point::one() * 2.0, Point::one() * 3.0);
        let cell_key = GridCell::new(Vector2Df::zero(), 1).get_key();

        assert!(grid.cells.is_empty(), "new grid should have no cells");

        grid.add_line(0, &line0);
        grid.add_line(1, &line0);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| cell.contains(&0) && cell.contains(&1)),
            "first cell should have both line ids"
        );

        grid.remove_line(1, &line0);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| cell.contains(&0) && !cell.contains(&1)),
            "first cell should only have one line ids after remove"
        );

        grid.move_line(0, &line0, &line1);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| !cell.contains(&0) && !cell.contains(&1)),
            "first cell should have no line ids after move"
        );

        grid.remove_line(0, &line1);

        assert!(
            !grid.cells.is_empty(),
            "grid should still have cells after removing all lines"
        );
    }

    #[test]
    fn select_rect() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);
        grid.add_line(0, &Line::new(Point::one() * 0.25, Point::one() * 0.5));
        grid.add_line(1, &Line::new(Point::one() * 0.5, Point::one() * 1.5));
        grid.add_line(2, &Line::new(Point::one() * 0.5, Point::one() * 2.5));
        let lines =
            grid.select_lines_near_rect(&Rectangle::new(Point::one() * -1.0, Point::one() * 5.0));
        assert!(lines.len() == 3, "large rect should include all");
        let lines =
            grid.select_lines_near_rect(&Rectangle::new(Point::one() * 0.25, Point::one() * 0.75));
        assert!(
            lines.len() == 3,
            "rect intersecting each should include all"
        );
        let lines =
            grid.select_lines_near_rect(&Rectangle::new(Point::one() * 1.25, Point::one() * 1.75));
        assert!(lines.len() == 2, "rect intersecting two should include two");
        let lines =
            grid.select_lines_near_rect(&Rectangle::new(Point::one() * -0.75, Point::one() * -0.5));
        assert!(
            lines.len() == 0,
            "rect intersecting nothing should include nothing"
        );
    }

    fn run_grid_tests(grid: Grid, data: String) {
        let test_cases: Vec<GridTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        for case in test_cases {
            let line = Line::new(
                Point::new(case.input.0, case.input.1),
                Point::new(case.input.2, case.input.3),
            );
            let grid_cells = grid.get_cell_positions_along(&line);
            assert!(
                grid_cells.len() == case.expected.len(),
                "Test '{}' failed",
                case.name
            );
            for i in 0..grid_cells.len() {
                assert!(
                    grid_cells[i].position().x() == case.expected[i].0
                        && grid_cells[i].position().y() == case.expected[i].1,
                    "Test '{}' failed",
                    case.name
                );
            }
        }
    }

    #[test]
    fn cell_positions_of_line_60() {
        let grid = Grid::new(super::GridVersion::V6_0, 14);
        let data =
            fs::read_to_string("tests/grid_60_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }

    #[test]
    fn cell_positions_of_line_61() {
        let grid = Grid::new(super::GridVersion::V6_1, 14);
        let data =
            fs::read_to_string("tests/grid_61_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }

    #[test]
    fn cell_positions_of_line_62() {
        let grid = Grid::new(super::GridVersion::V6_2, 14);
        let data =
            fs::read_to_string("tests/grid_62_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }
}
