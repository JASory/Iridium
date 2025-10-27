use crate::traits::{ChemElement,Isotope};
use crate::nstruct::core::Nuclide;

use crate::constant::*;
use crate::mmodel::mass_model;
use crate::decay::DecayMode;
use crate::nuclidedata::half_life::HALF_LIFE;
use crate::nuclidedata::decay_chain::DECAY_CHAIN;
use crate::Particle;
use crate::rng::rand;

use crate::decay::is_mode_recorded;
use crate::decay::dhelper::decay_mode_idx;
use crate::decay::dhelper::decay_select;
use crate::decay::decayimpl::decayindex;





impl Isotope for Nuclide {

    
    fn mass_deficit(&self) -> f64 {
        let (p,n) = self.proton_neutron();
        ( PROTONMASS* p as f64
            + ELECTRONMASS * p as f64
            + NEUTRONMASS * n as f64)
            - self.am()
    }
    
    fn binding_energy(&self) -> f64 {
        let (z, a) = (
            self.proton_neutron().0,
            self.proton_neutron().0 + self.proton_neutron().1,
        );
        mass_model(a, z)
    }
    
   
   fn branching_ratio<T: DecayMode>(&self) -> f64{
      let idx = self.nuclide_index() * 6 + 5; 
      match decayindex::<T>(idx){
        Some(x) => x as f64/FLOAT_64,
        None => f64::NAN,
      }
    }
    
    fn half_life<T: DecayMode>(&self) -> f64 {
       let branch = self.branching_ratio::<T>();
       HALF_LIFE[self.nuclide_index()]/branch
       
    }
    
    fn mean_lifetime<T: DecayMode>(&self) -> f64 {
        //reciprocal of ln(2) average lifespan of a particle
        self.half_life::<T>() * std::f64::consts::LOG2_E
    }

    /// Approximation of decay constant
    fn decay_constant<T: DecayMode>(&self) -> f64 {
        self.mean_lifetime::<T>().recip()
    }
    
    
    /// Returns probability of decay 
    fn decay_probability<T: DecayMode>(&self, time: f64) -> f64{
           1.0 - (-self.decay_constant::<T>() * time).exp()
    }
    
    fn daughter_theoretical<T: DecayMode>(&self) -> Option<Self>{
    	T::decay_result(self)
    }

    //lowest probability is 1/u64::MAX
    ///Returns true if the nuclide would have decayed in the time given. The nuclide remains unchanged
    fn decay_time<T: DecayMode>(&self, time: f64) -> bool {
        let prob =
            ((1.0 - (-self.decay_constant::<T>() * time).exp()) * FLOAT_64) as u64;

        prob > rand()
    }

