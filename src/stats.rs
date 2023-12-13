pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, is_last: bool) {
    *total_bytes += num_read;
    if !silent {
        eprint!("\rbytes read: {}", total_bytes);
        if is_last {
            eprintln!();
        }
    }
}
