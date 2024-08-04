/*
   Nuclide
*/

use crate::traits::{Atom,ChemElement};
use crate::constant::*;

use crate::nuclidedata::index::*

use crate::particle::*;

use crate::rng::rand;

use crate::nuclidedata::atomic_mass::ATOMIC_MASS;
use crate::nuclidedata::elemental::*;
use crate::nuclidedata::index::{NAME,SYMBOL};
use crate::nuclidedata::ionization::IONIZATION_ENERGIES;
use crate::nuclidedata::spinparity::SPIN_PARITY;


use crate::decay::decayimpl::DecayMode;
use crate::decay::decayimpl::decayindex;
use crate::decay::dhelper::{decay_select,decay_mode_idx};
use crate::decay::is_mode_recorded;


use crate::decay::types::*;
use crate::mmodel::mass_model;


use crate::nuclidedata::decay_chain::DECAY_CHAIN;
use crate::nuclidedata::half_life::HALF_LIFE;





/// Efficient representation of nuclide
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nuclide {
     idx: usize,
}

impl std::fmt::Display for Nuclide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iso = self.isotope();
        write!(f, "{}-{}",SYMBOL[iso.0 - 1], iso.1)
    }
}

impl Nuclide {

/// Initializes a new Nuclide from  a string representation.
/// Currently the input must be of the form {Symbol}-{Nucleon count}
/// ```
///         use ::Nuclide::Nuclide;
///         use ::Nuclide::Atom;
///          
///         let u235 = Nuclide::new("U-235").unwrap();
///         assert_eq!(u235.to_string(),"U-235");
/// ```
    pub fn new(input: &str) -> Option<Nuclide> {
    
        let z = input.split('-').collect::<Vec<&str>>();
        if z.len() != 2 {
            return None;
        }

        let isotope: usize = match z[1].parse::<usize>() {
            Ok(x) => x,
            Err(_) => usize::MAX,
        };

        if isotope == usize::MAX {
            return None;
        }

        let x = z[0];

        match SYMBOL.iter().position(|y| y == &x) {
            Some(x) => {
                if isotope >= SYMBOL_INDEX[x].1 && isotope <= SYMBOL_INDEX[x].2 {
                    Some(Nuclide {
                        idx: SYMBOL_INDEX[x].0 + isotope - SYMBOL_INDEX[x].1,
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
    
    /// In : proton, neutron
    /// Out: Nuclide  
    pub fn from_nucleons_unchecked(z: usize, n: usize) -> Self{
         Nuclide{idx: SYMBOL_INDEX[z - 1].0 - (SYMBOL_INDEX[z - 1].1 - z) + n}
    }

    /// In: proton, neutron 
    /// # None
    /// Returns None if the Nuclide doesn't exist
    pub fn from_nucleons(z: usize, n: usize) -> Option<Self> {
        if z == 0 || z > 118 {
            return None;
        }
        let n_lo = SYMBOL_INDEX[z - 1].1 - z;
        let n_hi = SYMBOL_INDEX[z - 1].2 - z;
        if n >= n_lo && n <= n_hi {
            return Some(Nuclide::from_nucleons_unchecked(z, n));
        }
        None
    }
    
    /// Construct a nuclide from the unique index. Avoid direct use as no checks are performed to ensure that it is valid
    pub fn assign(idx: usize) -> Self {
        Self { idx }
    }
    
   /// Transforms a nuclide from the unique index.
    pub fn change(&mut self, idx: usize) {
        self.idx = idx;
    }
    
    /// Returns the approximate mass and binding energy of a nuclide, theorectical or real, using the Bethe-Weizacker liquid-drop approximation.
    pub fn create(z: usize, n: usize) -> (f64, f64) {
        let b_e = mass_model(z + n, z);
        (
            (z as f64 * PROTONMASS + n as f64 * NEUTRONMASS) - (b_e / 931.36808885),
            b_e,
        )
    }
    
    /// Returns the underlying unique value. Can be used in conjunction with "assign" and "change" to rapidly create or
    /// convert nuclides without decay
    pub fn nuclide_index(&self) -> usize {
        self.idx
    }

    /// Returns the atomic number and the nucleon count
    pub fn isotope(&self) -> (usize, usize) {
        let element = self.atomic_num();
        (
            element as usize,
            (self.idx - SYMBOL_INDEX[element as usize - 1].0)
                + SYMBOL_INDEX[element as usize - 1].1,
        )
    }

    ///Returns the element name.     
    pub fn element_name(&self) -> String {
        NAME[self.atomic_num() as usize - 1].to_string()
    }

    ///Returns the proton and neutron count
    pub fn proton_neutron(&self) -> (usize, usize) {
        (self.isotope().0, self.isotope().1 - self.isotope().0)
    }

    /// Approximate neutron separation energy
    pub fn neutron_separation(&self) -> f64 {
        let (z, n) = self.proton_neutron();
        mass_model(z + n, z) - mass_model(z + n - 1, z)
    }
    
    /// Approximate proton separation energy
    pub fn proton_separation(&self) -> f64 {
        let (z, n) = self.proton_neutron();
        mass_model(z + n, z) - mass_model(z + n - 1, z - 1)
    }
    
    /// returns a vector of all isotopes of the element
    pub fn isotope_list(&self) -> Vec<Self> {
        let proton = self.atomic_num() as usize;
        let start = SYMBOL_INDEX[proton - 1].0;
        let delta = SYMBOL_INDEX[proton - 1].2 - SYMBOL_INDEX[proton - 1].1;
        let mut n_vector = vec![];
        for i in 0..delta + 1 {
            n_vector.push(Nuclide::assign(start + i))
        }
        n_vector
    }

    /// Returns the nuclide (if it exists) that has swapped proton-neutron count
    pub fn mirror(&self) -> Option<Self> {
        let (z, n) = self.proton_neutron();
        Nuclide::from_nucleons(n, z)
    }
    /*
      isobar = permutations of z+1,n-1 and z-1,n+1

    Iterate through the symbollist


    z-(z-i) n+(z-i)

    check that n+(z-i) is valid for the point z-(z-i)
      */
    /// Produces a vector of all nuclides sorted by atomic number, e.g all hydrogen isotopes, all helium isotopes, ...
    pub fn list() -> Vec<Self> {
        (0..ATOMIC_MASS.len())
            .map(Nuclide::assign)
            .collect::<Vec<Self>>()
    }
    
    /// Produces a list of all nuclides that share the same atomic number as the selected nuclide
    pub fn isobar_list(&self) -> Vec<Self> {
        let table = Nuclide::list();
        let mut isobars = vec![];
        let a = self.proton_neutron().0 + self.proton_neutron().1;
        for i in table {
            let (z, n) = i.proton_neutron();
            if (z + n) == a {
                isobars.push(i)
            }
        }
        isobars
    }

    /// Produces a list of nuclides that share the same number of neutrons
    pub fn isotone_list(&self) -> Vec<Self> {
        let n = self.proton_neutron().1;

        let mut n_vector = vec![];
        for (idx, el) in SYMBOL_INDEX.iter().enumerate() {
            let n_lo = el.1 - (idx + 1);
            let n_hi = el.2 - (idx + 1);
            if n >= n_lo && n <= n_hi {
                n_vector.push(Nuclide::from_nucleons_unchecked(idx + 1, n))
            }
        }
        n_vector
    }
    
    /// Probability of the provided Decay mode being taken
    /// # NAN
    /// If Decay Mode is not observed return NAN
    pub fn branching_ratio<T: DecayMode>(&self) -> f64{
      let idx = self.nuclide_index() * 6 + 5; 
      match decayindex::<T>(idx){
        Some(x) => x as f64/FLOAT_64,
        None => f64::NAN,
      }
    }
    
    
    /// Returns the daughter nuclide, regardless of whether it has been observed
    /// # None
    /// If the decay result is not a known nuclide, returns None
    pub fn daughter_theoretical<T: DecayMode>(&self) -> Option<Self>{
    	T::decay_result(self)
    }
    
    /*
    /// List of Daughter isotopes
    pub fn daughter_list(&self) -> Vec<Self>{
        let decay_vector = DECAY_CHAIN[self.nuclide_index() * 6 + 5].to_be_bytes();
        decay_vector[0] == 
        
    }
   */
    pub fn decay_probability<T: DecayMode>(&self, time: f64) -> f64{
           1.0 - (-self.decay_constant::<T>() * time).exp()
    }
    

    
}

impl Atom for Nuclide {

    fn atomic_num(&self) -> u64 {
        let mut count: u64 = 0;
        for i in SYMBOL_INDEX {
            if i.0 > self.idx {
                break;
            }
            count += 1;
        }
        count
    }
    
    /// Returns the atomic mass in daltons
    fn am(&self) -> f64 {
        ATOMIC_MASS[self.idx]
    }
    
    ///Returns the atomic mass in kilograms
    fn am_kg(&self) -> f64 {
        self.am() * DALTON_KG
    }

    fn mass_deficit(&self) -> f64 {
        let (p,n) = self.proton_neutron();
        ( PROTONMASS* p as f64
            + ELECTRONMASS * p as f64
            + NEUTRONMASS * n as f64)
            - self.am()
    }

    fn mass_deficit_kg(&self) -> f64 {
        self.mass_deficit() * DALTON_KG
    }
    /// Mass deficit as MeV   
    fn mass_deficit_ev(&self) -> f64 {
        self.mass_deficit() * DALTON_MeV
    }

    fn mass_deficit_j(&self) -> f64 {
        self.mass_deficit_ev() * 1.602177E-19
    }

    ///Returns the binding energy. Utilizing the mass model
    fn binding_energy(&self) -> f64 {
        let (z, a) = (
            self.proton_neutron().0,
            self.proton_neutron().0 + self.proton_neutron().1,
        );
        mass_model(a, z)
    }

    fn binding_energy_j(&self) -> f64 {
        self.binding_energy() * 1.602176634E-19
    }

    ///Returns the isospin and parity in the form of a i8 pair, one of which is negatively signed for - parity
    fn spin_parity(&self) -> (i8, i8) {
        SPIN_PARITY[self.idx]
    }

    ///Returns electron affinity
    fn electron_affinity(&self) -> f64 {
        ELECTRON_AFFINITY[self.atomic_num() as usize - 1]
    }
    fn electron_affinity_ev(&self) -> f64 {
        self.electron_affinity() * 0.010364265
    }
    ///Returns the ionization energies for all known levels. Values are in kj/mol.
    fn ionization_energies(&self, level: usize) -> Option<f64> {
        if self.atomic_num() > 110 {
            None
        } else if level > 0 && level < self.atomic_num() as usize + 1 {
            Some(
                IONIZATION_ENERGIES[((((self.atomic_num() * (self.atomic_num() + 1)) >> 1)
                    - self.atomic_num())
                    + level as u64
                    - 1) as usize],
            )
        } else {
            None
        }
    }

    fn ionization_energies_ev(&self, level: usize) -> Option<f64> {
        self.ionization_energies(level).map(|x| x * 0.010364265f64)
    }
    
    /// Returns the thermochemical electronegativity as calculated by Oganov and Tantardini. Currently the best predictor of experimental values
    fn electronegativity(&self) -> f64 {
        THERMOCHEMICAL_ELECTRO_NEGATIVE[self.atomic_num() as usize - 1]
    }
    ///Returns the Mullikan, or absolute, electronegativity in kj/mol
    fn mullikan_en(&self) -> f64 {
        (self.ionization_energies(1).unwrap() + ELECTRON_AFFINITY[self.atomic_num() as usize - 1])
            * 1.97E-3
            + 0.19
    }
    ///Allen Electronegativity
    fn allen_en(&self) -> f64 {
        ALLEN_ELECTRO[self.atomic_num() as usize - 1]
    }
    ///Pauling Electronegativity. A poor fit for experimental values, however it is here for completeness
    fn pauling_en(&self) -> f64 {
        PAULING_ELECTRO[self.atomic_num() as usize - 1]
    }
    /// Radius in Single-double-and-triple covalent bonds
    fn covalent_radii(&self, bond: usize) -> Option<f64> {
        if bond > 0 && bond < 4 {
            Some(COVALENT_RADII[(self.atomic_num() as usize - 1) * 3 + bond - 1])
        } else {
            None
        }
    }
    /// ionic radii     
    fn ionic_radii(&self) -> f64 {
        IONIC_RADII[self.atomic_num() as usize - 1]
    }

    ///Returns the Van Der Waal radius in crystalline structures. Values are in meters.
    fn vdr_crystal(&self) -> f64 {
        VAN_DER_WAAL_CRYSTAL[self.atomic_num() as usize - 1]
    }
    ///Returns the Van Der Waal radius of isolated atoms
    fn vdr_isolated(&self) -> f64 {
        VAN_DER_WAAL_ISOLATED[self.atomic_num() as usize - 1]
    }

    /// Half-life in seconds
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

    //lowest probability is 1/u64::MAX
    ///Returns true if the nuclide would have decayed in the time given. The nuclide remains unchanged
    fn decay_time<T: DecayMode>(&self, time: f64) -> bool {
        let prob =
            ((1.0 - (-self.decay_constant::<T>() * time).exp()) * FLOAT_64) as u64;

        prob > rand()
    }

    /// Returns the probable decay modes  as a string
    fn decay_mode(&self) -> String {
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

        let mut decay_string = vec![];

        let decay_vector = DECAY_CHAIN[self.nuclide_index() * 6 + 5].to_be_bytes();

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
                19 => decay_string.push("β− + α; "),
                20 => decay_string.push("β+ + α; "),
                21 => decay_string.push("β− + d; "),
                22 => decay_string.push("β− + t; "),
                23 => decay_string.push("SF; "),
                24 => decay_string.push("β− + SF; "),
                25 => decay_string.push("β+ + SF; "),
                26 => decay_string.push("C-14; "),
                27 => decay_string.push("Ne-20; "),
                28 => decay_string.push("Ne-24; "),
                29 => decay_string.push("Ne-20 + Ne-24; "),
                30 => decay_string.push("Si-32; "),
                31 => decay_string.push("Si-34; "),

                _ => decay_string.push("Null"),
            }
        }
        let mut decayvec = vec![];
        decayvec.push(unu_alea);
        decayvec.push(decay_string[0].to_string());
        decayvec.push(doua_alea);
        decayvec.push(decay_string[1].to_string());
        decayvec.push(trei_alea);
        decayvec.push(decay_string[2].to_string());
        decayvec.push(patru_alea);
        decayvec.push(decay_string[3].to_string());
        decayvec.push(cinci_alea);
        decayvec.push(decay_string[4].to_string());

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
       use ::Nuclide::{Atom,Nuclide};
       use ::Nuclide::decay::TotalDecay;
       let mut uranium = Nuclide::new("U-238").unwrap();

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
