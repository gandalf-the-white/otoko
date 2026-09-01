use otoko::{
    config::SshConfig,
    probes::{ReadOnlyFreeBsdProbe, SshFreeBsdProbe},
};

#[tokio::test]
#[ignore = "requires access to the FreeBSD test host"]
async fn ssh_probe_reads_disk_usage() {
    let config = SshConfig::new(
        "freebsd",
        22,
        "spike",
        "/Users/laurent/.ssh/id_ed25519_proxmox",
        "/Users/laurent/.ssh/known_hosts",
    );

    let probe = SshFreeBsdProbe::connect(&config)
        .await
        .expect("SSH connection should succeed");

    let result = probe.disk_usage().await.expect("disk usage should succeed");

    assert_eq!(result.exit_status, 0);

    assert!(!result.stdout.is_empty());
}
