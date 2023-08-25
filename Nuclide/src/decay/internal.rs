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
  /// Returns 
  ///# None
  /// Decay Mode is not supported
  fn daughter<T: DecayMode> -> Option<Self>
  
  fn daughter_energetic<T: DecayMode> -> (f64,Vec<Particle>)
  
  // Performs decay regardless of whether it has been observed or is possible with current models
  // # None
  // Returns None if the resultant nuclide does not exist
  fn forced_decay<T: DecayMode>(&self) -> Option<Self>
  
  fn forced_decay_energetic<T: DecayMode>(&self) -> (f64,Vec<Particle>)
  
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
  // Forces the decay mode regardless of whether it is supported, returns None if no real nuclide would be created
  fn force_decay<DecayMode> -> Option<Nuclide>
  
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
use crate::decay::types::*;
use crate::decay::dhelper::{max,theoretical_mass,elementary_mass, decay_mode_idx,decay_select};
use crate::atom::Atom;
//use crate::nuclidedata::decay_chain::DECAY_CHAIN;
use crate::rng::rand;
use crate::Particle;
use crate::constant::*;
use Pion::{Baryon,Lepton,AntiLepton};
use crate::nuclidedata::index::SYMBOL_INDEX;


pub trait InternalDecay {
   fn decay(x: &mut Nuclide) -> (f64,Vec<Particle>);
   // Decay_index return u8::MAX if not supported by system 254 for all decay modes
   fn decay_index() -> u8;
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>;
  // Return nan if the separation energy is not possible, negative if it 
  // fn separation_energy -> f64
  
  // The total energy released by the reaction, negative for 
   fn q_value(x: &Nuclide) -> f64;
}

impl<const K: usize> InternalDecay for ProtonEmission<K>{

   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
   
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
    
    let p_mass = PROTONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + p_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*(p_mass/t_mass);
    let p_energy = q*(d_mass/t_mass);
    
    *x = dghter;
    
