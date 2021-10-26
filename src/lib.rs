use neon::prelude::*;
use fastcdc::*;

fn get_chunks (mut cx: FunctionContext) -> JsResult<JsArray> {
    // unbox and check arguments
    let bytes_handle = cx.argument::<JsArrayBuffer>(0)?;
    let min_size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let avg_size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let max_size = cx.argument::<JsNumber>(3)?.value(&mut cx) as usize;

    // read all of the cuts into a temporary vector
    let mut cuts = Vec::new();
    cx.borrow(&bytes_handle, |bytes_ref| {
        let bytes = bytes_ref.as_slice::<u8>();
        let chunker = FastCDC::new(&bytes, min_size, avg_size, max_size);
        for entry in chunker {
            cuts.push(entry.offset);
        }
        cuts.push(bytes.len())
    });
    
    // copy the cuts into a js array
    let result = JsArray::new(&mut cx, cuts.len() as u32);
    for (i, v) in cuts.iter().enumerate() {
        let n = cx.number(*v as f64);
        result.set(&mut cx, i as u32, n)?;
    }
    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_chunks", get_chunks)?;
    Ok(())
}
