use std::collections::HashMap;
use web_sys::{ WebGlBuffer, WebGl2RenderingContext as GL };
use super::{ glctx, Buffer };

//https://stackoverflow.com/questions/29445026/converting-number-primitives-i32-f64-etc-to-byte-representations

#[derive(Debug)]
pub struct UboSpec{
	offset		: usize,
	data_type	: DataType,
}

#[derive(Debug)]
pub struct Ubo{
	ctx			: WebGlBuffer,
	name		: String,
	bind_point	: u32,
	byte_len	: usize,
	bytes		: Vec<u8>,
	items		: HashMap<String, UboSpec>,
}

#[derive(Debug, Clone)]
pub enum DataType{
	Float,
	Int,
	Bool,
	Mat2x4,
	Mat4,
	Mat3,
	Vec2,
	Vec3,
	Vec4,
}

#[derive(Debug)]
struct Item{
	name 		: String,
	data_type 	: DataType,
	offset		: u32,
	block_size	: u32,
	data_size	: u32,
	ary_len		: u32,
}


impl Item{
	pub fn new( name: &str, data_type: DataType, ary_len: u32 ) -> Self{
		Item{ 
			name		: name.to_string(),
			offset		: 0,
			block_size	: 0,
			data_size	: 0,
			data_type, 
			ary_len,
		}
	}
}

/*
	Ubo .addItem( ubo, "bones", "mat2x4", 90 )
		.addItem( ubo, "scale", "vec3", 90 )
		.addItem( ubo, "boneCount", "int" )
		.finalize( ubo, 4 );

	static addItem(ubo, iName, iType, aryLen = 0){ 
		ubo.items.set( iName, {type:iType, offset: 0, blockSize: 0, dataSize: 0, aryLen } );
		return Ubo;
	}

	let bld = UboBuilder::new();
	bld
		.add( "name", DataType.Vec3, 0 )
		.int()

				case "float": case "int": case "b": return [4,4];
				case "mat2x4":	return [32,32]; //16*2
				case "mat4": 	return [64,64]; //16*4
				case "mat3":	return [48,48]; //16*3
				case "vec2":	return [8,8];
				case "vec3":	return [16,12]; //Special Case
				case "vec4":	return [16,16];
				default:		return [0,0];
 */

pub struct UboBuilder{
	items: Vec<Item>,
}


impl UboBuilder{
	pub fn new() -> Self {
		UboBuilder{ items: Vec::new() }
	}

	pub fn add( &mut self, name: &str, dtype: DataType, ary_len: u32 ) -> &mut Self{
		self.items.push( Item::new( name, dtype, ary_len) );
		self
	}

	pub fn build( &mut self ) -> Result< Ubo, String >{
		let ctx 		= glctx().create_buffer().ok_or_else( || "Unable to create buffer object for ubo" )?;
		let byte_len 	= self.calc() as usize;

		let mut ubo = Ubo{
			ctx,
			name 		: "test".to_string(),
			bind_point	: 0,
			byte_len	: byte_len,
			bytes 		: vec![ 0; byte_len ],
			items		: HashMap::new(),
		};

		for i in &self.items{
			ubo.items.insert( i.name.clone(), UboSpec{ 
				offset		: i.offset as usize, 
				data_type	: i.data_type.clone(),
			} );
		}

		Ok( ubo )

		/*
			// Finish Setting up UBO
			ubo.bufferSize	= Ubo.calculate( ubo.items );			// Calc all the Offsets and Lengths
			ubo.bufferID 	= gl.ctx.createBuffer();				// Create Standard Buffer
			ubo.byteBuffer	= new ByteBuffer( ubo.bufferSize );
			ubo.bindPoint	= bindPoint;

			// GPU Buffer
			gl.ctx.bindBuffer(gl.ctx.UNIFORM_BUFFER, ubo.bufferID);							// Bind it for work
			gl.ctx.bufferData(gl.ctx.UNIFORM_BUFFER, ubo.bufferSize, gl.ctx.DYNAMIC_DRAW);	// Allocate Space in empty buf
			gl.ctx.bindBuffer(gl.ctx.UNIFORM_BUFFER, null);									// Unbind
			gl.ctx.bindBufferBase(gl.ctx.UNIFORM_BUFFER, bindPoint, ubo.bufferID);			// Save Buffer to Uniform Buffer Bind point

			Cache.ubos.set( ubo.name, ubo );
			return Ubo;
		 */
	}

	fn get_size( &self, t: &DataType ) -> (u32, u32) { // ( Alignment, Size )
		match t{
			DataType::Float | DataType::Int | DataType::Bool
								=> (4,4),
			DataType::Mat2x4	=> (32,32),	// 16*2
			DataType::Mat4		=> (64,64),	// 16*4
			DataType::Mat3		=> (48,48),	// 16*3
			DataType::Vec2		=> (8,8),
			DataType::Vec3		=> (16,12),	// Special Case
			DataType::Vec4		=> (16,16),
		}
	}

