use js_sys::WebAssembly;
use wasm_bindgen::JsCast;	// dyn_into::<>


pub struct Util{}


impl Util{
	pub fn vec_to_f32_array( v: &Vec<f32> )-> js_sys::Float32Array{
		let mem_buf	= wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
	   	let loc		= v.as_ptr() as u32;

		js_sys::Float32Array::new_with_byte_offset_and_length( &mem_buf , loc, v.len() as u32 )
	}
}