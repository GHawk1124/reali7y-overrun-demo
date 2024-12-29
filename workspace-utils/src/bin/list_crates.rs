use cargo_metadata::MetadataCommand;

fn main() {
    let metadata = MetadataCommand::new().exec().unwrap();
    for package in metadata.packages {
        println!("{}", package.name);
    }
}
