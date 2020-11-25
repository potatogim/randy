use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::collections::{HashSet};

fn chars() -> HashSet<String> {
    let mut pids = HashSet::new();
    for dir_entry in fs::read_dir("/proc").unwrap() {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => continue,
        };

        let path = &entry.path().display().to_string();
        if path.chars().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    }

    return pids;
}

fn chars_skip() -> HashSet<String> {
    let mut pids = HashSet::new();
    for dir_entry in fs::read_dir("/proc").unwrap() {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => continue,
        };

        let path = String::from(entry.path().to_str().unwrap());
        if path.chars().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    }

    return pids;
}

fn bytes() -> HashSet<String> {
    let mut pids = HashSet::new();
    for dir_entry in fs::read_dir("/proc").unwrap() {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => continue,
        };

        let path = &entry.path().display().to_string();
        if path.bytes().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    }

    return pids;
}

fn bytes_no_display() -> HashSet<String> {
    let mut pids = HashSet::new();
    for dir_entry in fs::read_dir("/proc").unwrap() {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => continue,
        };

        let path = String::from(entry.path().to_str().unwrap());
        if path.bytes().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    }

    return pids;
}

fn bytes_foreach() -> HashSet<String> {
    let mut pids = HashSet::new();

    fs::read_dir("/proc").unwrap().for_each(|dir_entry| {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => return,
        };

        let path = &entry.path().display().to_string();
        if path.bytes().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    });

    return pids;
}

fn bytes_foreach_no_display() -> HashSet<String> {
    let mut pids = HashSet::new();

    fs::read_dir("/proc").unwrap().for_each(|dir_entry| {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => return,
        };

        let path = String::from(entry.path().to_str().unwrap());
        if path.bytes().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            pids.insert(pid.to_string());
        }
    });

    return pids;
}

fn criterion_benchmark(c: &mut Criterion) {
    // println!("{:?}", bytes_foreach_skip());
    // println!("{:?}", bytes_foreach());

    let mut group = c.benchmark_group("direntry");
    for i in 0..50 {
        group.bench_function(format!("bytes3 {}", i), |b| b.iter(|| bytes_foreach_no_display()));
        group.bench_function(format!("bytes2 {}", i), |b| b.iter(|| bytes_foreach()));
        group.bench_function(format!("bytes  {}", i), |b| b.iter(|| bytes()));
        group.bench_function(format!("chars2 {}", i), |b| b.iter(|| chars_skip()));
        group.bench_function(format!("chars  {}", i), |b| b.iter(|| chars()));
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);