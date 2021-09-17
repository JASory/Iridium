#[derive(Debug,Copy,Clone)]
 pub enum Quark{
       Up,
       Down,
       Charm,
       Strange,
       Top,
       Bottom,    
 }
 
 #[derive(Debug,Copy,Clone)]
pub enum AntiQuark{
       Up,
       Down,
       Charm,
       Strange,
       Top,
       Bottom, 
 }
 
 
 impl Quark{
      
pub  const fn rest_mass(&self)->f64{
        match self {
         Quark::Up      => 1.7f64,
         Quark::Down    => 4.1f64,
         Quark::Charm   => 1270f64,
         Quark::Strange => 101f64,
         Quark::Top     => 172000f64,
         Quark::Bottom  => 41900f64,
        }
      }
      
 pub fn charge(&self)->f64 {
      match self {
         Quark::Up      => 2.0/3.0,
         Quark::Down    => -1.0/3.0,
         Quark::Charm   => 2.0/3.0,
         Quark::Strange => -1.0/3.0,
         Quark::Top     => 2.0/3.0,
         Quark::Bottom  => -1.0/3.0,
        }
 }      
 }
 
 impl AntiQuark{
      
pub const  fn rest_mass(&self)->f64{
      match self {
     AntiQuark::Up      => 1.7f64,
     AntiQuark::Down    => 4.1f64,
     AntiQuark::Charm   => 1270f64,
     AntiQuark::Strange => 101f64,
     AntiQuark::Top     => 172000f64,
     AntiQuark::Bottom  => 41900f64,
      }
 }
 
 pub fn charge(&self)->f64 {
      match self {
         AntiQuark::Up      => 2.0/3.0,
         AntiQuark::Down    => -1.0/3.0,
         AntiQuark::Charm   => 2.0/3.0,
         AntiQuark::Strange => -1.0/3.0,
         AntiQuark::Top     => 2.0/3.0,
         AntiQuark::Bottom  => -1.0/3.0,
        }
 }      
 
 }
