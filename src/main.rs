use ckb_vm::Bytes;

#[no_mangle]
pub fn _start() {
    let buffer = Bytes::from(include_bytes!("is13").to_vec());
    let r = ckb_vm::run::<u64, ckb_vm::SparseMemory<u64>>(
        &buffer,
        &["".into(), "13".into()], // Panic when change this to 12
        4 * 1024 * 1024,
    )
    .unwrap();
    assert!(r == 0);
}

fn main() {}