    /// Returns the probable decay modes  as a string
    fn decay_string(&self) -> String {
        let mut unu_alea =
            ((DECAY_CHAIN[self.nuclide_index() * 6] as f64 / FLOAT_64) * 100.0)
                .to_string();
        unu_alea.truncate(4);
        unu_alea.push_str("% ");
        let mut doua_alea = ((DECAY_CHAIN[self.nuclide_index() * 6 + 1] as f64
            / FLOAT_64)
            * 100.0)
            .to_string();
        doua_alea.truncate(4);
        doua_alea.push_str("% ");
        let mut trei_alea = ((DECAY_CHAIN[self.nuclide_index() * 6 + 2] as f64
            / FLOAT_64)
            * 100.0)
            .to_string();
        trei_alea.truncate(4);
        trei_alea.push_str("% ");
        let mut patru_alea = ((DECAY_CHAIN[self.nuclide_index() * 6 + 3] as f64
            / FLOAT_64)
            * 100.0)
            .to_string();
        patru_alea.truncate(4);
        patru_alea.push_str("% ");
        let mut cinci_alea = ((DECAY_CHAIN[self.nuclide_index() * 6 + 4] as f64
            / FLOAT_64)
            * 100.0)
            .to_string();
        cinci_alea.truncate(4);
        cinci_alea.push_str("% ");

        let mut decay_str = vec![];

        let decay_vector = DECAY_CHAIN[self.nuclide_index() * 6 + 5].to_be_bytes();

        for i in decay_vector[..5].iter() {
            match i {
                1 => decay_str.push("α; "),
                2 => decay_str.push("p; "),
                3 => decay_str.push("2p; "),
                4 => decay_str.push("n; "),
                5 => decay_str.push("2n; "),
                6 => decay_str.push("EC; "),
                7 => decay_str.push("2EC; "),
                8 => decay_str.push("β− + p; "),
                9 => decay_str.push("β+; "),
                10 => decay_str.push("2β+; "),
                11 => decay_str.push("β−; "),
                12 => decay_str.push("2β−; "),
                13 => decay_str.push("β− + n; "),
                14 => decay_str.push("β− + 2n; "),
                15 => decay_str.push("β− + 3n; "),
                16 => decay_str.push("β+ + p; "),
                17 => decay_str.push("β+ + 2p; "),
                18 => decay_str.push("β+ + 3p; "),
                19 => decay_str.push("β− + α; "),
                20 => decay_str.push("β+ + α; "),
                21 => decay_str.push("β− + d; "),
                22 => decay_str.push("β− + t; "),
                23 => decay_str.push("SF; "),
                24 => decay_str.push("β− + SF; "),
                25 => decay_str.push("β+ + SF; "),
                26 => decay_str.push("C-14; "),
                27 => decay_str.push("Ne-20; "),
                28 => decay_str.push("Ne-24; "),
                29 => decay_str.push("Ne-20 + Ne-24; "),
                30 => decay_str.push("Si-32; "),
                31 => decay_str.push("Si-34; "),
                32 => decay_str.push("Mg-28; "),
                33 => decay_str.push("F-23; "),
                34 => decay_str.push("O-18; "),
                35 => decay_str.push("O-20; "),
                36 => decay_str.push("C-12; "),
                37 => decay_str.push("Ne-22; "),
                38 => decay_str.push("Mg-30; "),
                39 => decay_str.push("Mg-28 + Mg-30; "),
                40 => decay_str.push("d + n; "),
                41 => decay_str.push("t + n; "),
                42 => decay_str.push("α + n; "),
                43 => decay_str.push("α + 2n; "),
                44 => decay_str.push("α + 3n; "),
                45 => decay_str.push("β− + p; "),
                46 => decay_str.push("β− + 2p; "),
                47 => decay_str.push("β− + 3p; "),
                48 => decay_str.push("3p; "),
                49 => decay_str.push("Ne-26; "),
                50 => decay_str.push("β− + 4n; "),
                _ => decay_str.push("Null"),
            }
        }
        let mut decayvec = vec![];
        decayvec.push(unu_alea);
        decayvec.push(decay_str[0].to_string());
        decayvec.push(doua_alea);
        decayvec.push(decay_str[1].to_string());
        decayvec.push(trei_alea);
        decayvec.push(decay_str[2].to_string());
        decayvec.push(patru_alea);
        decayvec.push(decay_str[3].to_string());
        decayvec.push(cinci_alea);
        decayvec.push(decay_str[4].to_string());

        if decayvec[0] == "0% " {
            "Stable".to_string()
        } else {
            match decayvec.iter().position(|r| r == "Null") {
                Some(x) => decayvec.truncate(x - 1),
                None => decayvec.truncate(10),
            }
            decayvec.join("")
        }
    }
    
    fn daughter_energetic<T: DecayMode>(&mut self) -> (f64,Vec<Particle>){
        T::decay(self)
    }
    
    
    fn daughter<T: DecayMode>(&self) -> Option<Self>{
        if is_mode_recorded::<T>(self){
          return T::decay_result(self)
        }
        None
    }
    
    /// Q-value (total energy) of a nuclear decay, regardless of whether it is observed
    /// # NAN
    /// Returns NAN if this decay mode results in a nonexistent nuclide
    fn decay_q<T: DecayMode>(&self) -> f64{
        T::q_value(self)
    } 
    
    /**
    Returns the name and isotope number of the nuclide

       ```
       use ::Nuclide::{Nuclide, Isotope,::decay::TotalDecay};
       let mut uranium = "U-238".parse::<Nuclide>().unwrap();

      // total particle energy of the nuclide and vector of decay particles
       let (particle_energy,particle_vector) = uranium.decay::<TotalDecay>(5E+20);

        // final nuclide in the U-238 decay series
        assert_eq!(uranium.to_string(), "Pb-206");
       ```
    */
    fn decay<T: DecayMode>(&mut self, mut time: f64) -> (f64, Vec<Particle>) {
        let mut total_energy = 0f64;
        let mut particlevec = vec![];
        
        if T::decay_index() == 254u8{
       
        loop {
        
        // Obtain index of decay branch
        let (b,b_idx) = decay_mode_idx(self);
       
        let idx = self.nuclide_index() * 6usize + b_idx as usize; 

        let ratio = (DECAY_CHAIN[idx] as f64)/FLOAT_64;
        
        let mean_time = (HALF_LIFE[self.nuclide_index()]/ratio) * std::f64::consts::LOG2_E; 

        if mean_time >= time || mean_time == f64::INFINITY{
          return (total_energy,particlevec)
        }
        
        let (energy, p_vector) = decay_select(self,b);
        particlevec.extend_from_slice(&p_vector[..]);
        total_energy+=energy;
        time-=mean_time;
        
        }
        
        } // end if
      
        loop{
         
             if !is_mode_recorded::<T>(self){
                return (total_energy,particlevec)
             }
      
             let mean_time = self.mean_lifetime::<T>();
             
             if mean_time >= time || mean_time.is_infinite() || mean_time.is_nan(){
                return (total_energy,particlevec)
             }
             
             let (energy, p_vector) = T::decay(self);
          
             particlevec.extend_from_slice(&p_vector[..]);
             total_energy+=energy;
             
             time-=mean_time;
          } 
    }
    
 }   
