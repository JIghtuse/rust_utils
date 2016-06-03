# rust_utils
Small utilities

## who_eats_my_hard_drive

This utility tries to detect Linux distribution it is running on and calls
distribution-specific command to list installed packages and their size. You
must likely will need to sort output:

    cargo run --bin who_eats_my_hard_drive |sort -n
