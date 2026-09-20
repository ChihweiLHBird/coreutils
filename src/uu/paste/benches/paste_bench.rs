// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use std::path::PathBuf;
use tempfile::TempDir;
use uu_paste::uumain;
use uucore::benchmark::{create_test_file, run_util_function, text_data};

/// Create `num_files` files of `num_lines` lines each and return their paths
fn create_input_files(temp_dir: &TempDir, num_files: usize, num_lines: usize) -> Vec<PathBuf> {
    (0..num_files)
        .map(|i| {
            let dir = temp_dir.path().join(format!("input_{i}"));
            std::fs::create_dir_all(&dir).unwrap();
            let data = text_data::generate_by_lines(num_lines, 40);
            create_test_file(&data, &dir)
        })
        .collect()
}

/// Benchmark merging lines of several files side by side
#[divan::bench(args = [100_000])]
fn paste_parallel(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let files = create_input_files(&temp_dir, 3, num_lines);
    let args: Vec<&str> = files.iter().map(|p| p.to_str().unwrap()).collect();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &args));
    });
}

/// Benchmark serial mode, where each file is collapsed onto a single line
#[divan::bench(args = [100_000])]
fn paste_serial(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let files = create_input_files(&temp_dir, 3, num_lines);
    let mut args = vec!["-s"];
    args.extend(files.iter().map(|p| p.to_str().unwrap()));

    bencher.bench(|| {
        black_box(run_util_function(uumain, &args));
    });
}

/// Benchmark a multi character delimiter list, which cycles through delimiters
#[divan::bench(args = [100_000])]
fn paste_custom_delimiters(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let files = create_input_files(&temp_dir, 3, num_lines);
    let mut args = vec!["-d", ",;"];
    args.extend(files.iter().map(|p| p.to_str().unwrap()));

    bencher.bench(|| {
        black_box(run_util_function(uumain, &args));
    });
}

fn main() {
    divan::main();
}
