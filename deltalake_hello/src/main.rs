use deltalake::open_table;

fn main() {
    println!("Hello deltalake...");
    let _ = async {
        let table = 
            match open_table("../data/simple_table").await {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };
        let files = table.get_files();
        println!("found {:?} files", files.len());
        for f in files {
            println!("printing files...");
            println!("file: {:?}", f.filename().unwrap());
        }
    };
    println!("done");
}
