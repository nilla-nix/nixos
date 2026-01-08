use log::error;

use crate::commands::common;

pub async fn build_cmd(cli: &nixos_cli_def::Cli, args: &nixos_cli_def::commands::build::BuildArgs) {
    let path = match common::get_nilla_nix_path(&cli.project).await {
        Ok(p) => p,
        Err(e) => return error!("{}", e),
    };

    let hostname = match common::get_hostname(args.name.clone()) {
        Ok(h) => h,
        Err(e) => return error!("{}", e),
    };

    let attribute = common::format_attribute(&hostname);
    let (build_host, _) = crate::util::args::extract_hosts_from_args(&args.extra_nixos_rebuild_args);

    common::log_build_operation(&hostname, build_host.as_ref());

    let mut cmd = match common::build_nixos_rebuild_command(
        "build",
        &path,
        &attribute,
        &args.extra_nixos_rebuild_args,
        false, // build command never needs sudo
    ) {
        Ok(c) => c,
        Err(e) => return error!("{}", e),
    };

    cmd.status().await.unwrap();
}
