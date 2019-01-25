#[macro_use]
extern crate neon;
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

extern crate libc;

use neon::prelude::*;

pub mod platform;
pub mod data;

pub use self::platform::Platform;
pub use self::platform::PlatformImpl as System;
pub use self::data::*;

fn get_drives(mut cx: FunctionContext) -> JsResult<JsArray> {
    let sys = System::new();
    let drives = sys.drives().unwrap();

    // Create the JS array
    let js_array = JsArray::new(&mut cx, drives.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in drives.iter().enumerate() {
        let drive_object = neon_serde::to_value(&mut cx, &obj)?;
        js_array.set(&mut cx, i as u32, drive_object).unwrap();
    }

    Ok(js_array)
}

register_module!(mut cx, {
    cx.export_function("getDrives", get_drives)
});
