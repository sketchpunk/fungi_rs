#![allow(dead_code)]

//##############################################################################
// CONSTANTS
pub const COLOR_BLACK	: Color = [ 0, 0, 0, 1 ];
pub const COLOR_WHITE	: Color = [ 255, 255, 255, 1 ];

pub const COLOR_RED		: Color = [ 255, 0, 0, 1 ];
pub const COLOR_GREEN	: Color = [ 0, 255, 0, 1 ];
pub const COLOR_BLUE	: Color = [ 0, 0, 255, 1 ];

pub const COLOR_YELLOW	: Color = [ 255, 255, 0, 1 ];
pub const COLOR_MAGENTA	: Color = [ 255, 0, 255, 1 ];
pub const COLOR_CYAN	: Color = [ 0, 255, 255, 1 ];
pub const COLOR_ORANGE	: Color = [ 255, 140, 0, 1 ];

pub const COLOR_GRAY_DARK	: Color = [ 85, 85, 85, 1 ];
pub const COLOR_GRAY_LITE	: Color = [ 200, 200, 200, 1 ];


//##############################################################################
pub type Color = [u8; 4];


//##############################################################################
pub trait ColorTrait{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn from_u32( v: u32, inc_alpha: bool ) -> Color{
			if !inc_alpha {
				[	((v >> 16) & 255) as u8,
					((v >> 8) & 255) as u8,
					(v & 255) as u8,
					1 ]
			}else{
				[	((v >> 24) & 255) as u8,
					((v >> 16) & 255) as u8,
					((v >> 8) & 255) as u8,
					(v & 255) as u8 ]
			}
		}

		//TODO, Need to handle # in string and parsing Alpha if Available
		fn from_hex_str( hex: &str ) -> Color{
	    	let mut rgb : Color = [0; 4];
	    	let mut ii  : usize;
	    	for i in 0..3{
		        ii          = i * 2;
	    	    rgb[ i ]    = u8::from_str_radix( &hex[ii..ii+2], 16).unwrap();
			}
			rgb
		}

	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		fn to_rgb_norm( &self ) -> [f32; 3];
		fn to_rgba_norm( &self ) -> [f32; 4];
}


impl ColorTrait for Color{
	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		fn to_rgb_norm( &self ) -> [f32; 3]{
			[	self[0] as f32 * 0.00392156862,		// Div by 255, inverted for mul
				self[1] as f32 * 0.00392156862,
				self[2] as f32 * 0.00392156862 ]
		}

		fn to_rgba_norm( &self ) -> [f32; 4]{
			[	self[0] as f32 * 0.00392156862,
				self[1] as f32 * 0.00392156862,
				self[2] as f32 * 0.00392156862,
				self[3] as f32 * 0.00392156862 ]
		}
}