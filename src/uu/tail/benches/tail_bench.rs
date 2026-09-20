// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use uu_tail::uumain;
use uucore::benchmark::{create_test_file, run_util_function, text_data};

/// Benchmark the default mode (last 10 lines) on a large file
#[divan::bench(args = [500_000])]
fn tail_default(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &[file_path_str]));
    });
}

/// Benchmark printing a large number of trailing lines
#[divan::bench(args = [200_000])]
fn tail_many_lines(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &["-n", "100000", file_path_str]));
    });
}

/// Benchmark byte mode on a large file
#[divan::bench(args = [20])]
fn tail_bytes(bencher: Bencher, size_mb: usize) {
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

/// Benchmark "start at line N", which streams the whole file out
#[divan::bench(args = [200_000])]
fn tail_from_line(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &["-n", "+1000", file_path_str]));
    });
}

fn main() {
    divan::main();
}
