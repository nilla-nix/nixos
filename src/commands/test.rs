use log::error;

use crate::commands::common;

pub async fn test_cmd(cli: &nixos_cli_def::Cli, args: &nixos_cli_def::commands::test::TestArgs) {
    let path = match common::get_nilla_nix_path(&cli.project).await {
        Ok(p) => p,
        Err(e) => return error!("{}", e),
    };

    let hostname = match common::get_hostname(args.name.clone()) {
        Ok(h) => h,
        Err(e) => return error!("{}", e),
    };

    let attribute = common::format_attribute(&hostname);
    let (build_host, target_host) =
        crate::util::args::extract_hosts_from_args(&args.extra_nixos_rebuild_args);

    // Don't use sudo locally if --target-host is specified (activation happens remotely)
    let needs_sudo = target_host.is_none();

    common::log_operation("Testing", &hostname, build_host.as_ref(), target_host.as_ref());

    let mut cmd = match common::build_nixos_rebuild_command(
        "test",
        &path,
        &attribute,
        &args.extra_nixos_rebuild_args,
        needs_sudo,
    ) {
        Ok(c) => c,
        Err(e) => return error!("{}", e),
    };

    cmd.status().await.unwrap();
}
