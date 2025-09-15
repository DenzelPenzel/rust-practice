use std::{fs::File, io::Write};

#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
struct Data {
    number: u16,
    tag: [u8; 8],
}

#[derive(Debug)]
struct DataWrapper {
    number: u16,
    tag: String,
}

fn main() {
    let dt = vec![
        Data {
            number: 1,
            tag: *b"hello   ",
        },
        Data {
            number: 2,
            tag: *b"world   ",
        },
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&dt);
    std::fs::write("data.bin", bytes).unwrap();


    // read the data back
    let bytes = std::fs::read("data.bin").unwrap();
    let res: &[Data] = bytemuck::cast_slice(&bytes);

    println!("{res:?}");

    // ===============================

    let dt = DataWrapper{
        number: 1,
        tag: "Hello word".to_string(),
    };

    // write the record in parts
    let mut file = File::create("bytes.bin").unwrap();

    assert_eq!(file.write(&dt.number.to_le_bytes()).unwrap(), 2);

    let len = dt.tag.as_bytes().len();
    assert_eq!(file.write(&(len as u64).to_le_bytes()).unwrap(), 8);

    assert_eq!(file.write(dt.tag.as_bytes()).unwrap(), len);

    // Read back
    let bytes = std::fs::read("bytes.bin").unwrap();

    // read the number
    let number = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

    let len = u64::from_le_bytes(bytes[2..10].try_into().unwrap());

    let tag = std::str::from_utf8(&bytes[10..(10 + len as usize)]).unwrap();

    let new_dt = DataWrapper{
        number,
        tag: tag.to_string(),
    };

    println!("{new_dt:?}");
}
