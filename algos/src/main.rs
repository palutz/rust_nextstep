fn binary_insert(source: &mut Vec<i32>, elem : i32) {
    let l = source.len();
    if l== 0 {
        source.insert(0, elem);
        return
    }
    let mut hidx = l - 1;
    let mut lidx = 0;
    let mut i = 0;
    // checking the extreme cases 
    if source[lidx] >= elem {
        i = 0
    } else if source[hidx] <= elem {
        i = l;
    } else {
        while lidx <= hidx {
            i = (hidx + lidx) / 2;
            println!("i={i}, {elem}<>{} lidx={lidx} hidx={hidx}", source[i]);
            match source[i].cmp(&elem) {
                std::cmp::Ordering::Less => lidx = i + 1,
                std::cmp:: Ordering::Equal => break,
                std::cmp::Ordering::Greater => hidx = i - 1,
            }
        };
    }
    println!(" --- lidx={lidx} hidx={hidx}");
    source.insert(i, elem);
}

pub fn test_insert() {
    let mut t = vec![0,1,1,2,3,4,4,4,4,6,7,8,9];
    binary_insert(&mut t, 5);
    println!("{:?}", t);
}

fn main() {
    test_insert();
}
