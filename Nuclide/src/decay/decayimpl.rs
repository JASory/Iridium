use crate::decay::internal::InternalDecay;
use crate::decay::types::*;
use crate::nuclidedata::decay_chain::DECAY_CHAIN;

pub trait DecayMode : InternalDecay{}

impl DecayMode for TotalDecay{}
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


// Given an u64 representation of the decay modes it returns the corresponding u64 for 
pub(crate) fn decayindex<T: DecayMode>(idx: usize) -> Option<u64>{
      let modes = DECAY_CHAIN[idx].to_be_bytes();
      let d_mode = T::decay_index();
      if d_mode == 254{
        return Some(u64::MAX)
      }
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
      None
}