	fn calc( &mut self ) -> u32 {
		let mut block_size	= 16;	// Data size in Bytes, UBO using layout std140 needs to build out the struct in blocks of 16 bytes.
		let mut offset		= 0;	// Offset in the buffer allocation
		let mut size		: (u32, u32);	// Data Size of the current type
		let mut ary_len		: u32;
		let mut itm			: &mut Item;

 		for i in 0..self.items.len(){			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// When dealing with arrays, Each element takes up 16 bytes regardless of type, but if the type
			// is a factor of 16, then that values times array length will work, in case of matrices
			ary_len	= self.items[i].ary_len;
			size 	= self.get_size( &self.items[i].data_type );
			if ary_len > 0{
				size.0 = if size.0 < 16 { ary_len * 16 }else{ size.0 * ary_len };	// Alignment
				size.1 = if size.1 < 16 { ary_len * 16 }else{ size.1 * ary_len };	// Size
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Check if there is enough block space, if not 
			// give previous item the remainder block space
			// If the block space is full and the size is equal too or greater, dont give back to previous
			if block_size >= size.0{
				block_size -= size.1;

			}else if block_size > 0 && i > 0 && !( block_size == 16 && size.1 >= 16 ){
				self.items[i-1].block_size	+= block_size;
				offset						+= block_size;
				block_size					= 16 - size.1;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Save data about the item
			itm = &mut self.items[i];
			itm.offset		= offset;
			itm.block_size	= size.1;
			itm.data_size	= size.1;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Cleanup
			if block_size <= 0 { block_size = 16; } //Reset
			offset += size.1;
 			//console_log!("{:?}", &self.items[i] );
 		}

 		//self.visualize();
 		offset
	}

	fn visualize( &mut self ){
		let mut sbuf	= String::new();
		let mut i		= 0;
		let mut tchunk	= 0;
		let mut chunk	: u32;
		let mut b_size 	= 0;

		for itm in &self.items{
			console_log!( "Item: {:?}", itm );
			chunk	= itm.block_size / 4;
			b_size	+= itm.block_size;

			for x in 0..chunk{
				if x == 0 || x == chunk-1 { 
					sbuf.push_str( &format!( "|.{}.", i ) );
				}else{
					sbuf.push_str( &"|..." );
				}

				tchunk += 1;
				if tchunk % 4 == 0 { sbuf.push_str( &"| ~ " ) }
			}
			i += 1;
		}

		if tchunk % 4 != 0{ sbuf.push_str( "|" ) };
		console_log!( "{}", sbuf );
		console_log!( "Total Chunks: {}, Byte Size: {} ", tchunk, b_size );
	}
}


impl Ubo{
	/////////////////////////////////////////////////////////////////////
	// Constructors
	/////////////////////////////////////////////////////////////////////
		/*
		pub fn new( name: &str ) -> Result< Ubo, String >{
			let ubo = Ubo{
				ctx			: glctx().create_buffer().ok_or_else( || "Unable to create buffer object" )?,
				name		: name.to_string(),
				bind_point	: 0,
				bytes		: Vec::new(),
			};
			Ok( ubo )
		}
		*/

	/////////////////////////////////////////////////////////////////////
	// 
	/////////////////////////////////////////////////////////////////////
		pub fn bind( &self ){ glctx().bind_buffer( GL::UNIFORM_BUFFER, Some( &self.ctx ) ); }
		pub fn unbind( &self ){ glctx().bind_buffer( GL::UNIFORM_BUFFER, None ); }
		pub fn delete( &self ){ glctx().delete_buffer( Some( &self.ctx ) ); }

		pub fn set_f32( &mut self, name: &str, v:f32 ) -> &mut Self{
			let offset	= self.items.get( name ).unwrap().offset;
			let ary		= v.to_bits().to_le_bytes(); // Little endian WebGL Needs it. 

			self.write( offset, &ary );
			self
		}

		pub fn write( &mut self, offset: usize, ary: &[u8] ){
			for i in 0..ary.len(){ self.bytes[ offset+i ] = ary[i]; }
		}
}


//		this.items 		= new Map();
//		this.bindPoint	= null; 		// Need this to bind UBO to shaders later on.
//		this.bufferID	= null;
//		this.bufferSize	= 0;
//		this.byteBuffer = null;
//gl.ctx.bindBuffer(gl.ctx.UNIFORM_BUFFER, (isOn)? this.bufferID : null)
//
/*

			//let ctx = match gl.create_buffer() { Some( b ) => b, None => return Err("Unable to create buffer object".to_string()) };
			let b = Buffer{
				ctx			: 
				elm_cnt		: 0,
				stride		: 0,
				offset		: 0,
				comp_len,
				buf_type,	
				data_type,
			};


		static finalize( ubo, bindPoint ){
			// Finish Setting up UBO
			ubo.bufferSize	= Ubo.calculate( ubo.items );			// Calc all the Offsets and Lengths
			ubo.bufferID 	= gl.ctx.createBuffer();				// Create Standard Buffer
			ubo.byteBuffer	= new ByteBuffer( ubo.bufferSize );
			ubo.bindPoint	= bindPoint;

			// GPU Buffer
			gl.ctx.bindBuffer(gl.ctx.UNIFORM_BUFFER, ubo.bufferID);							// Bind it for work
			gl.ctx.bufferData(gl.ctx.UNIFORM_BUFFER, ubo.bufferSize, gl.ctx.DYNAMIC_DRAW);	// Allocate Space in empty buf
			gl.ctx.bindBuffer(gl.ctx.UNIFORM_BUFFER, null);									// Unbind
			gl.ctx.bindBufferBase(gl.ctx.UNIFORM_BUFFER, bindPoint, ubo.bufferID);			// Save Buffer to Uniform Buffer Bind point

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Setup UBOs
		Ubo.build( "UBOGlobal", 0, [
			"projViewMatrix",	"mat4",
			"cameraPos",		"vec3",
			"globalTime",		"float",
			"screenSize",		"vec2"
		])	.setItem( "screenSize", [ gl.width, gl.height ] );

		//............................
		Ubo.build( "UBOModel", 1, [
			"modelMatrix",	"mat4",
			"normalMatrix",	"mat3",
		]);

		//............................
		Ubo.build( "UBOLighting", 2, [
			"lightPosition",	"vec3",  
			"lightDirection",	"vec3",
			"lightColor",		"vec3"
		])	.setItem( "lightPosition",	[  8.0,  4.0,  1.0 ] )
			.setItem( "lightDirection",	[ -8.0, -4.0, -1.0 ] )
			.setItem( "lightColor",		[  1.0,  1.0,  1.0 ] )
			.update();

		//............................
		if( (App.useArmature) & 1 == 1 ){
			let ubo = new Ubo( "UBOArmature" );
			Ubo .addItem( ubo, "bones", "mat2x4", 90 )
				.addItem( ubo, "scale", "vec3", 90 )
				.addItem( ubo, "boneCount", "int" )
				.finalize( ubo, 4 );

			ubo.setItem( "boneCount", 2 );
		}


	///////////////////////////////////////////////////////
	// STATIC SUPPORT AND DEBUG FUNCTION
	///////////////////////////////////////////////////////
		//Size of types and alignment for calculating offset positions
		static getSize( type ){ 
			switch(type){ //[Alignment,Size]
				case "float": case "int": case "b": return [4,4];
				case "mat2x4":	return [32,32]; //16*2
				case "mat4": 	return [64,64]; //16*4
				case "mat3":	return [48,48]; //16*3
				case "vec2":	return [8,8];
				case "vec3":	return [16,12]; //Special Case
				case "vec4":	return [16,16];
				default:		return [0,0];
			}
		}

		static calculate( m ){
			let blockSpace	= 16,	//Data size in Bytes, UBO using layout std140 needs to build out the struct in blocks of 16 bytes.
				offset		= 0,	//Offset in the buffer allocation
				size,				//Data Size of the current type
				prevItem	= null,
				key,itm, i;

			for( [key,itm] of m ){
				//.....................................
				// When dealing with arrays, Each element takes up 16 bytes regardless of type, but if the type
				// is a factor of 16, then that values times array length will work, in case of matrices
				size = Ubo.getSize(itm.type);
				if(itm.aryLen > 0){
					for(i=0; i < 2; i++){
						if(size[i] < 16)	size[i] = itm.aryLen * 16;
						else				size[i] *= itm.aryLen;
					}
				}

				//.....................................
				// Check if there is enough block space, if not 
				// give previous item the remainder block space
				// If the block space is full and the size is equal too or greater, dont give back to previous
				if(blockSpace >= size[0]) blockSpace -= size[1];
				else if(blockSpace > 0 && prevItem && !(blockSpace == 16 && size[1] >= 16) ){
					prevItem.blockSize += blockSpace;
					offset 		+= blockSpace;
					blockSpace	= 16 - size[1];
				}

				//.....................................
				// Save data about the item
				itm.offset		= offset;
				itm.blockSize	= size[1];
				itm.dataSize	= size[1];
				
				//.....................................
				// Cleanup
				offset			+= size[1];
				prevItem		= itm;

				if(blockSpace <= 0) blockSpace = 16; //Reset
			}

			return offset;
		}


 */