use rypper_reader::repomd;

fn main()
{
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_path = format!("{}/samples/valid-repomd-file.xml", &manifest_path);
    let read_file: repomd::RepoMd = repomd::RepoMd::from_file(&file_path).unwrap();
    println!("{:#?}", read_file);
}
