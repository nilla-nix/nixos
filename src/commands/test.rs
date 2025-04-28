use log::{debug, error, info};
use tokio::process::Command;

pub async fn test_cmd(cli: &nixos_cli_def::Cli, args: &nixos_cli_def::commands::test::TestArgs) {
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

    let sudo = match which::which("sudo") {
        Ok(s) => s,
        Err(_e) => match which::which("doas") {
            Ok(d) => d,
            Err(_e) => return error!("Could not find sudo or doas"),
        },
    };

    info!("Testing system {hostname}");
    Command::new(sudo)
        .arg("nixos-rebuild")
        .arg("test")
        .arg("--file")
        .arg(path.display().to_string())
        .arg("--attr")
        .arg(attribute)
        .status()
        .await
        .unwrap();
}
