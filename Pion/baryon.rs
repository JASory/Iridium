use crate::AntiQuark;
use crate::Quark;

///Currently only supports proton and neutron
#[derive(Debug,Copy,Clone)]
pub enum Baryon{
     Proton(f64), 
     Neutron(f64),
}
/// Currently only supports anti-proton and anti-neutron
#[derive(Debug,Copy,Clone)]
pub enum AntiBaryon{
     Proton(f64), 
     Neutron(f64),
}


 impl Baryon{
     ///Compose proton or neutron using quarks
  pub const fn new(x: Quark, y: Quark, z: Quark, ke: f64)->Baryon{
      
          match (x,y,z) {
          
          (Quark::Up, Quark::Up, Quark::Down)   => return Baryon::Proton(ke),
           _                                    => return Baryon::Neutron(ke),
 
          }
       }
       
  pub const fn charge(&self)->f64{
         match self{
          Baryon::Proton(_)  => 1f64,
          Baryon::Neutron(_) => 0f64,
          } 
  }      
       // MeV
  pub  const fn rest_mass(&self)->f64{
       
        match self{
          Baryon::Proton(_)  => 938.27231,
          Baryon::Neutron(_) => 939.5656,
          } 
       }
       
  pub  fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }    
       
   pub const fn kinetic(&self)-> f64{
          match self {
          Baryon::Proton(x)  => *x,
          Baryon::Neutron(x) => *x,
          }
   }
           //relativistic momentum
   pub  fn momentum(&self)->f64{
           self.rest_mass()*self.kinetic() / (1.0 - (self.kinetic()*self.kinetic())/89875517873681764f64).sqrt()
        }
        
  }    
  
  
  impl AntiBaryon{
     
  pub const fn new(x: AntiQuark, y: AntiQuark, z: AntiQuark, ke: f64)->AntiBaryon{
      
          match (x,y,z) {
          
          (AntiQuark::Up, AntiQuark::Up, AntiQuark::Down)   => return AntiBaryon::Proton(ke),
           _                                                => return AntiBaryon::Neutron(ke),
 
          }
       }
       
  pub const fn charge(&self)->f64{
         match self{
          AntiBaryon::Proton(_)  => 1f64,
          AntiBaryon::Neutron(_) => 0f64,
          } 
  }      
       /// MeV
  pub const fn rest_mass(&self)->f64{
       
        match self{
          AntiBaryon::Proton(_)  => 938.27231,
          AntiBaryon::Neutron(_) => 939.5656,
          } 
       }
       
  pub  fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }    
       
   pub const fn kinetic(&self)-> f64{
          match self {
          AntiBaryon::Proton(x)  => *x,
          AntiBaryon::Neutron(x) => *x,
          }
   }
           //relativistic momentum
   pub  fn momentum(&self)->f64{
           self.rest_mass()*self.kinetic() / (1.0 - (self.kinetic()*self.kinetic())/89875517873681764f64).sqrt()
        }
        
  } 
          
