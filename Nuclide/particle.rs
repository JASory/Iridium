

 use crate::Nuclide ;
 pub const PROTONMASS   : f64 = 1.007276466621f64      ;
 pub const NEUTRONMASS  : f64 =1.00866491588f64        ;
 pub const ELECTRONMASS : f64 = 5.48579909070E-4       ;  
 pub const ALPHAMASS    : f64 = 4.001506179127f64      ;
 pub const TRITONMASS   : f64 =   3.01550098060002103  ;
 pub const DEUTERONMASS : f64 = 2.01355338927343374    ;
 
 pub const NEUTRINOMASS : f64 = 1.288738236E-10        ;

// Replace with Pion library
#[derive(Debug,Clone)]
pub enum Particle {
      Photon(f64),  //Frequency/Energy
      Proton(f64),
      AntiProton(f64),
      Neutron(f64),
      AntiNeutron(f64),
      Electron(f64),
      Positron(f64),
      Neutrino(f64),
      AntiNeutrino(f64),
      Alpha(f64),
      Deuteron(f64),
      Triton(f64),
      Element(Nuclide,f64),
 }
 
 
