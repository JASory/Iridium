/*
mod atomic_mass;
mod decay;
mod decay_chain;
mod elemental;
mod electro;
mod index;
mod nuclide;
mod radii;
mod update;
mod half_life;

use electro::*;
use nuclide::*;
use nuclide::Nuclide::new;
use radii::*;
*/

//!  This crate is experimental and will have many breaks. For physics research projects use GEANT4. Otherwise the data here will meet or exceed that of most other libraries
            pub mod nuclide;
            
                mod electro;
                mod radii;
                mod index    ;
                mod update   ;
                mod half_life;
                mod decay    ;
                mod decay_chain;
                mod atomic_mass;
                mod elemental  ;
                use nuclide::Nuclide;





