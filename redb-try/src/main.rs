use redb::{Database, Error, ReadableTable, TableDefinition};

const TABLE: TableDefinition<(i16, i16, i16), &str> = TableDefinition::new("my_data");
// use redb::*;
//const TABLE: TableDefinition<u64, u64> = TableDefinition::new("my_data");

// #[cfg(not(target_os = "wasi"))]
fn main() -> Result<(), Error> {
    let db = Database::create("my_data/data.1")?;

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert((1,1,1), "stringa1")?;
        table.insert((1,1,2), "stringa1.2")?;
        table.insert((1,1,3), "stringa1.3")?;
        table.insert((1,2,1), "stringa2")?;
        table.insert((1,3,1), "stringa3")?;
        table.insert((2,0,0), "stringa4")?;
        table.insert((2,1,0), "stringa4.1")?;
        table.insert((2,2,0), "stringa4.2")?;
        table.insert((2,2,2), "stringa4.3")?;
        table.insert((2,5,0), "stringa5")?;
    }
    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    println!("value = {:?}", table.get((1,1,1))?.unwrap().value());
    let iter = table.range((1,1,1)..(1,100,1000)).unwrap();
    let l = iter.last().unwrap();
    let v = l.unwrap();
    println!("LAST value = {:?} {:?}", v.0.value(), v.1.value());
    let iter = table.range((2,2,0)..(2,3,0)).unwrap();
    for el in iter {
        let v = el.unwrap();
        println!("value = {:?} {:?}", v.0.value(), v.1.value());
    }


    // Remove the last one
    let wrt = db.begin_write().unwrap();
    {
        let mut tt = wrt.open_table(TABLE).unwrap();
        let last = tt.pop_last().unwrap().unwrap();
        println!("pop last = {:?} {:?}", last.0.value(), last.1.value());
        // pop last = (2, 5, 0) "stringa5"
    }
    wrt.commit().unwrap();  // remove the last one from the db

    // std::thread::sleep(std::time::Duration::from_millis(500));

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    let iter = table.range((1,0,0)..).unwrap();
    for el in iter {
        let v = el.unwrap();
        println!("value = {:?} {:?}", v.0.value(), v.1.value());
    }
    Ok(())
}
