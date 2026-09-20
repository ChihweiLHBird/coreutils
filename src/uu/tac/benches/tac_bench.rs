// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use uu_tac::uumain;
use uucore::benchmark::{create_test_file, run_util_function, text_data};

/// Benchmark reversing a file with many short lines
#[divan::bench(args = [200_000])]
fn tac_many_lines(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 40);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &[file_path_str]));
    });
}

/// Benchmark reversing a large file, dominated by the memchr based scan
#[divan::bench(args = [10])]
fn tac_large_file(bencher: Bencher, size_mb: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_size(size_mb, 80);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &[file_path_str]));
    });
}

/// Benchmark a custom separator, which uses a different search path
#[divan::bench(args = [100_000])]
fn tac_custom_separator(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 40);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(uumain, &["-s", "ab", file_path_str]));
    });
}

/// Benchmark the regex separator mode
#[divan::bench(args = [50_000])]
fn tac_regex_separator(bencher: Bencher, num_lines: usize) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data = text_data::generate_by_lines(num_lines, 40);
    let file_path = create_test_file(&data, temp_dir.path());
    let file_path_str = file_path.to_str().unwrap();

    bencher.bench(|| {
        black_box(run_util_function(
            uumain,
            &["-r", "-s", "[\n]", file_path_str],
        ));
    });
}

fn main() {
    divan::main();
}
