use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometry::Line;
use spatial_grid::{Grid, GridVersion};
use std::hint::black_box;
use vector2d::Vector2Df;

struct LineGenerator {
    x: f64,
    y: f64,
}

const LOWER_BOUND: f64 = -50.0;
const UPPER_BOUND: f64 = 50.0;

impl LineGenerator {
    fn new() -> Self {
        Self {
            x: LOWER_BOUND,
            y: LOWER_BOUND,
        }
    }

    fn get_next_line(&mut self) -> Line {
        if self.x < UPPER_BOUND {
            self.x += 1.0;
        } else {
            self.x = LOWER_BOUND;
            if self.y < UPPER_BOUND {
                self.y += 1.0;
            } else {
                self.y = LOWER_BOUND;
            }
        }

        Line::new(Vector2Df::zero(), Vector2Df::new(self.x, self.y))
    }
}

fn benchmark_add_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_add_line");
    let mut line_generator = LineGenerator::new();
    for version in [GridVersion::V6_0, GridVersion::V6_1, GridVersion::V6_2] {
        group.bench_function(BenchmarkId::from_parameter(version.to_string()), |b| {
            b.iter(|| {
                let line = line_generator.get_next_line();
                let mut grid = Grid::new(version);
                grid.add_line(black_box(&line));
            });
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmark_add_line
);
criterion_main!(benches);
