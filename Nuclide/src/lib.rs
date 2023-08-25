/*!

  Nuclide is a simple modeling and data library for nuclear isotopes (and eventually isomers and/or ions). It is meant to be a midpoint between
  basic periodic table libraries and advanced research software like GEANT4.

  Notable capabilities of this library are listed below.

  - Data on 3585 nuclides, including : Atomic mass, half-life (total and partial),  decay mode
  - Decay modeling for all unstable nuclides with total energies and particles emitted
  - Approximation of the mass and binding energy of theorectical nuclides
  - Elemental data including : ionization levels, various atomic radii and electronegativity measures
  - Hardcoded data. Unlike more advanced libraries, all data is hardcoded or computed resulting in a condensed standalone library. It is believed by the author to be the largest standalone nuclide library in any language

  Some features that are absent by design

  - Data on macroscopic properties, like melting point or abundance, are beyond the scope of this library which intends to model individual atoms.
  - Error bounds. Introduce levels of complexity that are beyond the intention of the library. For accurate proper modeling look for more
   specialized libraries.
  - Data on discovery and other metainformation. Not considered useful in application.
  
 

*/
#![allow(non_snake_case)]

mod atom;
mod isomer;
pub(crate) mod nuclidedata;
pub mod decay;
mod particle;
mod rng;
mod mmodel;
pub(crate) mod constant;

pub use crate::atom::Atom;
pub use crate::isomer::Isomer;
pub use crate::nuclidedata::nuclidestruct::Nuclide;
//pub use crate::Nuclide;
pub use crate::particle::Particle;
//pub use crate::types::*;
//pub(crate) use crate::decay::*;

// decay chain 
// adjacency_matrix
// daughter isotopes
// parent isotopes
// Decay types vector   
// partial_half_life
// partial_decay_constant
// decay_types 
