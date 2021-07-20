/*
const PROTON_DECAY_MASK     : u16 = 8192 ; // 1
const NEUTRON_DECAY_MASK    : u16 = 16384; // 2
const BETA_DECAY_MASK       : u16 = 24576; // 3 24576
const POSITRON_DECAY_MASK   : u16 = 32768; // 4 32768
const ELECTRON_CAPTURE_MASK : u16 = 40960; // 5    Particle 5 == Electron Neutrino
const ALPHA_DECAY_MASK      : u16 = 49152; // 6
const CLUSTER_DECAY_MASK    : u16 = 57344; // 7  (7,14) is a particle of C-14
*/
use crate::update::PROTON_DECAY_MASK     ;
use crate::update::NEUTRON_DECAY_MASK    ;     
use crate::update::ALPHA_DECAY_MASK      ;    
use crate::update::BETA_DECAY_MASK       ;
use crate::update::POSITRON_DECAY_MASK   ;
use crate::update::ELECTRON_CAPTURE_MASK ;
use crate::update::CLUSTER_DECAY_MASK    ;

use crate::decay_chain::DECAY_CHAIN      ;
use crate::half_life::HALF_LIFE          ;

 use crate::nuclide::nucleons_nuclide    ;
 use crate::nuclide::Nuclide;




pub fn rand()->u64{
   let mut x: u64 = 0;
  let k = unsafe{core::arch::x86_64::_rdrand64_step(&mut x)};
x
}


//Type, quantity, Replace with Energy for future values
#[derive(Debug)]
pub struct Particle(u16, usize);      



impl Clone for Particle{
    fn clone(&self)->Particle{
    Particle(self.0.clone(), self.1.clone())
    }
}




//DECAY_VECTOR = (DECAY CHAIN PROBABILITY)
// returns joules
impl Nuclide{

       ///Half-life in seconds
pub  fn half_life(&self)->f64{
          HALF_LIFE[self.nuclide_index()]
       }
 
  pub fn mean_lifetime(&self)->f64{   //reciprocal of ln(2) average lifespan of a particle
         self.half_life()*1.4426950408889634f64
      }
     
     ///Approximation of decay constant
  pub  fn decay_constant(&self)->f64{
          self.mean_lifetime().recip()
       }
 
 //lowest probability is 1/u64::MAX
    ///Returns true if the nuclide would have decayed in the time given. The nuclide remains unchanged
  pub  fn decay_time(&self, time: f64)->bool{
          let  prob =((1.0- (-self.decay_constant()*time).exp()) * 1.8446744073709551616E+19) as u64;

           if prob > rand(){
              return true 
           }
           else{
              return false
           }
       }



///Returns the total energy and a vector of particles. Avoid reliance, because this function will be changed to incorporate energy levels into the particles produced.
 pub fn decay(&mut self, time: f64)->(f64,Vec<Particle>){
 
  let mut particle_vector = vec![] ;
  let primul_nuclide = self.clone();
  let mut izotop = self.proton_neutron()  ;
  
          
   if self.decay_time(time) {
     let idx    = self.nuclide_index()*10      ; 
     let unu    = DECAY_CHAIN[idx]             ;
     let doua   = DECAY_CHAIN[idx + 1] + unu   ;
     let trei   = DECAY_CHAIN[idx + 2] + doua  ;
     let patru  = DECAY_CHAIN[idx + 3] + trei  ;
     let cinci  = DECAY_CHAIN[idx + 4] + patru ; 
     let alea   = rand()                       ;
   
   if alea > 0 && alea < unu {
      curry_decay(&mut izotop, &mut 0,DECAY_CHAIN[idx +5],&mut particle_vector);
   }
   
   else if alea > unu && alea < doua{
         curry_decay(&mut izotop,&mut 0,DECAY_CHAIN[idx +6],&mut particle_vector);
         
   }
   
   else if alea > doua && alea < trei {
         curry_decay(&mut izotop,&mut 0,DECAY_CHAIN[idx +7],&mut particle_vector);
         
   }
   
   else if alea > trei  && alea < patru{
         curry_decay(&mut izotop,&mut 0,DECAY_CHAIN[idx +8],&mut particle_vector);
         
   }
   
   else if alea > patru && alea < cinci { 
         curry_decay(&mut izotop,&mut 0,DECAY_CHAIN[idx +9],&mut particle_vector);
         
   }
   
   
   
   }//end if
   self.change(crate::nuclide::nucleons_nuclide(&izotop));
 ((primul_nuclide.amu() - self.amu()) * 8.98755178780128E+16, particle_vector)
 }

}

