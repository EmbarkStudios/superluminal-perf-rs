use std::num::Wrapping;

fn heavy_calc1() -> Wrapping<u64> {
    let mut v = Wrapping(1u64);
    for i in 0..1_000_000_000 {
        v += v * Wrapping(i);
    }
    v
}

fn heavy_calc2() -> Wrapping<u64> {
    let mut v = Wrapping(1u64);
    for i in 0..800_000_000 {
        v += v * Wrapping(i);
    }
    v
}

fn big_fn() {
    // work 1

    superluminal_perf::begin_event(b"example-work1\0");
    let v = heavy_calc1();
    superluminal_perf::end_event();
    println!("work1 done: {}", v);

    // work 2

    superluminal_perf::begin_event(b"example-work2\0");
    let v = heavy_calc2();
    superluminal_perf::end_event();
    println!("work2 done: {}", v);
}

fn main() {
    println!("Enabled: {}", superluminal_perf::enabled());

    superluminal_perf::set_current_thread_name("mythread").unwrap();

    big_fn();
}
