 use crate::decay::*;
 use crate::decay::internal::InternalDecay;
 use crate::nuclidedata::decay_chain::DECAY_CHAIN;
 use crate::rng::rand;
 use crate::{Nuclide,Particle};
 use crate::traits::ChemElement;
 use crate::constant::*;
 use crate::mmodel::mass_model;
 
 /*

  Given a Nuclide, this function computes a random decay mode and the index point of 
  the rng bound.  

    In: Nuclide
    Out: Randomly selected decay, and idx point for the decay coefficient
 */
 pub(crate) fn decay_mode_idx(input: &Nuclide) -> (u8,u8){    
            let mut x = 255;
            let mut ridx = 0u8;
            let idx = input.nuclide_index() * 6;
            let unu = DECAY_CHAIN[idx];
            let doua = DECAY_CHAIN[idx + 1] + unu;
            let trei = DECAY_CHAIN[idx + 2] + doua;
            let patru = DECAY_CHAIN[idx + 3] + trei;
            let cinci = DECAY_CHAIN[idx + 4] + patru;
            let alea = rand();
            let decay_vector = DECAY_CHAIN[input.nuclide_index() * 6 + 5].to_be_bytes();

            if alea > 0 && alea < unu {
                x = decay_vector[0];
                ridx = 0;
            } else if alea > unu && alea < doua {
                x = decay_vector[1];
                ridx = 1;
            } else if alea > doua && alea < trei {
                x = decay_vector[2];
                ridx = 2;
            } else if alea > trei && alea < patru {
                x = decay_vector[3];
                ridx = 3;
            } else if alea > patru && alea < cinci {
                x = decay_vector[4];
                ridx = 4;
            }
            (x,ridx)
            }
            
         /*
             Takes a mode from the above function and then computes the decay with energy and Vec<Particle>
         */   
   pub(crate)  fn decay_select(input : &mut Nuclide, mode: u8) -> (f64,Vec<Particle>){
               match mode {
                1 => AlphaEmission::<1>::decay(input),
                2 => ProtonEmission::<1>::decay(input),
                3 => ProtonEmission::<2>::decay(input),
                4 => NeutronEmission::<1>::decay(input),

                5 => NeutronEmission::<2>::decay(input),
                6 => ElectronCapture::<1>::decay(input),
                7 => ElectronCapture::<2>::decay(input),

                8 => ElectronProton::<1>::decay(input),
                9 => PositronEmission::<1>::decay(input),
                10 => PositronEmission::<2>::decay(input),
                11 => ElectronEmission::<1>::decay(input),
                12 => ElectronEmission::<2>::decay(input),
                13 => ElectronNeutron::<1>::decay(input),
                14 => ElectronNeutron::<2>::decay(input),
                15 => ElectronNeutron::<3>::decay(input),
                16 => PositronProton::<1>::decay(input),
                17 => PositronProton::<2>::decay(input),
                18 => PositronProton::<3>::decay(input),
                19 => ElectronAlpha::decay(input),
                20 => PositronAlpha::decay(input),
                21 => ElectronDeuteron::decay(input),
                22 => ElectronTriton::decay(input),
                23 => SpontaneousFission::decay(input),
                24 => ElectronFission::decay(input),
                25 => PositronFission::decay(input),
                26 => ClusterDecay::<61>::decay(input), // C-14 
                27 => ClusterDecay::<129>::decay(input), // Ne-20
                28 => ClusterDecay::<133>::decay(input), // Ne-24

                29 => DoubleCluster::<133,135>::decay(input), // Ne-24 + Ne-26  
                
                30 => ClusterDecay::<223>::decay(input), //Si-32 emission
                31 => ClusterDecay::<225>::decay(input), //Si-34 emission
                32 => ClusterDecay::<176>::decay(input),  // Mg-28 emission
                33 => ClusterDecay::<115>::decay(input),  // F-23 emission
                34 => ClusterDecay::<94>::decay(input),  // O-18 emission
                35 => ClusterDecay::<96>::decay(input),  // O-20 emission
                36 => ClusterDecay::<59>::decay(input),  // C-12 emission
                37 => ClusterDecay::<131>::decay(input),  // Ne-22 emission
                38 => ClusterDecay::<178>::decay(input), // Mg-30 emission
                
                39 => DoubleCluster::<175,178>::decay(input), // Mg-28 + Mg-30 emission
                
		40 => NeutronDeuteron::decay(input),
		41 => NeutronTriton::decay(input),
		42 => AlphaNeutron::<1>::decay(input),
		43 => AlphaNeutron::<2>::decay(input), 
		44 => AlphaNeutron::<3>::decay(input),
		45 => ElectronProton::<1>::decay(input),
		46 => ElectronProton::<2>::decay(input),  
		47 => ElectronProton::<3>::decay(input),  	
		48 => ProtonEmission::<3>::decay(input),
		49 => ClusterDecay::<135>::decay(input), // Ne-26	
		50 => ElectronNeutron::<4>::decay(input), // B- 4n
                _ => (0f64, Vec::<Particle>::new()), // if no decay then return empty data
            } // end match
 }   
 
 // Determines if the DecayMode is recorded 
 pub(crate)  fn is_mode_recorded<T: InternalDecay>(input: &Nuclide) -> bool{
 
               let val = DECAY_CHAIN[input.nuclide_index() * 6 + 5].to_be_bytes();
               let search_val =  T::decay_index();
               
               if val[0] == search_val{
                 return true
               }
               if val[1] == search_val{
                 return true
               }
               if val[2] == search_val{
                 return true
               }
               if val[3] == search_val{
                 return true
               }
               if val[4] == search_val{
                 return true
               }
               false
 }
 
 pub(crate) fn max(x: Nuclide, y: Nuclide) -> (Nuclide, Nuclide) {
    if x.am() > y.am() {
        return (x, y);
    }
    (y, x)
}

pub (crate) fn theoretical_mass(p: usize, n: usize) -> f64{
      ((p as f64)*PROTONMASS + (n as f64)*NEUTRONMASS)-(mass_model(p+n,p)*MeV_DALTON)
   }
   
pub (crate) fn elementary_mass(p: usize, n: usize, e: usize, neu: usize) -> f64{
      (p as f64)*PROTONMASS + (n as f64)*NEUTRONMASS + (e as f64)*ELECTRONMASS + (neu as f64)*NEUTRINOMASS
   }   
   
