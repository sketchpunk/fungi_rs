#![allow(dead_code)]


pub fn quad_verts() -> Vec<f32>{
	[
		-0.5, 0.5, 0.0,
		-0.5, -0.5, 0.0,
		0.5, -0.5, 0.0,

		0.5, -0.5, 0.0,
		0.5, 0.5, 0.0,
		-0.5, 0.5, 0.0,
	].to_vec()
}


pub fn grid_floor_verts() -> Vec<f32>{
	let grid_size	= 0.2;							// Distance between lines
	let len			= 70;							// How may lines to make
	let vsize 		= ( len * 4 + 2 ) * 2 * 4;		// Total lines, 2 Points Each, 4 floats per point
	let t 			= len as f32 * grid_size;		// Total size of the grid
	let mut v		= vec![ 0.0; vsize ]; //Vec::with_capacity( vsize );	// Vertex Array
	let mut p;										// Position

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	let mut ii = 0;
	for i in 1..=len{
		p = i as f32 * grid_size;
		//v.push(	p,0,t,0, p,0,-t,0,
		//		-p,0,t,0, -p,0,-t,0,
		//		-t,0,p,0, t,0,p,0,
		//		-t,0,-p,0, t,0,-p,0 )

		v[ii]		= p;
		v[ii+1]		= 0.0;
		v[ii+2]		= t;
		v[ii+3]		= 0.0;
		v[ii+4]		= p;
		v[ii+5]		= 0.0;
		v[ii+6]		= -t;
		v[ii+7]		= 0.0;

		v[ii+8]		= -p;
		v[ii+9]		= 0.0;
		v[ii+10]	= t;
		v[ii+11]	= 0.0;
		v[ii+12]	= -p;
		v[ii+13]	= 0.0;
		v[ii+14]	= -t;
		v[ii+15]	= 0.0;

		v[ii+16]	= -t;
		v[ii+17]	= 0.0;
		v[ii+18]	= p;
		v[ii+19]	= 0.0;
		v[ii+20]	= t;
		v[ii+21]	= 0.0;
		v[ii+22]	= p;
		v[ii+23]	= 0.0;

		v[ii+24]	= -t;
		v[ii+25]	= 0.0;
		v[ii+26]	= -p;
		v[ii+27]	= 0.0;
		v[ii+28]	= t;
		v[ii+29]	= 0.0;
		v[ii+30]	= -p;
		v[ii+31]	= 0.0;

		ii += 32;
	}
	
	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	v[ii]		= -t;
	v[ii+1]		= 0.007;
	v[ii+2]		= 0.0;
	v[ii+3]		= 1.0;
	v[ii+4]		= t;
	v[ii+5]		= 0.007;
	v[ii+6]		= 0.0;
	v[ii+7]		= 1.0;
	
	v[ii+8]		= 0.0;
	v[ii+9]		= 0.007;
	v[ii+10]	= t;
	v[ii+11]	= 2.0;
	v[ii+12]	= 0.0;
	v[ii+13]	= 0.007;
	v[ii+14]	= -t;
	v[ii+15]	= 2.0;

	v
}