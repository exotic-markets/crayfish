use {
    crayfish_idl_generator::Convert,
    crayfish_metadata_extractor::{parsing::ParsingContext, program::Program},
    std::{
        path::Path,
        process::{Command, Stdio},
    },
};

pub fn main() {
    use cargo_manifest::Manifest;

    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = Path::new("/home/aursen/crayfish/examples/counter")
        .join("Cargo.toml")
        .canonicalize()
        .unwrap();
    let manifest = Manifest::from_path(manifest_path).unwrap();

    if let Some(workspace) = manifest.workspace {
        println!("{workspace:?}");
        println!("WORKSPACE");
        // Workspace program
        // for member in workspace.members {
        //     // TODO with one and cache what needed.
        // }
    } else {
        // println!("unique one");

        let command = Command::new("cargo") //TODO do it without expand `cargo +nightly rustc --profile=check -- -Zunpretty=expanded`
            .arg("expand")
            .arg("--lib")
            .arg(&format!("--package={}", manifest.package.unwrap().name))
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
            .stdout;
        let content = String::from_utf8(command).unwrap();
        let file = syn::parse_file(&content).unwrap();
        let context = ParsingContext::from(&file);
        let program = Program::try_from(context).unwrap();
        let idl = program.convert();

        println!("{:?}", serde_json::to_string(&idl));
    }
}
