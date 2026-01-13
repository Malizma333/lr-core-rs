use geometry::Point;
use vector2d::{Vector2Df, Vector2Di};

pub(crate) const CELL_SIZE: f64 = 14.0;

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct CellKey(i32);

pub(crate) struct GridCell {
    position: Vector2Di,
    remainder: Vector2Df,
}

impl GridCell {
    pub(crate) fn new(world_position: Point) -> GridCell {
        let scaled_position = world_position / CELL_SIZE;
        #[expect(clippy::cast_possible_truncation)]
        let position = Vector2Di::new(
            scaled_position.x().floor() as i32,
            scaled_position.y().floor() as i32,
        );
        let remainder = world_position.vector_from(CELL_SIZE * Point::from(position));
        GridCell {
            position,
            remainder,
        }
    }

    pub(crate) fn position(&self) -> &Vector2Di {
        &self.position
    }

    pub(crate) fn remainder(&self) -> &Vector2Df {
        &self.remainder
    }

    pub(crate) fn get_key(&self) -> CellKey {
        let x_comp = if self.position.x() >= 0 {
            2 * self.position.x()
        } else {
            -2 * self.position.x() - 1
        };

        let y_comp = if self.position.y() >= 0 {
            2 * self.position.y()
        } else {
            -2 * self.position.y() - 1
        };

        let hash = if x_comp >= y_comp {
            x_comp * x_comp + x_comp + y_comp
        } else {
            y_comp * y_comp + x_comp
        };

        #[expect(clippy::integer_division)]
        let key = if hash % 2 == 1 {
            (-(hash - 1) / 2) - 1
        } else {
            hash / 2 + 1
        };

        CellKey(key)
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use std::collections::HashMap;

    use crate::grid_cell::{CELL_SIZE, CellKey, GridCell};

    #[test]
    fn unique_hash() {
        let mut seen: HashMap<CellKey, (i32, i32)> = HashMap::new();

        for i in -10..11 {
            for j in -10..11 {
                let key =
                    GridCell::new(CELL_SIZE * Point::new(f64::from(i), f64::from(j))).get_key();
                assert!(!seen.contains_key(&key));
                seen.insert(key, (i, j));
            }
        }
    }
}
