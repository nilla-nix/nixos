use log::{debug, error, info};
use tokio::process::Command;

pub async fn build_cmd(cli: &nixos_cli_def::Cli, args: &nixos_cli_def::commands::build::BuildArgs) {
    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };

    let mut path = project.get_path();

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => return error!("File not found"),
        _ => {}
    }

    let hostname = if let Some(name) = args.name.clone() {
        if name.contains('.') {
            return error!("Invalid hostname {}", name);
        } else {
            name
        }
    } else {
        gethostname::gethostname().into_string().unwrap()
    };

    let attribute = &format!("systems.nixos.\"{hostname}\".result");

    info!("Building system {hostname}");
    Command::new("nixos-rebuild")
        .arg("build")
        .arg("--file")
        .arg(path.display().to_string())
        .arg("--attr")
        .arg(attribute)
        .status()
        .await
        .unwrap();
}
