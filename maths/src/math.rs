#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod Math{
	//use super::super::Vec3;

	////////////////////////////////////////////////////////////////////
	// CONSTANTS
	////////////////////////////////////////////////////////////////////
		pub const PI		:f32 = 3.14159265359;
		pub const PI_H		:f32 = 1.5707963267948966;
		pub const PI_H_MIN	:f32 = 1.57079630934166; 	// 89.999999
		pub const PI_2		:f32 = 6.283185307179586;
		pub const PI_2_INV	:f32 = 0.159154943096;
		pub const RAD_45	:f32 = 0.7853981633974483;
		pub const RAD_270	:f32 = 4.71238898038;
		pub const DEG2RAD	:f32 = 0.01745329251; 		// PI / 180
		pub const RAD2DEG	:f32 = 57.2957795131; 		// 180 / PI
		pub const EPSILON	:f32 = 0.000001;


	////////////////////////////////////////////////////////////////////
	// 
	////////////////////////////////////////////////////////////////////
		pub fn to_rad( v: f32 ) -> f32	{ v * DEG2RAD }
		pub fn to_deg( v: f32 ) -> f32	{ v * RAD2DEG }
		pub fn dot_rad( d: f32 ) -> f32	{ d.min( 1.0 ).max( -1.0 ).acos() } // acos(clamp(d,-1,1))

		pub fn map( x: f32, xMin: f32, xMax: f32, zMin: f32, zMax: f32) -> f32 { ( x - xMin ) / ( xMax - xMin ) * ( zMax - zMin ) + zMin }
		pub fn norm( v: f32, min: f32, max: f32 ) -> f32 { (v - min) / (max - min) }
		pub fn clamp( v:f32, min:f32, max:f32 ) -> f32 { v.min( max ).max( min ) }

		pub fn lerp( a: f32, b: f32, t: f32) -> f32 { (1.0 - t) * a + t * b }  //return a + t * (b-a);
		pub fn fract( f: f32 ) -> f32 { f - f.floor() }

		pub fn step( edge: f32, x: f32 ) -> f32 { if x < edge { 0.0 } else { 1.0 } }
		pub fn smooth_t_step( t: f32 ) -> f32 { t * t * (3.0 - 2.0 * t) }
		pub fn smooth_step( edge1: f32, edge2: f32, val: f32 ) -> f32 { 	//https://en.wikipedia.org/wiki/Smoothstep
			let x = (( val - edge1 ) / ( edge2 - edge1 )).min( 1.0 ).max( 0.0 );
			x * x * ( 3.0 - 2.0 * x ) 
		}

		//Loops between 0 and Len, once over len, starts over again at 0, like a sawtooth wave
		pub fn repeat( t: f32, len: f32 ) -> f32 { ( t - (t / len).floor() * len).max( 0.0 ).min( len ) }

		// Loops back and forth between 0 and len, it functions like a triangle wave.
		pub fn ping_pong( t: f32, len: f32 ) -> f32 { len - (repeat( t, len * 2.0 ) - len).abs() }

		pub fn grad010( t: f32 ) -> f32 {
			let tt = t * 2.0;
			if tt > 1.0 { 1.0 - ( tt - 1.0 ) } else { tt }
		}


	////////////////////////////////////////////////////////////////////
	// TRIG
	////////////////////////////////////////////////////////////////////
		// SSS : Solve only knowing sides Side-Side-Side
		// https://www.mathsisfun.com/algebra/trig-solving-sss-triangles.html
		pub fn lawcos_sss( aLen: f32, bLen: f32, cLen: f32 ) -> f32 {
			// Law of Cosines - SSS : cos(C) = (a^2 + b^2 - c^2) / 2ab
			// The Angle between A and B with C being the opposite length of the angle.
			((aLen*aLen + bLen*bLen - cLen*cLen) / (2.0 * aLen * bLen)).acos()
		}


	////////////////////////////////////////////////////////////////////
	// POLAR
	////////////////////////////////////////////////////////////////////
	
		pub fn polar_to_cartesian( lon: f32, lat: f32, radius: f32 ) -> [f32;3]{
			let phi		= PI_H - lat; // 90 Degrees in Rad - Lat
			let theta	= lon + PI;
			let s_phi	= phi.sin();
			[ 	-( radius * s_phi * theta.sin() ),
				radius * phi.cos(),
				radius * s_phi * theta.cos() ]
		}
	
		pub fn cartesian_to_polar( v: &[f32;3] ) -> [f32;2]{
			let lon = v[0].atan2( -v[2] );
			let len = ( v[0] * v[0] + v[2] * v[2] ).sqrt();
			let lat = v[1].atan2( len );
			[ lon, lat ]
		}


		// X and Y axis need to be normalized vectors, 90 degrees of eachother.
		/*
		pub fn planeCircle( center : &Vec3, xAxis : &Vec3, yAxis : &Vec3, angle :f32, radius :f32, out : &mut Vec3 ){
			let sin = angle.sin();
			let cos = angle.cos();
			out.x = center.x + radius * cos * xAxis.x + radius * sin * yAxis.x;
			out.y = center.y + radius * cos * xAxis.y + radius * sin * yAxis.y;
			out.z = center.z + radius * cos * xAxis.z + radius * sin * yAxis.z;
		}

		//X and Y axis need to be normalized vectors, 90 degrees of eachother.
		pub fn planeEllipse( center: &[f32;3], xAxis: &[f32;3], yAxis: &[f32;3], angle :f32, xRadius :f32, yRadius:f32, out : &mut [f32;3] ){
			let sin = angle.sin();
			let cos = angle.cos();
			out[0] = center[0] + xRadius * cos * xAxis[0] + yRadius * sin * yAxis[0];
			out[1] = center[1] + xRadius * cos * xAxis[1] + yRadius * sin * yAxis[1];
			out[2] = center[2] + xRadius * cos * xAxis[2] + yRadius * sin * yAxis[2];
		}
		*/
}