use log::{debug, error, info};
use std::path::PathBuf;
use tokio::process::Command;

/// Common arguments trait for commands that support nixos-rebuild passthrough
pub trait NixosRebuildArgs {
    fn name(&self) -> Option<String>;
    fn extra_nixos_rebuild_args(&self) -> &[String];
}

/// Get the path to nilla.nix file from the project
pub async fn get_nilla_nix_path(project: &str) -> Result<PathBuf, String> {
    debug!("Resolving project {}", project);
    let project = crate::util::project::resolve(project)
        .await
        .map_err(|_| format!("Could not find project {}", project))?;

    let mut path = project.get_path();
    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => Err("File not found".to_string()),
        _ => Ok(path),
    }
}

/// Get and validate hostname from args or system
pub fn get_hostname(name: Option<String>) -> Result<String, String> {
    if let Some(name) = name {
        if name.contains('.') {
            return Err(format!("Invalid hostname {}", name));
        }
        Ok(name)
    } else {
        Ok(gethostname::gethostname().into_string().unwrap())
    }
}

/// Format the NixOS attribute path for a hostname
pub fn format_attribute(hostname: &str) -> String {
    format!("systems.nixos.\"{hostname}\".result")
}

/// Get sudo/doas command path
fn get_sudo_command() -> Result<std::path::PathBuf, String> {
    which::which("sudo")
        .or_else(|_| which::which("doas"))
        .map_err(|_| "Could not find sudo or doas".to_string())
}

/// Build the nixos-rebuild command with proper sudo handling
pub fn build_nixos_rebuild_command(
    subcommand: &str,
    path: &PathBuf,
    attribute: &str,
    extra_args: &[String],
    needs_sudo: bool,
) -> Result<Command, String> {
    let mut cmd = if needs_sudo {
        let sudo = get_sudo_command()?;
        let mut sudo_cmd = Command::new(sudo);
        sudo_cmd.arg("nixos-rebuild");
        sudo_cmd
    } else {
        Command::new("nixos-rebuild")
    };

    cmd.arg(subcommand)
        .arg("--file")
        .arg(path.display().to_string())
        .arg("--attr")
        .arg(attribute);

    for arg in extra_args {
        cmd.arg(arg);
    }

    Ok(cmd)
}

/// Format location string for logging
fn format_location(host: Option<&String>) -> &str {
    host.map(|h| h.as_str()).unwrap_or("locally")
}

/// Log operation with location information
pub fn log_operation(
    action: &str,
    hostname: &str,
    build_host: Option<&String>,
    target_host: Option<&String>,
) {
    let build_location = format_location(build_host);
    let activate_location = format_location(target_host);

    if build_host.is_some() || target_host.is_some() {
        info!("{action} system {hostname} (build: {build_location}, activate: {activate_location})");
    } else {
        info!("{action} system {hostname} locally");
    }
}

/// Log build operation (simpler version for build command)
pub fn log_build_operation(hostname: &str, build_host: Option<&String>) {
    let build_location = format_location(build_host);
    if build_host.is_some() {
        info!("Building system {hostname} on {build_location}");
    } else {
        info!("Building system {hostname} locally");
    }
}
