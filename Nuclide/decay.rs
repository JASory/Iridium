

use crate::decay_chain::DECAY_CHAIN      ;
use crate::half_life::HALF_LIFE          ;

 use crate::nuclide::nucleons_nuclide    ;
 use crate::nuclide::Nuclide             ;
 use crate::particle::Particle           ;

 use crate::particle::PROTONMASS         ; 
 use crate::particle::NEUTRONMASS        ; 
 use crate::particle::ELECTRONMASS       ; 
 use crate::particle::ALPHAMASS          ; 
 use crate::particle::DEUTERONMASS       ; 
 use crate::particle::TRITONMASS         ; 
 use crate::particle::NEUTRINOMASS       ;   

pub fn rand()->u64{
   let mut x: u64 = 0;
  let k = unsafe{core::arch::x86_64::_rdrand64_step(&mut x)};
x
}


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
       
    /// Returns the probable decay modes  as a string 
   pub fn decay_mode(&self)-> String{
   
  let mut unu_alea   =   ((DECAY_CHAIN[(self.nuclide_index()*6)] as f64/18446744073709552000f64)*100.0).to_string(); 
      unu_alea.truncate(4); unu_alea.push_str("% ");
  let mut doua_alea  =   ((DECAY_CHAIN[(self.nuclide_index()*6+1)] as f64/18446744073709552000f64)*100.0).to_string();
      doua_alea.truncate(4); doua_alea.push_str("% ");
  let mut trei_alea  =   ((DECAY_CHAIN[(self.nuclide_index()*6+2)] as f64/18446744073709552000f64)*100.0).to_string();
       trei_alea.truncate(4); trei_alea.push_str("% ");
  let mut patru_alea =   ((DECAY_CHAIN[(self.nuclide_index()*6+3)] as f64/18446744073709552000f64)*100.0).to_string();
      patru_alea.truncate(4); patru_alea.push_str("% ");
  let mut cinci_alea =   ((DECAY_CHAIN[(self.nuclide_index()*6+4)] as f64/18446744073709552000f64)*100.0).to_string();   
      cinci_alea.truncate(4); cinci_alea.push_str("% ");

  let mut decay_string = vec![];
  
  let decay_vector = DECAY_CHAIN[self.nuclide_index()*6 + 5].to_be_bytes();
  
  for i in decay_vector[..5].iter() {
    match i {
           1 => decay_string.push("α; "),
           2 => decay_string.push("p; "),
           3 => decay_string.push("2p; "),
           4 => decay_string.push("n; "),
           5 => decay_string.push("2n; "),
           6 => decay_string.push("EC; "),  
           7 => decay_string.push("2EC; "), 
           8 => decay_string.push("β− + p; "),
           9 => decay_string.push("β+; "),
          10 => decay_string.push("2β+; "),
          11 => decay_string.push("β−; "),
          12 => decay_string.push("2β−; "),
          13 => decay_string.push("β− + n; "), 
          14 => decay_string.push("β− + 2n; "),
          15 => decay_string.push("β− + 3n; "),
          16 => decay_string.push("β+ + p; "),
          17 => decay_string.push("β+ + 2p; "),
          18 => decay_string.push("β+ + 3p; "),
          19 => decay_string.push("β- + α; "),
          20 => decay_string.push("β+ + α; "),
          21 => decay_string.push("β- + d; "),    
          22 => decay_string.push("β- + t; "),
          23 => decay_string.push("SF; "),
          24 => decay_string.push("β- + SF; "),     
          25 => decay_string.push("β+ + SF; "),     
          26 => decay_string.push("C-14; "),    
          27 => decay_string.push("Ne-20; "),                                              
          28 => decay_string.push("Ne-24; "), 
          29 => decay_string.push("Ne-20 + NE-24; "), 
          30 => decay_string.push("Si-32; "),                                              
          31 => decay_string.push("Si-34; "), 



        
        _=> decay_string.push("Null"),
  }
  }
  let mut decayvec = vec![]; decayvec.push(unu_alea); decayvec.push(decay_string[0].to_string()); 
  decayvec.push(doua_alea); decayvec.push(decay_string[1].to_string()); decayvec.push(trei_alea);
  decayvec.push(decay_string[2].to_string()); decayvec.push(patru_alea);decayvec.push(decay_string[3].to_string());
  decayvec.push(cinci_alea); decayvec.push(decay_string[4].to_string());
  
  
  if decayvec[0] == "0% " {
  return "Stable".to_string();
  }
  else {
    match decayvec.iter().position(|r| r == "Null"){
      Some(x)=> decayvec.truncate(x-1),
      None=> decayvec.truncate(10),
    }
    return decayvec.join("")
  }

   }    
   
   



