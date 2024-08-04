/*

  Decay modes, For efficiency purposes absolutely zero checks are performance
  ZZZAAAM syntax acceptance
  Isomers


  check if electron neutrino and electron have same energy output/mass

  /*
     Correct formula for particle energies

     total_energy = daughterenergy + n*particle + n*particle2
     particle_energy = n*particle + n*particle2;

     To solve for particle mass

     particlemass*
  */
  
  DecayStruct should have an All variant
  
  
  Functions to implement
  // return NAN if DecayMode not possible
  fn branching_ratio<DecayMode> -> f64
  
  fn parent_list -> Vec<Nuclide>
  // Vector of potential daughters no energies
  fn daughter_list -> Vec<Nuclide>
  
  fn daughter_list_energetic<DecayMode> -> Vec<Vec<Particle>>
  // returns
  fn unchecked_decay<DecayMode> -> 
  
  // Attempts to decay nuclide returns None if it is not a supported branch
  fn single_decay<DecayMode> -> Option<Nuclide>
  
  // Continously applies the same decay mode
  fn continous_decay<DecayMode> -> 
  
  // Returns Nan for All or if separation is not possible
  fn separation_energy<DecayMode> -> f64
  
  // Halflife for the decay branch, inf if stable and NaN if decaymode is not supported
  fn half_life<DecayMode> -> f64
  
    // Decay constant for the decay branch, inf if stable and NaN if decaymode is not supported
  fn decay_constant<DecayMode> -> f64
  // Replaces decay_time, returns the probability of decay for given branch. Zero if stable or branch not supported
  fn decay_probability<DecayMode>(T)
  
  
  Constructing a parent list 
  
    Enumerate all decay possibilities
    Change the nuclide to find the parent, then check possible decay modes that lead to the daughter
    Fission is not counted

*/
use crate::nuclidedata::nuclidestruct::Nuclide;
use crate::nuclidedata::decaymodes::*;
//use crate::atom::Atom;
use crate::nuclidedata::decay_chain::DECAY_CHAIN;
use crate::Particle;

// Given an u64 representation of the decay modes it returns the corresponding u64 for 
pub(crate) fn decayindex<T: DecayMode>(idx: usize, decay_rep: u64) -> Option<u64>{
      let modes = DECAY_CHAIN[idx].to_be_bytes();
      let d_mode = T::decay_index() as u8;
      
      if modes[0] == d_mode{
        return Some(DECAY_CHAIN[idx-5])
      }
      if modes[1] == d_mode{
         return Some(DECAY_CHAIN[idx-4])
      }
      if modes[2] == d_mode{
      return Some(DECAY_CHAIN[idx-3])
      }
      if modes[3] == d_mode{
      return Some(DECAY_CHAIN[idx-2])
      }
      if modes[4] == d_mode{
       return Some(DECAY_CHAIN[idx-1])
      }
      return None
}


pub(crate) mod private{
//use crate::Nuclide;
//use crate::Particle;
use crate::*;
use crate::nuclidedata::decaymodes::*;
pub trait InternalDecay {
   fn decay(x: &mut Nuclide) -> (f64,Vec<Particle>);
   // Decay_index return u32::MAX if not supported by system 254 for all decay modes
   fn decay_index() -> u32;
  // fn unchecked_decay_result(x: &Nuclide) -> Nuclide;
  // fn decay_result(x: &Nuclide) -> Option<Nuclide>;
}

impl<const K: usize> InternalDecay for ProtonEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     proton_emission(x,K)
   }
   
   fn decay_index() -> u32{
     if K == 1{
      return 2
     }
     
     if K == 2{
      return 3
     }
     
     return u32::MAX
   }
   
   

}

impl<const K: usize> InternalDecay for NeutronEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     neutron_emission(x,K)
   }
   
   fn decay_index() -> u32{
     if K == 1{
      return 4
     }
     if K == 2{
      return 5
     }
     return u32::MAX
   }
}

impl InternalDecay for NeutronDeuteron{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     neutron_deuteron(x)
   }
    // FIXME
     fn decay_index() -> u32{
       return u32::MAX
     }
}

impl InternalDecay for NeutronTriton{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     neutron_triton(x)
   }
   // FIXME
  fn decay_index() -> u32{
      return u32::MAX
  }
}

impl<const K: usize> InternalDecay for AlphaEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     alpha_emission(x,K)
   }
   
  fn decay_index() -> u32{
    if K == 1{
      return 1
    }
    return u32::MAX  
  }
}

impl<const K: usize> InternalDecay for AlphaNeutron<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     alpha_neutron(x,K)
   }
   // FIXME
  fn decay_index() -> u32{
      return u32::MAX
  }   
}