    let mut particle_vector = vec![];
    for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Proton(p_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,n) = x.proton_neutron();
       p=p.wrapping_sub(K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
   fn decay_index() -> u8{
     if K == 1{
      return 2
     }
     
     if K == 2{
      return 3
     }
     
     if K == 3{
      return 48
     }
     
     u8::MAX
   }
   
   fn q_value(x: &Nuclide) -> f64{
      let (mut p,n) = x.proton_neutron();
      if K >= p{
        return f64::NAN
      }
      p=p.wrapping_sub(K);
      let res_mass = elementary_mass(K,0,0,0);
      
      match Nuclide::from_nucleons(p,n){
       // If a daughter nuclide exists sum masses of results and compute energy from mass loss
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        // If daughter nuclide does not exist
        // Compute binding energy from theoretical nuclide
        // Subtract it from the mass of protons and neutrons to get the theoretical mass
        None => {
        let d_mass = theoretical_mass(p,n);
        (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
   }
   

/*
  fn sep_energy(x: &Nuclide) -> f64{
    match Self::decay_result(x){
     Some(z) =>,
     None => f64::NAN,
    }
  }
  */
}

impl<const K: usize> InternalDecay for NeutronEmission<K>{
   
     fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
   
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }
 
    match Self::decay_result(x){
        Some(dghter) => {
    
    let n_mass = NEUTRONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + n_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*(n_mass/t_mass);
    let p_energy = q*(d_mass/t_mass);
    
    *x = dghter;
    
    let mut particle_vector = vec![];
    for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Neutron(p_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (p,mut n) = x.proton_neutron();
       n=n.wrapping_sub(K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
   fn decay_index() -> u8{
     if K == 1{
      return 4
     }
     if K == 2{
      return 5
     }
     u8::MAX
   }
   
fn q_value(x: &Nuclide) -> f64{
      let (p,mut n) = x.proton_neutron();
      if K >= n{
        return f64::NAN
      }
      n=n.wrapping_sub(K);
      let res_mass = elementary_mass(0,K,0,0);
      
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
   }
   
}

impl InternalDecay for NeutronDeuteron{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }
    match Self::decay_result(x){
        Some(dghter) => {
    
    let n_mass = NEUTRONMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + n_mass + DEUTERONMASS;
    let d_energy = q*((n_mass + DEUTERONMASS)/t_mass);
    let n_energy = q*((d_mass + DEUTERONMASS)/t_mass);
    let deu_energy = q*((n_mass + d_mass)/t_mass);
        
    *x = dghter;
    
    let particle_vector = vec![
            Particle::Deuteron(deu_energy),
            Particle::Baryon(Baryon::Neutron(n_energy)),
        ];
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(1);
       n=n.wrapping_sub(3); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
   
     fn decay_index() -> u8{
       40
     }
     
     fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      if 1 >= p || 3 >= n{
        return f64::NAN
      }
      p=p.wrapping_sub(1);
      n=n.wrapping_sub(3);
      let res_mass = elementary_mass(0,1,0,0) + DEUTERONMASS;
      
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
      
     } 
}

impl InternalDecay for NeutronTriton{
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }
    match Self::decay_result(x){
        Some(dghter) => {
    
    let n_mass = NEUTRONMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + n_mass + TRITONMASS;
    let d_energy = q*((n_mass + TRITONMASS)/t_mass);
    let n_energy = q*((d_mass + TRITONMASS)/t_mass);
    let tri_energy = q*((n_mass + d_mass)/t_mass);
        
    *x = dghter;
    
    let particle_vector = vec![
            Particle::Triton(tri_energy),
            Particle::Baryon(Baryon::Neutron(n_energy)),
        ];
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(1);
       n=n.wrapping_sub(4); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  
  fn decay_index() -> u8{
      41
  }
  
  
     
   fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      if 1 >= p || 4 >= n{
        return f64::NAN
      }
      p=p.wrapping_sub(1);
      n=n.wrapping_sub(4);
      let res_mass = elementary_mass(0,1,0,0) + TRITONMASS;
      
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }
      
}

impl<const K: usize> InternalDecay for AlphaEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }
 
    match Self::decay_result(x){
        Some(dghter) => {
    
    let a_mass = ALPHAMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + a_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*(a_mass/t_mass);
    let a_energy = q*(d_mass/t_mass);
        
    *x = dghter;
    
    let mut particle_vector = vec![];
    for _ in 0..K{
        particle_vector.push(Particle::Alpha(a_energy * scalar));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(K<<1);
       n=n.wrapping_sub(K<<1); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
    if K == 1{
      return 1
    }
    u8::MAX  
  }
  
  fn q_value(x: &Nuclide) -> f64{
       let (mut p,mut n) = x.proton_neutron();
       
       if (K<<1) >= p || (K<<1) >= n{
        return f64::NAN
      }
       
       p=p.wrapping_sub(K<<1);
       n=n.wrapping_sub(K<<1);
       let res_mass = (K as f64)*ALPHAMASS;
       
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }
     
}

impl<const K: usize> InternalDecay for AlphaNeutron<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }
    match Self::decay_result(x){
        Some(dghter) => {
    
    let scale = (K as f64).recip();
    let n_mass = NEUTRONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + n_mass + ALPHAMASS;
    let d_energy = q*((n_mass + ALPHAMASS)/t_mass);
    let n_energy = q*((d_mass + ALPHAMASS)/t_mass);
    let a_energy = q*((n_mass + d_mass)/t_mass);
        
    *x = dghter;
    
    let mut particle_vector = vec![
            Particle::Alpha(a_energy)
        ];
        
     for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Neutron(n_energy * scale)) )
     }
        
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(2);
       n=n.wrapping_sub(2+K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
  
  fn decay_index() -> u8{
      if K == 1{
        return 42
      }
      if K == 2{
        return 43
      }
      if K == 3{
       return 44
      }
      u8::MAX
  }  
  
  fn q_value(x: &Nuclide) -> f64{
     let (mut p,mut n) = x.proton_neutron();
       
       if 2 >= p || (K+2) >= n{
        return f64::NAN
      }
       
       p=p.wrapping_sub(2);
       n=n.wrapping_sub(K+2);
       let res_mass = elementary_mass(0,K,0,0) + ALPHAMASS; 
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }  
}

impl<const K: usize> InternalDecay for ElectronCapture<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*(neu_mass/t_mass);
    let neu_energy = q*(d_mass/t_mass);
        
    *x = dghter;
    
    let mut particle_vector = vec![];
    for _ in 0..K{
        particle_vector.push(Particle::Lepton(Lepton::ElectronNeutrino(neu_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(K);
       n=n.wrapping_add(K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }   

  fn decay_index() -> u8{
      if K == 1{
       return 6
      }
      if K == 2{
       return 7
      }
      u8::MAX
  }   
  
  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      
      if K >= p{
        return f64::NAN
      }
      
      p=p.wrapping_sub(K);
      n=n.wrapping_add(K);
      let res_mass = elementary_mass(0,0,0,K);
      match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
      
     }
     
}

impl<const K: usize> InternalDecay for ElectronEmission<K>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS*(K as f64);
    let e_mass = ELECTRONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*((neu_mass + e_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass)/t_mass);
        
    *x = dghter;
        
    let mut particle_vector = vec![];
    for _ in 0..K{
        particle_vector.push(Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy * scalar)));
        particle_vector.push(Particle::Lepton(Lepton::Electron(e_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
  
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_add(K);
       n=n.wrapping_sub(K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }    
   
  fn decay_index() -> u8{
      if K == 1{
       return 11
      }
      if K == 2{
       return 12
      }
      u8::MAX
  }
  
  fn q_value(x: &Nuclide) -> f64{
    let (mut p,mut n) = x.proton_neutron();
   
     if K >= n{
       return f64::NAN
     }
     
       p=p.wrapping_add(K);
       n=n.wrapping_sub(K);
       let res_mass = elementary_mass(0,0,K,0);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
       
       
     }
     
}

impl<const K: usize> InternalDecay for ElectronNeutron<K>{
  
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let n_mass = NEUTRONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + n_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*((neu_mass + e_mass + n_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + n_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + n_mass)/t_mass);
    let n_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let mut particle_vector = vec![
    Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy)),
    Particle::Lepton(Lepton::Electron(e_energy))
    ];
    for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Neutron(n_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_add(1);
       n=n.wrapping_sub(1+K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }

  fn decay_index() -> u8{
      if K == 1{
        return 13
      }
      if K == 2{
        return 14
      }
      if K == 3{
       return 15
      }
      u8::MAX
  }
  
  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      
      if (K+1) >= n{
       return f64::NAN
      }
      
       p=p.wrapping_add(1);
       n=n.wrapping_sub(K+1);
       let res_mass = elementary_mass(0,K,1,1);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
       
     }
     
}

impl<const K: usize> InternalDecay for ElectronProton<K>{

// FIXME
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let p_mass = PROTONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + p_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*((neu_mass + e_mass + p_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + p_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + p_mass)/t_mass);
    let p_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let mut particle_vector = vec![
    Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy)),
    Particle::Lepton(Lepton::Electron(e_energy))
    ];
    for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Proton(p_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(K-1);
       n=n.wrapping_sub(1); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      if K == 1{
        return 45
      }
      if K == 2{
        return 46
      }
      if K == 3{
       return 47
      }
      u8::MAX
  }
  
  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      
      if (K-1) >= p{
       return f64::NAN
      }
      
       p=p.wrapping_add(K-1);
       n=n.wrapping_sub(1);
       let res_mass = elementary_mass(K,0,1,1);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
      
     }
    
}

impl InternalDecay for ElectronAlpha{
// FIXME
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let a_mass = ALPHAMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + a_mass;
   
    let d_energy = q*((neu_mass + e_mass + a_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + a_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + a_mass)/t_mass);
    let a_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let particle_vector = vec![
    Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy)),
    Particle::Lepton(Lepton::Electron(e_energy)),
    Particle::Alpha(a_energy)
    ];
    
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   // FIXME Same as Neutron Deuteron
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(1);
       n=n.wrapping_sub(3); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      19
  }   
  
