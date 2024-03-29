use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    /// Host on which the MQTT Broker is running
    #[arg(
        long,
        short,
        env = "MQTT_BROKER",
        value_hint = ValueHint::Hostname,
        value_name = "HOST",
        default_value = "localhost",
    )]
    pub broker: String,

    /// Port on which the MQTT Broker is running
    #[arg(
        long,
        short,
        env = "MQTT_PORT",
        value_hint = ValueHint::Other,
        value_name = "INT",
        default_value = "1883",
    )]
    pub port: std::num::NonZeroU16,

    /// Username to access the MQTT broker.
    ///
    /// Anonymous access when not supplied.
    #[arg(
        long,
        short,
        env = "MQTT_USERNAME",
        value_hint = ValueHint::Username,
        value_name = "STRING",
        requires = "password",
    )]
    pub username: Option<String>,

    /// Password to access the MQTT broker.
    ///
    /// Passing the password via command line is insecure as the password can be read from the history!
    #[arg(
        long,
        env = "MQTT_PASSWORD",
        value_hint = ValueHint::Other,
        value_name = "STRING",
        hide_env_values = true,
        requires = "username",
    )]
    pub password: Option<String>,
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
