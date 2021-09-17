use Pion::lepton::Lepton;
use Pion::lepton::AntiLepton;
use Pion::baryon::Baryon;
use Pion::baryon::AntiBaryon;

use crate::Nuclide ;

 pub const PROTONMASS   : f64 = 1.007276466621f64      ;
 pub const NEUTRONMASS  : f64 =1.00866491588f64        ;
 pub const ELECTRONMASS : f64 = 5.48579909070E-4       ;  
 pub const ALPHAMASS    : f64 = 4.001506179127f64      ;
 pub const TRITONMASS   : f64 =   3.01550098060002103  ;
 pub const DEUTERONMASS : f64 = 2.01355338927343374    ;
 
 pub const NEUTRINOMASS : f64 = 1.288738236E-10        ;

 #[derive(Debug,Clone)]
 pub enum Particle{
     Photon(f64),
     Lepton(Lepton),
     AntiLepton(AntiLepton),
     Baryon(Baryon),
     AntiBaryon(AntiBaryon),
     Alpha(f64),
     Deuteron(f64),
     Triton(f64),
     Element(Nuclide,f64),   
 }
 
 
