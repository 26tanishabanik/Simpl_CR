use clap::Parser;
use humantime::parse_duration;
use serde::Deserialize;
use std::path::PathBuf;
use std::{fs::read, io, time::Duration};

#[derive(Parser)]
#[command(name = "pull", version = "0.1.0")]
#[command(about = "Pull an image from a registry", long_about = None)]
struct Args {
    /// Name of the image to pull.
    image_name: String,

    /// Location of the client config file. If not specified and the default does not exist,
    /// the program's directory is searched as well.
    #[arg(
        short,
        long,
        env = "CRI_CONFIG_FILE",
        default_value = "/etc/crictl.yaml"
    )]
    config: PathBuf,

    /// Endpoint of CRI container runtime service (default: uses in order the first successful
    /// one of [
    ///     "unix:///var/run/dockershim.sock",
    ///     "unix:///run/containerd/containerd.sock",
    ///     "unix:///run/crio/crio.sock",
    ///     "unix:///var/run/cri-dockerd.sock",
    /// ]). Default is now deprecated and the endpoint should be set instead.
    #[arg(short, long, env = "CONTAINER_RUNTIME_ENDPOINT")]
    runtime_endpoint: Option<String>,

    /// Endpoint of CRI image manager service (default: uses 'runtime-endpoint' setting.
    #[arg(short, long, env = "IMAGE_SERVICE_ENDPOINT")]
    image_endpoint: Option<String>,

    /// Timeout of connecting to the server in seconds (e.g. 2s, 20s.).
    /// 0 or less is set to default.
    #[arg(short, long, value_parser = parse_duration, default_value = "2s")]
    timeout: Duration,
}

const DEFAULT_RUNTIME_ENDPOINTS: [&str; 4] = [
    "unix:///var/run/dockershim.sock",
    "unix:///run/containerd/containerd.sock",
    "unix:///run/crio/crio.sock",
    "unix:///var/run/cri-dockerd.sock",
];

#[derive(Deserialize, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Config {
    #[serde(default)]
    runtime_endpoint: String,
    #[serde(default)]
    image_endpoint: String,
}
#[derive(Debug)]
enum ConfigFileError {
    ReadFile(io::Error),
    Deserialize(serde_yaml::Error),
}

impl TryFrom<Args> for Config {
    type Error = ConfigFileError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let filepath = value.config.as_path();
        let config_data = read(filepath).map_err(|err| {
            eprintln!("failed to read file {}: {}", filepath.display(), err);
            ConfigFileError::ReadFile(err)
        })?;
        let config: Config = serde_yaml::from_slice(config_data.as_slice()).map_err(|err| {
            eprintln!("failed to deserialize file {}: {}", filepath.display(), err);
            ConfigFileError::Deserialize(err)
        })?;

        Ok(config)
    }
}

fn main() {
    let args = Args::parse();

    // Read and deserialize config file.
    let config = Config::try_from(args).unwrap();
}
