use rand::distributions::{Alphanumeric, DistString};
use rusty_leveldb::{Options, DB};

const KEY_LEN: usize = 4;
const VAL_LEN: usize = 8;

fn gen_string(n: usize) -> String {
    Alphanumeric
        .sample_string(&mut rand::thread_rng(), n)
        .to_lowercase()
}

fn write(db: &mut DB, n: usize) {
    time_test::time_test!("write");
    for _ in 0..n {
        let (k, v) = (gen_string(KEY_LEN), gen_string(VAL_LEN));
        db.put(k.as_bytes(), v.as_bytes()).unwrap();
    }
    {
        time_test::time_test!("write-flush");
        db.flush().unwrap();
    }
}

fn read(db: &mut DB, n: usize) -> usize {
    let mut succ = 0;
    time_test::time_test!("read");
    for _ in 0..n {
        let k = gen_string(KEY_LEN);
        if let Some(_) = db.get(k.as_bytes()) {
            succ += 1;
        }
    }
    succ
}

fn write_once(db: &mut DB, n: usize) {
    let (k, v) = ("blues", "maysonshen");
    db.put(k.as_bytes(), v.as_bytes()).unwrap();
}

fn main() {
    let n = 100_000;
    let m = 10;
    let path = "debugdb";
    let mut entries = 0;

    let opt = Options::default();
    // opt.compression_type = rusty_leveldb::CompressionType::CompressionSnappy;
    let mut db = DB::open(path, opt).unwrap();
    write_once(&mut db, n);

    // for i in 0..m {
    //     write(&mut db, n);
    //     entries += n;
    //     println!("Wrote {} entries ({}/{})", entries, i + 1, m);
    //     let s = read(&mut db, n);
    //     println!("Read back {} entries (found {}) ({}/{})", n, s, i + 1, m);
    // }
}