impl<const K: usize> InternalDecay for ElectronCapture<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_capture(x,K)
   }

  fn decay_index() -> u32{
      if K == 1{
       return 6
      }
      if K == 2{
       return 7
      }
      return u32::MAX
  }   
}

impl<const K: usize> InternalDecay for ElectronEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_emission(x,K)
   }
   
  fn decay_index() -> u32{
      if K == 1{
       return 11
      }
      if K == 2{
       return 12
      }
      return u32::MAX
  }
     
}

impl<const K: usize> InternalDecay for ElectronNeutron<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_neutron(x,K)
   }

  fn decay_index() -> u32{
      if K == 1{
        return 13
      }
      if K == 2{
        return 14
      }
      if K == 3{
       return 15
      }
      return u32::MAX
  }
     
}

impl<const K: usize> InternalDecay for ElectronProton<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_proton(x,K)
   }
   // FIXME
  fn decay_index() -> u32{
      return u32::MAX
  }
     
}

impl InternalDecay for ElectronAlpha{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_alpha(x)
   }
   
  fn decay_index() -> u32{
      return 19
  }   
}

impl InternalDecay for ElectronDeuteron{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_deuteron(x)
   }
   
  fn decay_index() -> u32{
      return 21
  }   
}

impl InternalDecay for ElectronTriton{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_triton(x)
   }
   
  fn decay_index() -> u32{
      return 22
  }   
}

impl InternalDecay for ElectronFission{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     electron_fission(x)
   }
   
  fn decay_index() -> u32{
      return 24
  }   
}

impl<const K: usize> InternalDecay for PositronEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     positron_emission(x,K)
   }
   
  fn decay_index() -> u32{
      if K == 1{
       return 9
      }
      if K == 2{
       return 10
      }
      return u32::MAX
  }   
}

impl<const K: usize> InternalDecay for PositronProton<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     positron_proton(x,K)
   }

  fn decay_index() -> u32{
      if K == 1{
        return 16
      }
      if K == 1{
        return 17
      }
      if K == 1{
        return 18
      }
      return u32::MAX
  }   
}

impl InternalDecay for PositronAlpha{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     positron_alpha(x)
   }
   
  fn decay_index() -> u32{
      return 20
  }   
}

impl InternalDecay for PositronFission{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     positron_fission(x)
   }
   
  fn decay_index() -> u32{
      return 25
  }   
}

impl InternalDecay for SpontaneousFission{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     spontaneous_fission(x)
   }
   
  fn decay_index() -> u32{
      return 23
  }   
}

impl<const K: usize> InternalDecay for ClusterDecay<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     cluster_decay(x,&Nuclide::assign(K))
   }
   
  fn decay_index() -> u32{
      if K == 60{
        return 26
      }
      if K == 128{
       return 27
      }
      if K == 132{
       return 28
      }
      if K == 224{
       return 30
      }
      if K == 226{
       return 31
      }
      return u32::MAX
  }   
}

impl<const D: usize, const S: usize> InternalDecay for DoubleCluster<D,S>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     double_cluster(x,&Nuclide::assign(D), &Nuclide::assign(S))
   }
   
  fn decay_index() -> u32{
      if D == 132 && S == 134{
        return 29
      }
      if D == 134 && S == 132{
       return 29
      }
      return u32::MAX
  }   
} // end trait

} // end pub mod

pub trait DecayMode : private::InternalDecay{}

impl<const K: usize> DecayMode for ProtonEmission<K>{}
impl<const K: usize> DecayMode for NeutronEmission<K>{}
impl<const K: usize> DecayMode for AlphaEmission<K>{}
impl<const K: usize> DecayMode for AlphaNeutron<K>{}
impl DecayMode for NeutronDeuteron{}
impl DecayMode for NeutronTriton{}
impl<const K: usize> DecayMode for ElectronCapture<K>{}
impl<const K: usize> DecayMode for ElectronEmission<K>{}
impl<const K: usize> DecayMode for ElectronNeutron<K>{}
impl<const K: usize> DecayMode for ElectronProton<K>{}
impl DecayMode for ElectronAlpha{}
impl DecayMode for ElectronDeuteron{}
impl DecayMode for ElectronTriton{}
impl DecayMode for ElectronFission{}
impl<const K: usize> DecayMode for PositronEmission<K>{}
impl<const K: usize> DecayMode for PositronProton<K>{}
impl DecayMode for PositronAlpha{}
impl DecayMode for PositronFission{}
impl DecayMode for SpontaneousFission{}
impl<const K: usize> DecayMode for ClusterDecay<K>{}
impl<const D: usize, const S: usize> DecayMode for DoubleCluster<D,S>{}
