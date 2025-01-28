import { ProjectFile } from "../types/file";

export class ProjectManager extends BroadcastChannel {
  constructor(name: string) {
    super(name);
  }

  name: string = "";
  projectFiles: ProjectFile = {
    name: this.name,
    id: 0,
    content: "",
    children: [],
  };
  projectFileRefs: Map<number, ProjectFile> = new Map();

  wasm: Blob | undefined;

  public createDefaultFileStructure() {
    let rustFile = {
      id: 3,
      name: "lib.rs",
      content: this.contractContent(),
    };
    this.projectFileRefs.set(3, rustFile);

    let cargoFile = {
      id: 2,
      name: "cargo.toml",
      content: this.cargoContent(),
    };
    this.projectFileRefs.set(2, cargoFile);

    let srcFile = {
      id: 1,
      name: "src",
      content: "",
      children: [rustFile],
    };

    this.projectFiles["children"] = [srcFile, cargoFile];
  }

  public createEmbedFileStructure() {
    let rustFile = {
      id: 2,
      name: "lib.rs",
      content: this.contractContent(),
      children: [],
    };
    this.projectFileRefs.set(1, rustFile);

    this.projectFiles["children"] = [rustFile];
  }

  // react-accessible-treeview
  // useFlattenTree
  public getProjectFiles() {
    return this.projectFiles;
  }

  public getFileContent(id: number): string | undefined {
    return this.projectFileRefs.get(id)?.content;
  }

  public getFileName(id: number): string | undefined {
    return this.projectFileRefs.get(id)?.name;
  }

  public updateFileContent(id: number, content: string) {
    let projectFile = this.projectFileRefs.get(id);
    if (projectFile !== undefined) {
      projectFile.content = content;
      this.projectFileRefs.set(id, projectFile);
    }
  }

  private contractContent(): string {
    return `#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

const COUNTER: Symbol = Symbol::short("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env.events()
            .publish((COUNTER, Symbol::short("increment")), count);

        // Return the count to the caller.
        count
    }
}

mod test;
    `;
  }

  private cargoContent(): string {
    return `
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]
[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

[profile.release-with-logs]
inherits = "release"
debug-assertions = true

[dependencies]
soroban-sdk = "20.0.0-rc2"

[dev-dependencies]
rand = { version = "0.8.5", default-features = false, features = ["small_rng"] }
soroban-sdk = { version = "20.0.0-rc2", features = ["testutils"] }
    `;
  }
}
