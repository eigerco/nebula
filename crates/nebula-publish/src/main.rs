use clap::Parser;
use nebula_publish::push_wasm;
use oci_distribution::{annotations, secrets::RegistryAuth, Client, Reference};
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Push {
    #[clap(short, long)]
    module: PathBuf,
    #[clap(short, long)]
    image: String,
    #[clap(long)]
    annotations: Vec<String>,
    #[clap(long)]
    username: Option<String>,
    #[clap(long)]
    password: Option<String>,
}
#[tokio::main]
pub async fn main() {
    let config = Push::parse();
    let mut values: HashMap<String, String> = HashMap::new();
    for annotation in config.annotations {
        let tmp: Vec<_> = annotation.splitn(2, '=').collect();
        if tmp.len() == 2 {
            values.insert(String::from(tmp[0]), String::from(tmp[1]));
        }
    }
    if !values.contains_key(&annotations::ORG_OPENCONTAINERS_IMAGE_TITLE.to_string()) {
        values.insert(
            annotations::ORG_OPENCONTAINERS_IMAGE_TITLE.to_string(),
            config.module.to_str().unwrap().to_string(),
        );
    }
    let mut client = Client::new(oci_distribution::client::ClientConfig {
        protocol: oci_distribution::client::ClientProtocol::Https,
        ..Default::default()
    });
    let reference: Reference = config.image.parse().expect("Not a valid image reference");
    let auth = if let Some(username) = &config.username {
        RegistryAuth::Basic(
            username.clone(),
            config.password.unwrap_or_default().clone(),
        )
    } else {
        RegistryAuth::Anonymous
    };
    push_wasm(&mut client, &auth, &reference, &config.module, Some(values)).await;
}
