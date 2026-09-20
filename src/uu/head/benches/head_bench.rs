// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use uu_head::uumain;
use uucore::benchmark::{create_test_file, run_util_function, text_data};

/// Benchmark the default mode (first 10 lines) on a large file
#[divan::bench(args = [500_000])]
fn head_default(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &[file_path_str]));
    });
}

/// Benchmark printing a large amount of lines, the line scanning is the hot path here
#[divan::bench(args = [200_000])]
fn head_many_lines(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &["-n", "200000", file_path_str]));
    });
}

/// Benchmark byte mode, which copies bytes without looking for line endings
#[divan::bench(args = [20])]
fn head_bytes(bencher: Bencher, size_mb: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_size(size_mb, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(
            uumain,
            &["-c", "10000000", file_path_str],
        ));
    });
}

/// Benchmark "all but the last N lines", which has to buffer the whole input
#[divan::bench(args = [200_000])]
fn head_all_but_last_lines(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &["-n", "-1000", file_path_str]));
    });
}

fn main() {
    divan::main();
}
