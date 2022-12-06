use day06::find_start;
use neon::prelude::*;

fn _find_start(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let buffer = cx.argument::<JsString>(0)?;
    let window_size = cx.argument::<JsNumber>(1)?;
    let start = find_start(&buffer.value(&mut cx), window_size.value(&mut cx) as usize);
    Ok(cx.number(start as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("findStart", _find_start)?;
    Ok(())
}
