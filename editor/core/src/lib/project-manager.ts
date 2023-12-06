import { flattenTree } from "react-accessible-treeview";
import { ProjectFile } from "../types/file";

export class ProjectManager extends BroadcastChannel {
  constructor(name: string) {
    super(name);
  }

  name: string = "";
  projectFiles: ProjectFile = {
    name: "",
    id: 0,
    content: "",
    children: [],
  };
  projectFileRefs: Map<number, ProjectFile> = new Map();
  currentFileId = 3;

  wasm: Blob | undefined;

  public createDefaultFileStructure() {
    let rustFile = {
      id: 3,
      name: "lib.rs",
      content: this.contractContent(),
      children: [],
    };
    this.projectFileRefs.set(3, rustFile);

    let cargoFile = {
      id: 2,
      name: "cargo.toml",
      content: this.cargoContent(),
      children: [],
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

  public getNodes() {
    return flattenTree(this.projectFiles);
  }

  public setCurrentFileId(id: number) {
    this.currentFileId = id;
    this.postMessage({ event: "setCurrentFile", id: id });
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
    return `
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
  pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
    vec![&env, symbol_short!("Hello"), to]
  }

  pub fn good_bye(env: Env, admin: Address, to: Symbol, from: u32) -> Vec<Symbol> {
    vec![&env, symbol_short!("Hello"), to]
  }
}
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
