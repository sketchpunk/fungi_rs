//##################################################
#[macro_use]
pub mod macros;
pub mod web_gl;
pub mod shader;
pub mod vao;

use web_gl::WebGL;

//##################################################

use wasm_bindgen::JsCast;	// dyn_into::<>
use web_sys::{ HtmlCanvasElement, WebGl2RenderingContext as GL };

pub fn get_webgl_context( c_name: &str ) -> Result< GL, &'static str >{
	let win		= web_sys::window().expect( "Window Not Found" );
	let doc		= win.document().expect( "Window.Document Not Found" );
	let elm		= doc.get_element_by_id( c_name ).expect( &format!( "Canvas by the ID {}, Not Found", c_name ) );

	let canvas: HtmlCanvasElement = elm.dyn_into::<HtmlCanvasElement>()
		.expect( "Unable to convert html element to canvas element" );

	let ctx		= canvas.get_context( "webgl2" ) // Result< Option<>, Jsvalue >
					.expect( "Unable to get WebGL 2.0 context from canvas" )
					.unwrap() 			// For Option
					.dyn_into::< GL >()
					.expect( "Error converting WebGL context to rust." );

	Ok( ctx )
}



/*
pub fn new( c_name: &str ) -> Result< WebGL, &'static str >{
		let win		= web_sys::window().expect( "no global 'window' exists" );
		let doc		= win.document().expect( "should have a document on window" );
		//let bod		= doc.body().expect( "document should have a body" );
		let elm		= doc.get_element_by_id( c_name ).expect( &format!("Can not find canvas with ID of {}", c_name ) );

		let canvas: HtmlCanvasElement = elm.dyn_into::<HtmlCanvasElement>()
			.expect("Unable to convert html element to canvas element");

		//match elm.dyn_into::<HtmlCanvasElement>(){
		//	Ok(e)	=> canvas = e,
		//	Err(_)	=> return Err( "Unable to convert html element to canvas element" ),
		//};
		
		//let ctx = canvas.get_context( "webgl2" )?
		//		.unwrap()
		//		.dyn_into::<WebGl2RenderingContext>()?;

		let ctx = canvas.get_context( "webgl2" )	// Returns Result< Option<>, JsValue ), so need to unwrap Option
					.expect( "Unable to get WebGL2.0 Context from canvas" )
					.unwrap()												// UnWrap Option
					.dyn_into::<GL>()					// Convert Object
					.expect( "Error converting webgl context to rust." );
*/