/// Perfroms a maximum of one decay in the time given
pub fn static_decay(&mut self, time: f64)->(f64,Vec<Particle>) {
 
let mut particle_vector = vec![];
let mut energia = 0f64 ;
let mut izotop = self.proton_neutron();
 if self.decay_time(time) {
 
 let mut x : u8 = 0;
 
     let idx    = self.nuclide_index()*6      ; 
     let unu    = DECAY_CHAIN[idx]             ;
     let doua   = DECAY_CHAIN[idx + 1] + unu   ;
     let trei   = DECAY_CHAIN[idx + 2] + doua  ;
     let patru  = DECAY_CHAIN[idx + 3] + trei  ;
     let cinci  = DECAY_CHAIN[idx + 4] + patru ; 
     let alea   = rand()                       ;
     
     let decay_vector = DECAY_CHAIN[self.nuclide_index()*6 + 5].to_be_bytes();
     
     if alea > 0 && alea < unu { 
       x = decay_vector[0] ;
     }
     else if alea > unu && alea < doua{
       x = decay_vector[1] ;
     }
     else if alea > doua && alea < trei {
      x = decay_vector[2] ;
     }
     else if alea > trei  && alea < patru{
      x = decay_vector[3] ;
     }
     else if alea > patru && alea < cinci { 
      x = decay_vector[4] ;
     }
     
  
match x {
  
    1  => {  //1 alpha
            izotop.0-=2; izotop.1-=2;  
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(ALPHAMASS/self.amu())* totalenergia ; 
             energia = totalenergia-particleenergia;
             self.change(daughter.nuclide_index());
             particle_vector.push(Particle::Alpha(particleenergia));
             } ,
    2  => {       // 1p
            izotop.0-=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(PROTONMASS/self.amu())* totalenergia ; 
             energia = totalenergia-particleenergia;
             self.change(daughter.nuclide_index());
             particle_vector.push(Particle::Proton(particleenergia));
          } ,
    3  => {                    // 2p
            izotop.0-=2;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(2.0*PROTONMASS/self.amu())* totalenergia ;   
             energia = totalenergia-particleenergia;
             self.change(daughter.nuclide_index());
             particle_vector.push(Particle::Proton(particleenergia/2.0));
             particle_vector.push(Particle::Proton(particleenergia/2.0));
          } ,
    4  => {          // n 
                  izotop.1-=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(NEUTRONMASS/self.amu())* totalenergia ; 
            energia = totalenergia-particleenergia;
             self.change(daughter.nuclide_index());
             particle_vector.push(Particle::Neutron(particleenergia));            
          } ,
    5  => {          // 2n 
                  izotop.1-=2;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(2.0*NEUTRONMASS/self.amu())* totalenergia ;
             energia = totalenergia-particleenergia;
             self.change(daughter.nuclide_index());
             particle_vector.push(Particle::Neutron(particleenergia/2.0));
             particle_vector.push(Particle::Neutron(particleenergia/2.0));       
          }      
    6  => {        println!("6");            // Electron capture
                  izotop.0-=1; izotop.1+=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(NEUTRINOMASS/self.amu())* totalenergia ;
            energia = totalenergia-particleenergia;
            self.change(daughter.nuclide_index());
            particle_vector.push(Particle::Neutrino(particleenergia));
          } ,
    7  => {            // Double Electron Capture
                  izotop.0-=2; izotop.1+=2;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -(2.0*NEUTRINOMASS/self.amu())* totalenergia ;
            energia = totalenergia-particleenergia;
            self.change(daughter.nuclide_index());
            particle_vector.push(Particle::Neutrino(particleenergia/2.0));
            particle_vector.push(Particle::Neutrino(particleenergia/2.0));
          } , 
    8  => {                    // B-p
            izotop.1-=1; 
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+PROTONMASS)/self.amu())* totalenergia ;   
            let protonenergia  = particleenergia - particleenergia*2.35E-7;
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Proton(protonenergia));
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));  
          } ,      
    9  => {          // Beta +  http://hyperphysics.phy-astr.gsu.edu/hbase/Nuclear/beta2.html#c1
                  izotop.0-=1; izotop.1+=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS)/self.amu())* totalenergia ;  
            energia = totalenergia - particleenergia ; 
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7));              
          } ,
   10  => {            // Double Beta + 
                  izotop.0-=2; izotop.1+=2;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS)*2.0/self.amu())* totalenergia ;  
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)*0.5));  
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)*0.5));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*0.5));
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*0.5));    
          } ,
   11  => {           // Beta -
                  izotop.0+=1; izotop.1-=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu() -daughter.amu())*931.49410242;;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS)/self.amu())* totalenergia ; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7));
            self.change(daughter.nuclide_index());
          } ,
   12  => {          // Double Beta -
                  izotop.0+=2; izotop.1-=2;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let mass_delta = self.amu() -daughter.amu(); 
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS)*2.0/self.amu())* totalenergia ;  
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)*0.5));  
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)*0.5));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*0.5));
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*0.5));      
          } ,
   13  => {                    // B- + n 
                  izotop.0+=1; izotop.1-=2; 
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+NEUTRONMASS)/self.amu())* totalenergia ;   
            let neutronenergia  = particleenergia - particleenergia*2.35E-7;
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Neutron(neutronenergia));
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));      
          } ,
   14  => {                    // B- + 2n
                  izotop.0+=1; izotop.1-=3; 
                  
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+2.0*NEUTRONMASS)/self.amu())* totalenergia ;   
            let neutronenergia  = particleenergia - particleenergia*2.35E-7;   
            energia = totalenergia - particleenergia ;       
            particle_vector.push(Particle::Neutron(neutronenergia/2.0));
            particle_vector.push(Particle::Neutron(neutronenergia/2.0));            
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));       
          } ,
   15  => {                    // B- + 3n
            izotop.0+=1; izotop.1-=4; 
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+3.0*NEUTRONMASS)/self.amu())* totalenergia ;   
            let neutronenergia  = particleenergia - particleenergia*2.35E-7;
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Neutron(neutronenergia/3.0));
            particle_vector.push(Particle::Neutron(neutronenergia/3.0));      
            particle_vector.push(Particle::Neutron(neutronenergia/3.0));            
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));       
          } ,
   16  => {                    // B+ + p
            izotop.0-=2; izotop.1+=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+PROTONMASS)/self.amu())* totalenergia  ;
            let protonenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Proton(protonenergia));    
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*ELECTRONMASS));   
          } ,
   17  => {                    // B+ + 2p
            izotop.0-=3; izotop.1+=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+2.0*PROTONMASS)/self.amu())* totalenergia  ;
            let protonenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Proton(protonenergia/2.0));  
            particle_vector.push(Particle::Proton(protonenergia/2.0));      
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*ELECTRONMASS));      
          } ,
   18  => {                    // B+ + 3p
            izotop.0-=4; izotop.1+=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+3.0*PROTONMASS)/self.amu())* totalenergia;  
            let protonenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Proton(protonenergia/3.0));  
            particle_vector.push(Particle::Proton(protonenergia/3.0));      
            particle_vector.push(Particle::Proton(protonenergia/3.0));      
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*ELECTRONMASS));       
                                  
          } ,
   19  => {                    // B- + A 
                  izotop.0-=1; izotop.1-=3;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+ALPHAMASS)/self.amu())* totalenergia ;  
            let alphaenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Alpha(alphaenergia));    
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS))
          } ,
   20  => {                    // B+ + A
                  izotop.0-=3; izotop.1-=1;
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+ALPHAMASS)/self.amu())* totalenergia ;   
            let alphaenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Alpha(alphaenergia));    
            particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*ELECTRONMASS));      
          } ,
   21  => {                    // B- + Deuteron 
                  izotop.1-=3; 
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+DEUTERONMASS)/self.amu())* totalenergia   ;  
            let deuteronenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Deuteron(deuteronenergia));    
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));            
          } ,
   22  => {                    // B- + Triton
                  izotop.1-=4;     
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let particleenergia = totalenergia -((ELECTRONMASS+NEUTRINOMASS+TRITONMASS)/self.amu())* totalenergia   ;  
            let tritonenergia  = particleenergia - particleenergia*2.35E-7; 
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Triton(tritonenergia));    
            particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));       
          } ,
   23  => {                    // SF
             let rand = rand()>>54;
             let decay_particle = Nuclide::assign(rand as usize);
             let iso = decay_particle.proton_neutron() ;
             izotop.0-=iso.0 ; izotop.1-=iso.1 ;
             let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
             let totalenergia = (self.amu()- daughter.amu())*931.49410242;
             let particleenergia = totalenergia -(decay_particle.amu()/self.amu())* totalenergia;  
             energia = totalenergia - particleenergia ;
              particle_vector.push(Particle::Element(decay_particle,particleenergia));
             
          } ,
          
   24  => {                    // B- + SF
             let rand = rand()>>54;
             let decay_particle = Nuclide::assign(rand as usize);
             let iso = decay_particle.proton_neutron() ;
             izotop.0-=(iso.0-1) ; izotop.1-=(iso.1+1) ;
             let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
             let totalenergia = (self.amu()- daughter.amu())*931.49410242;
             let particleenergia = (daughter.amu()/self.amu())* totalenergia ;  
             energia = totalenergia - particleenergia ;
              particle_vector.push(Particle::Element(decay_particle,particleenergia));
              particle_vector.push(Particle::Electron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::AntiNeutrino(particleenergia*2.35E-7*ELECTRONMASS));  
          } ,
   25  => {                    // B+  +  SF
             let rand = rand()>>54;
             let decay_particle = Nuclide::assign(rand as usize);
             let iso = decay_particle.proton_neutron() ;
             izotop.0-=(iso.0-1) ; izotop.1-=(iso.1+1) ;
             let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
             let totalenergia = (self.amu()- daughter.amu())*931.49410242;
             let particleenergia = (daughter.amu()/self.amu())* totalenergia ;  
             energia = totalenergia - particleenergia ;
              particle_vector.push(Particle::Element(decay_particle,particleenergia));
              particle_vector.push(Particle::Positron(particleenergia*(1.0-2.35E-7)));  
            particle_vector.push(Particle::Neutrino(particleenergia*2.35E-7*ELECTRONMASS));  
          } ,
   26  => {                    // CD C14 
                   izotop.0-=6; izotop.1-=8;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let c14 = Nuclide::assign(60usize);
            let particleenergia = totalenergia - (c14.amu()/self.amu())* totalenergia ;  
            energia = totalenergia - particleenergia ;
            particle_vector.push(Particle::Element(c14,particleenergia));       //  missing value   
          } ,
   27  => {                    // Ne-20
               izotop.0-=10; izotop.1-=10;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let Ne20 = Nuclide::assign(128usize);
            let particleenergia = totalenergia - (Ne20.amu()/self.amu())* totalenergia ;  
            energia = totalenergia - particleenergia ; 
            particle_vector.push(Particle::Element(Ne20,particleenergia));                                 
          } ,                                   
          
   28  => {                    //  Ne-24
            izotop.0-=10; izotop.1-=14;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let Ne24 = Nuclide::assign(132usize);
            let particleenergia = totalenergia - (Ne24.amu()/self.amu())* totalenergia ; 
            energia = totalenergia - particleenergia ;   
            particle_vector.push(Particle::Element(Ne24,particleenergia));                                          
          } ,
          
   29  => {                  // Ne-26 + Ne-24                    // correct decay energys
            izotop.0-=20; izotop.1-=30;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let Ne24 = Nuclide::assign(132usize); let Ne26 = Nuclide::assign(134usize);
            let particleenergia = totalenergia - ((Ne24.amu() + Ne26.amu())/self.amu())* totalenergia ; 
            let neonenergia  = particleenergia -(Ne24.amu()/(Ne26.amu()+Ne24.amu()))*particleenergia; 
            particle_vector.push(Particle::Element(Ne24,neonenergia)); 
            particle_vector.push(Particle::Element(Ne26,particleenergia-neonenergia));       
          } ,                     
                                             // 2924 60268       28 Mg  175    23F   18O  20O  add B-p
                                             
   30  => {                   // Si-32
             izotop.0-=14; izotop.1-=18;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let Si32 = Nuclide::assign(224usize);
            let particleenergia = totalenergia - (Si32.amu()/self.amu())* totalenergia ;  
           energia = totalenergia - particleenergia ; 
           particle_vector.push(Particle::Element(Si32,particleenergia)); 
          } ,    
   31  => {                   // Si-34
             izotop.0-=14; izotop.1-=20;                        
            let daughter= Nuclide::assign(nucleons_nuclide(&izotop) );
            let totalenergia = (self.amu()- daughter.amu())*931.49410242;
            let Si34 = Nuclide::assign(226usize);
            let particleenergia = totalenergia - (Si34.amu()/self.amu())* totalenergia ;  
           energia = totalenergia - particleenergia ; 
           particle_vector.push(Particle::Element(Si34,particleenergia)); 
          }
   
    _   => (),

 }
 }
 self.change(nucleons_nuclide(&izotop));
 (energia,particle_vector)
}
/** 
     Performs decay over the given time
     
        ```
        let uranium = Nuclide::new("U", 238).unwrap();
        
       // total particle energy of the nuclide and vector of decay particles
        let (particle_energy,particle_vector) = uranium.decay(5E+20);
        
         // final nuclide in the U-238 decay series
        assert_eq!(uranium.identity(), "Pb-206"); 
        ```
     */
 pub fn decay(&mut self, mut time: f64)->(f64, Vec<Particle>){// 
   let mut total_energy = 0f64;
   let mut particlevec = vec![];
  while  time > self.mean_lifetime(){
     let  k = self.static_decay(time) ; 
     total_energy+=k.0;
     particlevec.extend_from_slice(&k.1[..]);
     
     time -=self.mean_lifetime(); 
  }
  (total_energy, particlevec)
 }
 /*
 theorectical nuclide using the liquid drop model
                                   &str amu, binding energy,  likely decay mode
 fn create(proton: usize, neutron: usize)->(f64,f64){
    
 }
 
*/
}



