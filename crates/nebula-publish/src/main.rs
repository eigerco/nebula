use clap::Parser;
use oci_distribution::{
    annotations,
    client::{Config, ImageLayer},
    manifest,
    secrets::RegistryAuth,
    Client, Reference,
};
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
    push_wasm(
        &mut client,
        &RegistryAuth::Anonymous,
        &reference,
        &config.module,
        Some(values),
    )
    .await;
}

async fn push_wasm(
    client: &mut Client,
    auth: &RegistryAuth,
    reference: &Reference,
    module: &PathBuf,
    annotations: Option<HashMap<String, String>>,
) {
    let data = tokio::fs::read(module)
        .await
        .expect("Cannot read Wasm module from disk");

    let layers = vec![ImageLayer::new(
        data,
        manifest::WASM_LAYER_MEDIA_TYPE.to_string(),
        None,
    )];

    let config = Config {
        data: b"{}".to_vec(),
        media_type: manifest::WASM_CONFIG_MEDIA_TYPE.to_string(),
        annotations: None,
    };

    let image_manifest = manifest::OciImageManifest::build(&layers, &config, annotations);

    let response = client
        .push(&reference, &layers, config, &auth, Some(image_manifest))
        .await
        .map(|push_response| push_response.manifest_url)
        .expect("Cannot push Wasm module");

    println!("Wasm module successfully pushed {:?}", response);
}
