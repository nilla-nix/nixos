/// nixos-rebuild flag for specifying a remote build host
pub const BUILD_HOST_FLAG: &str = "--build-host";

/// nixos-rebuild flag for specifying a remote target host
pub const TARGET_HOST_FLAG: &str = "--target-host";

/// Extract a host value from command-line arguments for a given flag.
///
/// Supports both `--flag value` and `--flag=value` formats.
pub fn extract_host_from_args(args: &[String], flag: &str) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag {
            // Check if next argument is the host value
            if let Some(next) = args.get(i + 1) {
                if !next.starts_with('-') {
                    return Some(next.clone());
                }
            }
        } else if arg.starts_with(&format!("{}=", flag)) {
            // Handle --flag=value format
            return arg.split('=').nth(1).map(|s| s.to_string());
        }
    }
    None
}

/// Extract both build and target host information from arguments.
pub fn extract_hosts_from_args(args: &[String]) -> (Option<String>, Option<String>) {
    (
        extract_host_from_args(args, BUILD_HOST_FLAG),
        extract_host_from_args(args, TARGET_HOST_FLAG),
    )
}
