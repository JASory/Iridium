use crate::Quark;
use crate::AntiQuark;

// Currently only supports pion and kaon
#[derive(Debug,Copy,Clone)]
pub enum Meson{
     Pion(f64), 
     Kaon(f64),
}

///Currently only supports anti-pion and anti-kaon
#[derive(Debug,Copy,Clone)]
pub enum AntiMeson{
     Pion(f64), 
     Kaon(f64),
}

impl Meson{

   /// Construct pion or kaon using quarks and energy
 pub const fn new(x: Quark, y: AntiQuark, ke: f64)->Meson{
         match (x,y) {
          
          (Quark::Up, AntiQuark::Down)   => return Meson::Pion(ke),
           _                             => return Meson::Kaon(ke),
 
          }
 }
 
 pub const fn rest_mass(&self)->f64{
        match self{
          Meson::Pion(_)  => 139.6f64,
          Meson::Kaon(_) => 135.0f64,
          } 
 }
 
 pub  fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }
 
 pub const fn charge(&self)->f64{
        match self{
          Meson::Pion(_)  => 1f64,
          Meson::Kaon(_) => 1f64,
          } 
 }

}


 impl AntiMeson{
 // construct 
 pub const fn new(x: AntiQuark, y: Quark, ke: f64)->AntiMeson{
         match (x,y) {
          
          (AntiQuark::Up, Quark::Down)   => return AntiMeson::Pion(ke),
           _                             => return AntiMeson::Kaon(ke),
 
          }
 }
 /// Rest mass 
 pub const fn rest_mass(&self)->f64{
        match self{
          AntiMeson::Pion(_)  => 139.6f64,
          AntiMeson::Kaon(_) => 135.0f64,
          } 
 }
 
 pub  fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }
     
 pub const fn charge(&self)->f64{
        match self{
          AntiMeson::Pion(_)  => -1f64,
          AntiMeson::Kaon(_) => -1f64,
          } 
 }
 
 
 
 }
