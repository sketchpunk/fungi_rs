#![allow(dead_code)]

use super::{ ComponentManager, EntityManager, SystemManager, ICom };

pub struct Ecs{
	pub cm: ComponentManager,
	pub em: EntityManager,
	pub sm: SystemManager,
}


impl Ecs{
	///////////////////////////////////////////////////////////////
	// Constructors
	///////////////////////////////////////////////////////////////
		pub fn new() -> Self{
			Ecs{
				cm	: ComponentManager::new(),
				em	: EntityManager::new(),
				sm	: SystemManager::new(),
			}
		}

	///////////////////////////////////////////////////////////////
	// Entities
	///////////////////////////////////////////////////////////////
		
		pub fn new_entity( &self, name: &str, tag: u16 ) -> usize{ self.em.mk( name, tag ) }
		pub fn new_entity_c1<A:ICom>( &self, name: &str, tag: u16, c0: A ) -> usize{
			let ei	= self.em.mk( name, tag );
			let bit	= self.cm.insert( ei, c0 );
			self.em.on_com( ei, bit );
			ei
		}
		pub fn new_entity_c2<A:ICom, B:ICom>( &self, name: &str, tag: u16, c0: A, c1: B ) -> usize{
			let ei		= self.em.mk( name, tag );
			let mut e	= self.em.get_mut( ei );
			e.com
				.on( self.cm.insert( ei, c0 ) )
				.on( self.cm.insert( ei, c1 ) );
			ei
		}
		pub fn new_entity_c3<A:ICom, B:ICom, C:ICom>( &self, name: &str, tag: u16, c0: A, c1: B, c2: C ) -> usize{
			let ei		= self.em.mk( name, tag );
			let mut e	= self.em.get_mut( ei );
			e.com
				.on( self.cm.insert( ei, c0 ) )
				.on( self.cm.insert( ei, c1 ) )
				.on( self.cm.insert( ei, c2 ) );
			ei
		}

		pub fn com_entity<T:ICom>( &self, e_idx: usize, com: T ){
			let c_bit = self.cm.insert( e_idx, com );
			self.em.on_com( e_idx, c_bit );
		}

		pub fn rm_entity( &self, idx: usize ){
			let vc = self.em.get_com_list( idx );
			for b in vc{ self.cm.rm_by_bit( idx, b ); }

			self.em.rm( idx );
		}

	///////////////////////////////////////////////////////////////
	// Systems
	///////////////////////////////////////////////////////////////

		pub fn run_sys( &self, ecs: &Ecs ){ self.sm.run( ecs ); }
}