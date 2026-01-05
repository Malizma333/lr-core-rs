use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use physics::Engine;
use std::{fs, hint::black_box};

// TODO benchmarks
// Free falling long length
// Heavy red multiline
// Heavy multirider
// Large single skeleton
// Long line

struct PhysicsBenchmark {
    file: &'static str,
    target_frame: u32,
    name: &'static str,
}

fn bench_simulate(group: &mut BenchmarkGroup<'_, WallTime>, engine: &Engine) {
    // TODO view target frame in benchmark
    todo!()
}

fn bench_engine_simulation(c: &mut Criterion) {
    let benchmarks = Vec::<PhysicsBenchmark>::new();

    // TODO read from fixtures file

    for benchmark in benchmarks {
        let file_name = format!(
            "../fixtures/physics/benchmarks/{}.track.json",
            benchmark.file
        );
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let track = format_json::read(&file).expect("Failed to parse track file");
        let engine = Engine::from_track(track, false);
        let mut group = c.benchmark_group(format!("physics/simulate/{}", benchmark.name));
        bench_simulate(&mut group, &engine);
        group.finish();
    }
}

criterion_group!(benches, bench_engine_simulation);
criterion_main!(benches);