fn curry_decay(nuclide:&mut (usize, usize), step: &mut usize, instruction: u64 ,  decay_vec: &mut Vec<Particle>)->((usize,usize), usize, Vec<Particle>){
 if instruction ==0{
    return(*nuclide, 0usize, decay_vec.to_vec())
 }
 else{
 let mut decay_vector = unsafe{ std::mem::transmute::<u64,[u16;4]>(instruction)};
 
 if decay_vector[*step]>>13 == 1u16{
    let proton_count = (PROTON_DECAY_MASK^decay_vector[*step]) as usize; 
    decay_vec.push(Particle(1,proton_count));
    *nuclide = (nuclide.0 -proton_count, nuclide.1);
 }
 
 else if decay_vector[*step]>>13== 2u16{
  let neutron_count = (NEUTRON_DECAY_MASK^decay_vector[*step]) as usize;
    decay_vec.push(Particle(2,neutron_count));
    *nuclide = (nuclide.0, nuclide.1-neutron_count);
 }
 
 else if decay_vector[*step]>>13==3u16{
   let electron_count = (BETA_DECAY_MASK^decay_vector[*step]) as usize;
    decay_vec.push(Particle(3,electron_count));
    *nuclide = (nuclide.0 + electron_count, nuclide.1-electron_count);

 }
 
 else if decay_vector[*step]>>13==4u16{
    let positron_count = (POSITRON_DECAY_MASK^decay_vector[*step]) as usize;
    decay_vec.push(Particle(4,positron_count));
    *nuclide = (nuclide.0-positron_count, nuclide.1+positron_count)
 }
 
 else if decay_vector[*step]>>13==5u16{
    let electrons_captured = (ELECTRON_CAPTURE_MASK^decay_vector[*step]) as usize;
    decay_vec.push(Particle(5,electrons_captured));
    *nuclide = (nuclide.0-electrons_captured , nuclide.1+electrons_captured)
 }
 
 else if decay_vector[*step]>>13==6u16{
    let alphas = (ALPHA_DECAY_MASK^decay_vector[*step]) as usize;
    decay_vec.push(Particle(6,alphas));
    *nuclide = (nuclide.0 -alphas*2 , nuclide.1 -alphas*2)
 }
 
 else if decay_vector[*step]>>13==7u16{   
    let daughter = (CLUSTER_DECAY_MASK^decay_vector[*step]) as usize;
    if daughter == 8191 {                                   // rand fission
       let daughter_rand =  (rand()>>54) as usize;  //ensures randomness
       decay_vec.push(Particle(7,daughter_rand));
       let daughter_rand_iso = Nuclide::assign(daughter_rand).isotope();
       *nuclide = (nuclide.0-daughter_rand_iso.0, nuclide.1-daughter_rand_iso.1)
    }
    else{
    decay_vec.push(Particle(7,daughter));
    let daughter_iso = Nuclide::assign(daughter).isotope();
    *nuclide = (nuclide.0 - daughter_iso.0, nuclide.1 - daughter_iso.1)
    }
 }

  decay_vector[*step] = 0u16;
   *step+=1;
  
  let instruction2 = unsafe{std::mem::transmute::<[u16;4], u64>(decay_vector)};
 
  return curry_decay(nuclide, step, instruction2,decay_vec)
 }

}

