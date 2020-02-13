use std::io::prelude::*;
use std::io::SeekFrom;

#[test]
fn test_loop() {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("/tmp/rio-test-loop-data")
        .unwrap();

    file.write(b"hello hello again").unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    for _ in 0..1024 {
        let ring = rio::new().unwrap();
        let mut buf = vec![0_u8; 17];
        ring.read_at(&file, &mut buf, 0).wait().unwrap();
        assert_eq!(buf, b"hello hello again");
    }

    std::fs::remove_file("/tmp/rio-test-loop-data").unwrap();
}

