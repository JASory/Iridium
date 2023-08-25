
#[derive(Debug,Copy,Clone)]
pub enum Lepton{
     Electron(f64),
     Muon(f64), 
     Tau(f64),
     ElectronNeutrino(f64),
     MuNeutrino(f64),
     TauNeutrino(f64),
}
#[derive(Debug,Copy,Clone)]
pub enum AntiLepton{
     Electron(f64),
     Muon(f64), 
     Tau(f64),
     ElectronNeutrino(f64),
     MuNeutrino(f64),
     TauNeutrino(f64),
}

 impl Lepton{
               ///rest mass in MeV
   pub const fn rest_mass(&self)->f64{
   
    match self {
        Lepton::Electron(_) => 0.510998950,
        Lepton::Muon(_)     => 105.6583755,
        Lepton::Tau(_)      => 1776.86,
        Lepton::ElectronNeutrino(_) => 0f64,
        Lepton::MuNeutrino(_)       => 0f64,
        Lepton::TauNeutrino(_)      => 0f64,
        }
    }
    
    
 
  pub  fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }
    
  pub const  fn kinetic(&self)->f64{
       match self {
        Lepton::Electron(x) => *x,
        Lepton::Muon(x)     => *x,
        Lepton::Tau(x)      => *x,
        Lepton::ElectronNeutrino(x) => *x,
        Lepton::MuNeutrino(x)       => *x,
        Lepton::TauNeutrino(x)      => *x,
        }
  }
 
    
 
  pub fn momentum(&self)->f64{
       self.rest_mass()*self.kinetic() / (1.0 - (self.kinetic()*self.kinetic())/89875517873681764f64).sqrt()
    }
    
 pub const fn charge(&self)->f64{
        match self {
        Lepton::Electron(_) => -1f64,
        Lepton::Muon(_)     => -1f64,
        Lepton::Tau(_)      => -1f64,
        Lepton::ElectronNeutrino(_) => 0f64,
        Lepton::MuNeutrino(_)       => 0f64,
        Lepton::TauNeutrino(_)      => 0f64,
        }
 }   
 }
 
 
 impl AntiLepton{
        ///rest mass in MeV
   pub const fn rest_mass(&self)->f64{
   
    match self {
        AntiLepton::Electron(_) => 0.510998950,
        AntiLepton::Muon(_)     => 105.6583755,
        AntiLepton::Tau(_)      => 1776.86,
        AntiLepton::ElectronNeutrino(_) => 0f64,
        AntiLepton::MuNeutrino(_)       => 0f64,
        AntiLepton::TauNeutrino(_)      => 0f64,
        }
    }
    
    
 
  pub fn rest_mass_kg(&self)->f64{
      self.rest_mass()*1.79E-30
    }
    
  pub const fn kinetic(&self)->f64{
       match self {
        AntiLepton::Electron(x) => *x,
        AntiLepton::Muon(x)     => *x,
        AntiLepton::Tau(x)      => *x,
        AntiLepton::ElectronNeutrino(x) => *x,
        AntiLepton::MuNeutrino(x)       => *x,
        AntiLepton::TauNeutrino(x)      => *x,
        }
  }
 
 pub const fn charge(&self)->f64{
        match self {
        AntiLepton::Electron(_) => 1f64,
        AntiLepton::Muon(_)     => 1f64,
        AntiLepton::Tau(_)      => 1f64,
        AntiLepton::ElectronNeutrino(_) => 0f64,
        AntiLepton::MuNeutrino(_)       => 0f64,
        AntiLepton::TauNeutrino(_)      => 0f64,
        }
 }   
 
  pub  fn momentum(&self)->f64{
       self.rest_mass()*self.kinetic() / (1.0 - (self.kinetic()*self.kinetic())/89875517873681764f64).sqrt()
    }
 }
 
