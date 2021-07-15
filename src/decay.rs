/*
const PROTON_DECAY_MASK     : u16 = 8192 ; // 1
const NEUTRON_DECAY_MASK    : u16 = 16384; // 2
const BETA_DECAY_MASK       : u16 = 24576; // 3 24576
const POSITRON_DECAY_MASK   : u16 = 32768; // 4 32768
const ELECTRON_CAPTURE_MASK : u16 = 40960; // 5    Particle 5 == Electron Neutrino
const ALPHA_DECAY_MASK      : u16 = 49152; // 6
const CLUSTER_DECAY_MASK    : u16 = 57344; // 7  (7,14) is a particle of C-14
*/
use Database::update::PROTON_DECAY_MASK     ;
use Database::update::NEUTRON_DECAY_MASK    ;     
use Database::update::ALPHA_DECAY_MASK      ;    
use Database::update::BETA_DECAY_MASK       ;
use Database::update::POSITRON_DECAY_MASK   ;
use Database::update::ELECTRON_CAPTURE_MASK ;
use Database::update::CLUSTER_DECAY_MASK    ;
use Database::decay_chain::DECAY_CHAIN      ;

use crate::Nuclide;


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
     let alea   = crate::rand()                ;
   
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
   self.change(crate::nucleons_nuclide(&izotop));
 ((primul_nuclide.atomic_mass() - self.atomic_mass()) * 8.98755178780128E+16, particle_vector)
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
       let daughter_rand =  (crate::rand()>>54) as usize;  //ensures randomness
       decay_vec.push(Particle(7,daughter_rand));
       let daughter_rand_iso = Nuclide{nuclide: daughter_rand}.isotope();
       *nuclide = (nuclide.0-daughter_rand_iso.0, nuclide.1-daughter_rand_iso.1)
    }
    else{
    decay_vec.push(Particle(7,daughter));
    let daughter_iso = Nuclide{nuclide: daughter}.isotope();
    *nuclide = (nuclide.0 - daughter_iso.0, nuclide.1 - daughter_iso.1)
    }
 }

  decay_vector[*step] = 0u16;
   *step+=1;
  
  let instruction2 = unsafe{std::mem::transmute::<[u16;4], u64>(decay_vector)};
 
  return curry_decay(nuclide, step, instruction2,decay_vec)
 }

}