  fn q_value(x: &Nuclide) -> f64{
     let (mut p,mut n) = x.proton_neutron();
      
      if 1 >= p || 3 >= n{
       return f64::NAN
      }
      
       p=p.wrapping_add(1);
       n=n.wrapping_sub(3);
       let res_mass = elementary_mass(0,0,1,1)+ALPHAMASS;
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     } 
}

impl InternalDecay for ElectronDeuteron{
// FIXME
  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let deu_mass = DEUTERONMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + deu_mass;
   
    let d_energy = q*((neu_mass + e_mass + deu_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + deu_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + deu_mass)/t_mass);
    let deu_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let particle_vector = vec![
    Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy)),
    Particle::Lepton(Lepton::Electron(e_energy)),
    Particle::Deuteron(deu_energy)
    ];
    
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
   fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (p,mut n) = x.proton_neutron();
       n=n.wrapping_sub(2); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      21
  }
  
  fn q_value(x: &Nuclide) -> f64{
       let (p,mut n) = x.proton_neutron();
       
       if 2 >= n{
         return f64::NAN
       }
       
       n=n.wrapping_sub(2);
       let res_mass = elementary_mass(0,0,1,1)+DEUTERONMASS;
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }   
}

impl InternalDecay for ElectronTriton{

  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let tri_mass = TRITONMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + tri_mass;
   
