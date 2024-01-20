
fn main() {
    let db = sled::open("my_db").unwrap();

    let _ = db.insert(&[1,1], "cippa").unwrap();
    let _ = db.insert(&[1,2], "cippa").unwrap();
    let _ = db.insert(&[2,2], "ambabarabaccicciccoco").unwrap();
    let _ = db.insert(&[1,3], "lippa").unwrap();
    let _ = db.insert(&[1,2,255], "lippa").unwrap();
    let old = db.insert(&[1,2], "lipp").unwrap();
    if let Some(oldv) = old {
        let _ = db.insert(&[1,2,1], oldv.clone());
        let _ = db.insert(&[1,2,123], oldv);
    }

    let iter = db.iter();
    for val in iter {
        let v = val.unwrap();
        println!("{:?} - {:?}", v.0, v.1);
    }

    //let start : &[u8] = &[1,2];
    //let end : &[u8] = &[1,3];
    //let arange = db.range(start..end);
    //let last = arange.last();
    //println!("last version is {:?}", last.unwrap().unwrap());

    //let arange = db.range(start..end);
    //for v in arange {
    //    let vv = v.unwrap();
    //    let l = vv.0.len();
    //    println!("key length: {}", l);
    //    println!("{:?} {:?}", vv.0, vv.1);
    //}

    let last = db.last().unwrap().unwrap();
    println!("last key {:?}-{:?}", last.0, last.1);
    println!("Another filter **************");
    let start : &[u8] = &[1,2];
    let end : &[u8] = &[last.0[0]+1,3];
    let arange = db.range(start..end);
    for v in arange {
        let vv = v.unwrap();
        println!("{:?} {:?}", vv.0, vv.1);
    }
}
