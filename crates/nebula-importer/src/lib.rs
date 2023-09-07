use anyhow::Context;
use directories::ProjectDirs;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use oci_distribution::{manifest, secrets::RegistryAuth, Client, Reference};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::{runtime::Builder, sync::Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Contract {
    NoDigest(String),
    WithDigest {
        digest: Option<String>,
        reference: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    imports: HashMap<String, Contract>,
    cache: Option<PathBuf>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    nebula: Config,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub metadata: Metadata,
}

impl Contract {
    fn reference(&self) -> Reference {
        match self {
            Contract::NoDigest(reference) => reference.parse().unwrap(),
            Contract::WithDigest { reference, .. } => reference.parse().unwrap(),
        }
    }
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.reference().to_string().as_bytes());
        let bytes = hasher.finalize().to_vec();
        format!("{:x?}", bytes)
    }
}

macro_rules! throw_warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn import_all_contracts() {
    let package: Package = Figment::new()
        .merge(Toml::file("Cargo.toml"))
        .extract()
        .expect("Could not read config in `Cargo.toml`.");

    let config = &package.metadata.nebula;

    let contracts_dir = config.cache.clone().unwrap_or({
        let project_dirs = ProjectDirs::from("co", "eiger", "nebula-importer")
            .expect("Could not find a base path to cache contracts.");
        project_dirs.data_local_dir().into()
    });
    std::fs::create_dir_all(&contracts_dir)
        .expect("[importer] Contracts path could not be resolved");
    sync_contracts(&config, &contracts_dir).expect("[importer] Could not sync contracts.");
}

pub fn sync_contracts(config: &Config, cache: &PathBuf) -> anyhow::Result<()> {
    let client = Client::new(oci_distribution::client::ClientConfig {
        protocol: oci_distribution::client::ClientProtocol::Https,
        ..Default::default()
    });
    let client = Arc::new(Mutex::new(client));
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;

    for (name, contract) in &config.imports {
        runtime
            .block_on(runtime.spawn(find_and_sync_contract(
                name.clone(),
                contract.clone(),
                cache.clone(),
                client.clone(),
            )))
            .context(format!("Loading contract: {:?}", contract))?;
    }
    Ok(())
}

async fn find_and_sync_contract(
    name: String,
    contract: Contract,
    mut path: PathBuf,
    client: Arc<Mutex<Client>>,
) {
    path.push(format!("{name}_{}.wasm", contract.hash()));
    match path.try_exists() {
        Ok(true) => {
            std::fs::read(path).unwrap();
        }
        Ok(false) => {
            throw_warning!("Contract [{name}] could not be found in cache, fetching...");
            let mut client = client.lock().await;
            let reference = contract.reference();
            pull_wasm(&mut client, &RegistryAuth::Anonymous, &reference, &path).await;
        }
        Err(e) => throw_warning!("{e:?}"),
    };
}

pub(crate) async fn pull_wasm(
    client: &mut Client,
    auth: &RegistryAuth,
    reference: &Reference,
    output: &PathBuf,
) {
    let image_content = client
        .pull(reference, auth, vec![manifest::WASM_LAYER_MEDIA_TYPE])
        .await
        .expect(&format!(
            "Cannot pull Wasm module from {}",
            reference.to_string()
        ))
        .layers
        .into_iter()
        .next()
        .map(|layer| layer.data)
        .expect("No data found");
    tokio::fs::write(output, image_content)
        .await
        .expect("Cannot write to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use figment;

    #[test]
    fn test_config() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "Cargo.toml",
                r#"
                [package.metadata.nebula.imports]
                token = "ghcr.io/eigerco/nebula/contracts/token"
            "#,
            )?;
            let config: Config = Figment::new()
                .merge(Toml::file("Cargo.toml"))
                .extract()
                .unwrap();
            Ok(())
        });
    }
}
