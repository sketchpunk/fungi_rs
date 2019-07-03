#![allow(dead_code)]
use super::{ BitSet };

use std::cell::{ Ref, RefCell, RefMut };

//##############################################################
	#[derive(Debug)]
	pub struct Entity{
		pub name		: String,			// Name given to entity
		pub com			: BitSet,			// Which Components are in use
		pub tag			: u16,
		pub is_active	: bool,				// Is the entity in an enabled state?
		pub in_use		: bool,				// Is being used or deleted?
	}

	#[derive(Debug)]
	pub struct EntityManager{
		list	: RefCell< Vec<Entity> >,
		unused	: RefCell< Vec<usize> >,
		/*
			active	: bitset,		// Entity Index
			com		: Vec<Bitset>,	// What Components are assigned to entity.
		*/
	}


//##############################################################
	impl EntityManager{
		pub fn new() -> Self{
			EntityManager{
				list	: RefCell::new( Vec::new() ),	// List of All Entities
				unused	: RefCell::new( Vec::new() ),	// List of Unused Entity Indices
			}
		}

		///////////////////////////////////////////////////////////
		//
		///////////////////////////////////////////////////////////
			pub fn mk( &self, name: &str, tag: u16 ) -> usize {
				let idx : usize;

				let mut unused	= self.unused.borrow_mut();
				let mut list	= self.list.borrow_mut();

				match unused.pop() {
					Some(i) => {
						let e = &mut list[ i ];
						e.name		= name.to_string();
						e.tag 		= tag;
						e.is_active	= true;
						e.in_use	= true;
						idx = i;
					},
					None => {
						idx = list.len();
						list.push( Entity{ 
							name		: name.to_string(), 
							tag			: tag,
							com 		: BitSet::new(), 
							is_active	: true, 
							in_use		: true,
						});
					}
				}

				idx
			}

			pub fn get_mut( &self, idx: usize ) -> RefMut< Entity >{
				RefMut::map( self.list.borrow_mut(), |el|{
					&mut el[ idx ]
				})
			}


			pub fn rm( &self, idx: usize ){
				let mut e = &mut self.list.borrow_mut()[idx];
				e.in_use = false;
				e.com.clear();

				self.unused.borrow_mut().push( idx );
			}

			pub fn on_com( &self, e_idx: usize, bit: usize ){
				let e = &mut self.list.borrow_mut()[ e_idx ];
				e.com.on( bit );
			}

			pub fn get_com_list( &self, e_idx: usize ) -> Vec<usize> {
				let b = &self.list.borrow()[ e_idx ].com;
				b.to_on_vec()
			}

			pub fn borrow( &self ) -> Ref< Vec<Entity> >{ self.list.borrow() }

		///////////////////////////////////////////////////////////
		//
		///////////////////////////////////////////////////////////			
			pub fn query_com_mask(){

			}
	}