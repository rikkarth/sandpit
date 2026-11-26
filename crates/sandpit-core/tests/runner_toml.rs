use std::time::Duration;

use sandpit_core::{Network, Runner};

const SAMPLE: &str = r#"
name = "review"
image = "ghcr.io/rikkarth/sandpit-agent-base@sha256:0123456789abcdef"
command = ["sh", "-c", "echo hello"]
prompt = "Review the diff."
artifacts = "/out"
network = "egress"

[limits]
timeout = "30m"

[[mounts]]
host = "./worktree"
container = "/workspace"
read_only = false
"#;

#[test]
fn sample_runner_round_trips() {
    let runner: Runner = toml::from_str(SAMPLE).expect("parse sample runner");

    assert_eq!(runner.network, Network::Egress);
    assert_eq!(runner.limits.timeout, Some(Duration::from_secs(30 * 60)));
    assert_eq!(runner.mounts.len(), 1);

    let encoded = toml::to_string_pretty(&runner).expect("serialize runner");
    let reparsed: Runner = toml::from_str(&encoded).expect("reparse serialized runner");
    assert_eq!(runner, reparsed);
}

#[test]
fn network_defaults_to_none() {
    let runner: Runner = toml::from_str(
        r#"
image = "example@sha256:abc"
command = ["true"]
"#,
    )
    .expect("parse minimal runner");

    assert_eq!(runner.network, Network::None);
    assert!(runner.mounts.is_empty());
}

#[test]
fn unknown_fields_are_rejected() {
    let result = toml::from_str::<Runner>(
        r#"
image = "example@sha256:abc"
command = ["true"]
netwrok = "egress"
"#,
    );

    assert!(result.is_err(), "typo in a field name must fail to parse");
}
