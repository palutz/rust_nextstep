use deltalake::open_table;
use futures::executor::block_on;

fn main() {
    println!("Hello deltalake...");
    let table = block_on(open_table("./data/simple_table")).unwrap();
    println!("{table}");

    let files = table.get_files();
    for f in files {
        println!("{f}");
    }
    println!("done");
}
