extern crate deno_core;
extern crate futures;

use deno_core::plugin_api::{Buf, Interface, Op, ZeroCopyBuf};
use futures::future::FutureExt;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("testSync", op_test_sync);
    interface.register_op("testAsync", op_test_async);
}

pub fn op_test_sync(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = zero_copy[0].clone();
    let buf = zero_copy[1].clone();
    let data_str = std::str::from_utf8(&data[..]).unwrap();
    let buf_str = std::str::from_utf8(&buf[..]).unwrap();
    println!(
        "Hello from plugin. data: {} | zero_copy: {}",
        data_str, buf_str
    );

    let result = b"test";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

pub fn op_test_async(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = zero_copy[0].clone();
    let buf = zero_copy[1].clone();
    let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
    let fut = async move {
        let buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!(
            "Hello from plugin. data: {} | zero_copy: {}",
            data_str, buf_str
        );
        let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            tx.send(Ok(())).unwrap();
        });
        assert!(rx.await.is_ok());
        let result = b"test";
        let result_box: Buf = Box::new(*result);
        result_box
    };

    Op::Async(fut.boxed())
}