    let d_energy = q*((neu_mass + e_mass + tri_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + tri_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + tri_mass)/t_mass);
    let tri_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let particle_vector = vec![
    Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energy)),
    Particle::Lepton(Lepton::Electron(e_energy)),
    Particle::Triton(tri_energy)
    ];
    
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
  
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (p,mut n) = x.proton_neutron();
       n=n.wrapping_sub(3); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      22
  } 
  
  fn q_value(x: &Nuclide) -> f64{
       let (p,mut n) = x.proton_neutron();
       
       if 3 >= n{
         return f64::NAN
       }
       
       n=n.wrapping_sub(3);
       let res_mass = elementary_mass(0,0,1,1)+TRITONMASS;
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }  
}

impl InternalDecay for ElectronFission{

  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
        let (energia, mut particle_vector) = ElectronEmission::<1>::decay(x);
        let (s_energia, s_vector) = SpontaneousFission::decay(x);
        particle_vector.push(s_vector[0].clone());
        (energia + s_energia, particle_vector)
   }
  
  fn decay_result(x: &Nuclide) ->  Option<Nuclide>{
    let mut xclone = *x;
    Self::decay(&mut xclone);
    Some(xclone)
  }
   
  fn decay_index() -> u8{
      24
  }  
   // Fission cannot be reproduced due to RNG call
  fn q_value(_x: &Nuclide) -> f64{
      f64::NAN
     } 
}

impl<const K: usize> InternalDecay for PositronEmission<K>{

   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
    let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS*(K as f64);
    let e_mass = ELECTRONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*((neu_mass + e_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass)/t_mass);
        
    *x = dghter;
        
    let mut particle_vector = vec![];
    for _ in 0..K{
    
        particle_vector.push(Particle::Lepton(Lepton::ElectronNeutrino(neu_energy * scalar)));
        particle_vector.push(Particle::AntiLepton(AntiLepton::Electron(e_energy * scalar))); 
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(K);
       n=n.wrapping_add(K); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      if K == 1{
       return 9
      }
      if K == 2{
       return 10
      }
      u8::MAX
  }
  
  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
   
     if K >= n{
       return f64::NAN
     }
     
       p=p.wrapping_sub(K);
       n=n.wrapping_add(K);
       let res_mass = elementary_mass(0,0,K,0);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }    
}

impl<const K: usize> InternalDecay for PositronProton<K>{

   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let p_mass = PROTONMASS*(K as f64);
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + p_mass;
    let scalar = (K as f64).recip();
    let d_energy = q*((neu_mass + e_mass + p_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + p_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + p_mass)/t_mass);
    let p_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
    
    let mut particle_vector = vec![
    Particle::Lepton(Lepton::ElectronNeutrino(neu_energy)),
    Particle::AntiLepton(AntiLepton::Electron(e_energy))
    ];
    for _ in 0..K{
        particle_vector.push(Particle::Baryon(Baryon::Proton(p_energy * scalar)));
    }
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }

  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(1+K);
       n=n.wrapping_add(1); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }

  fn decay_index() -> u8{
      if K == 1{
        return 16
      }
      if K == 2{
        return 17
      }
      if K == 3{
        return 18
      }
      u8::MAX
  }
 

  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      
      if (K-1) >= p{
       return f64::NAN
      }
      
       p=p.wrapping_sub(K+1);
       n=n.wrapping_sub(1);
       let res_mass = elementary_mass(K,0,1,1);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }   
}

impl InternalDecay for PositronAlpha{

  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
   
    
    let neu_mass = NEUTRINOMASS;
    let e_mass = ELECTRONMASS;
    let a_mass = ALPHAMASS;
    let d_mass = dghter.am();
    let t_mass = d_mass + neu_mass + e_mass + a_mass;
   
