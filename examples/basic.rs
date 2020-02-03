use std::num::Wrapping;

fn heavy_calc1() -> Wrapping<u64> {
    let mut v = Wrapping(1u64);
    for i in 0..1_000_000_000 {
        v += v * Wrapping(i);
    }
    v
}

fn main() {
    // work 1

    superluminal_perf::begin_event(b"example_loop_1\0");
    let v = heavy_calc1();
    superluminal_perf::end_event();
    println!("work done: {}", v);

    // work 2
}