    let d_energy = q*((neu_mass + e_mass + a_mass)/t_mass);
    let neu_energy = q*((d_mass + e_mass + a_mass)/t_mass);
    let e_energy = q*((d_mass + neu_mass + a_mass)/t_mass);
    let a_energy = q*((d_mass + neu_mass + e_mass)/t_mass);
        
    *x = dghter;
        
    let particle_vector = vec![
    Particle::Lepton(Lepton::ElectronNeutrino(neu_energy)),
    Particle::AntiLepton(AntiLepton::Electron(e_energy)),
    Particle::Alpha(a_energy)
    ];
    
    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
 
 fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       p=p.wrapping_sub(3);
       n=n.wrapping_sub(1); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
     
  fn decay_index() -> u8{
      20
  }
 
  fn q_value(x: &Nuclide) -> f64{
     let (mut p,mut n) = x.proton_neutron();
      
      if 1 >= p || 3 >= n{
       return f64::NAN
      }
      
       p=p.wrapping_sub(1);
       n=n.wrapping_sub(3);
       let res_mass = elementary_mass(0,0,1,1)+ALPHAMASS;
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }    
}

impl InternalDecay for PositronFission{

  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
        let (energia, mut particle_vector) = PositronEmission::<1>::decay(x);
        let (s_energia, s_vector) = SpontaneousFission::decay(x);
        particle_vector.push(s_vector[0].clone());
        (energia + s_energia, particle_vector)
   }
   
   fn decay_result(x: &Nuclide) ->  Option<Nuclide>{
    let mut xclone = *x;
    Self::decay(&mut xclone);
    Some(xclone)
  }
   
  fn decay_index() -> u8{
      25
  }
  
  // FIXME  
  fn q_value(_x: &Nuclide) -> f64{
      f64::NAN
     }      
}

impl InternalDecay for SpontaneousFission{

  fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
        let mut izotop = x.proton_neutron();
        let rn = rand() as usize;

        let mut z = rn % izotop.0;

        if z == 0 {
            z = 1;
        }
        let start =SYMBOL_INDEX[z - 1].1 -z;
        let stop = SYMBOL_INDEX[z - 1].2 -z;
      
        let n = (rn%(stop-start-1) + start)%izotop.1;

        let decay = Nuclide::from_nucleons_unchecked(z,n);

        let de_mass = decay.am();
        izotop.0 -= z;
        izotop.1 -= n;
        let dghtr = Nuclide::from_nucleons_unchecked(izotop.0,izotop.1);
        let q = (x.am() - (dghtr.am() + de_mass)) * DALTON_MeV;
        let (big, small) = max(dghtr, decay);
        let mut particle_vector = vec![];
        let energia = q - (small.am() / (big.am() + small.am())) * q;
        particle_vector.push(Particle::Element(small, energia));
        x.change(big.nuclide_index());
        (q - energia, particle_vector)
   }
  
  fn decay_result(x: &Nuclide) ->  Option<Nuclide>{
        let  izotop = x.proton_neutron();
        let rn = rand() as usize;

        let mut z = rn % izotop.0;

        if z == 0 {
            z = 1;
        }
        let start =SYMBOL_INDEX[z - 1].1 -z;
        let stop = SYMBOL_INDEX[z - 1].2 -z;
      
        let n = (rn%(stop-start-1) + start)%izotop.1;

        Nuclide::from_nucleons(z, n)  
  }
  
  fn decay_index() -> u8{
      23
  }
  
  // FIXME  
  fn q_value(_x: &Nuclide) -> f64{
      f64::NAN
     }    
}

impl<const K: usize> InternalDecay for ClusterDecay<K>{

   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
     //cluster_decay(x,&Nuclide::assign(K))
     let c_nuclide = Nuclide::assign(K);
     let q = Self::q_value(x);
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
    
    let c_mass = c_nuclide.am();
    let d_mass = dghter.am();
    let t_mass = d_mass + c_mass;
   
    let d_energy = q*(c_mass/t_mass);
    let c_energy = q*(d_mass/t_mass);
        
    *x = dghter;
    
    let particle_vector = vec![Particle::Element(c_nuclide,c_energy)];

    (d_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
     
   }
  
  fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       let (p2,n2) = Nuclide::assign(K).proton_neutron();
       p=p.wrapping_sub(p2);
       n=n.wrapping_sub(n2); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      match K{
      59 => 36, // C-12
      61 => 26, // C-14
     129 => 27, // Ne-20
     131 => 37, // Ne-22
     133 => 28, // Ne-24
     135 => 49, // Ne-26
     176 => 32, // Mg-28
     178 => 38, // Mg-30
     115 => 33, // F-23
      94 => 34, // O-18
      96 => 35, // O-20
     223 => 30, // Si-32
     225 => 31, // Si-34
       _ => u8::MAX,
      }
  }
  
  fn q_value(x: &Nuclide) -> f64{
    
      let (mut p,mut n) = x.proton_neutron();
      let (cp, cn) = Nuclide::assign(K).proton_neutron();
      
      if cp >= p || cn >= n{
       return f64::NAN
      }
      
       p=p.wrapping_sub(cp);
       n=n.wrapping_sub(cn);
       let res_mass = elementary_mass(cp,cn,0,0);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }     
}

impl<const D: usize, const S: usize> InternalDecay for DoubleCluster<D,S>{
   fn decay(x : &mut Nuclide) -> (f64,Vec<Particle>){
   
     let d_nuclide = Nuclide::assign(D);
     let s_nuclide = Nuclide::assign(S);
     let q = Self::q_value(x);
     
    if q.is_nan(){
      return (0f64,vec![])
    }

    match Self::decay_result(x){
        Some(dghter) => {
    
    let d_mass = d_nuclide.am();
    let s_mass = s_nuclide.am();
    let prog_mass = dghter.am();
    let t_mass = prog_mass + d_mass + s_mass;
   
    let prog_energy = q*((s_mass+d_mass)/t_mass);
    
    let s_energy = q*((prog_mass + d_mass)/t_mass);
    
    let d_energy = q*((prog_mass + s_mass)/t_mass);
        
    *x = dghter;
    
    let particle_vector = vec![Particle::Element(d_nuclide,d_energy),Particle::Element(s_nuclide,s_energy) ];

    (prog_energy,particle_vector)
    
   }
   None => (0f64,vec![]),
   }
   }
   
     fn decay_result(x: &Nuclide) -> Option<Nuclide>{
       let (mut p,mut n) = x.proton_neutron();
       let (p2,n2) = Nuclide::assign(D).proton_neutron();
       let (p3,n3) = Nuclide::assign(S).proton_neutron();
       p=p.wrapping_sub(p2+p3);
       n=n.wrapping_sub(n2+n3); // if it wraps around from_nucleons will detect it
       Nuclide::from_nucleons(p,n)
   }
   
  fn decay_index() -> u8{
      if D == 132 && S == 134{
        return 29
      }
      if D == 134 && S == 132{
       return 29
      }
      u8::MAX
  }
  
  
  fn q_value(x: &Nuclide) -> f64{
      let (mut p,mut n) = x.proton_neutron();
      let (dp, dn) = Nuclide::assign(D).proton_neutron();
      let (sp, sn) = Nuclide::assign(S).proton_neutron();
      
      if (dp+sp) >= p || (dn+sn) >= n{
       return f64::NAN
      }
      
       p=p.wrapping_sub(dp+sp);
       n=n.wrapping_sub(dn+sn);
       let res_mass = elementary_mass(dp+sp,dn+sn,0,0);
       
       match Nuclide::from_nucleons(p,n){
        Some(dghtr) => {
         (x.am()-(dghtr.am()+res_mass))*DALTON_MeV
        }
        None => {
          let d_mass = theoretical_mass(p,n);
          (x.am()-(d_mass+res_mass))*DALTON_MeV
        }
      }
     }     
       
 }
 
  
impl InternalDecay for TotalDecay{

  fn decay(input : &mut Nuclide) -> (f64,Vec<Particle>){

            let x = decay_mode_idx(input).0;
            decay_select(input, x)
            
    } // end function
    
   fn decay_result(_x: &Nuclide) -> Option<Nuclide>{
     None
   } 
   
   fn decay_index() -> u8{
      254
   }
   
   
   // FIXME  
  fn q_value(_x: &Nuclide) -> f64{
      f64::NAN
     }   
}
//} // end trait

