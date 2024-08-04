use crate::nuclidedata::index::{NAME,SYMBOL,SYMBOL_INDEX};
use crate::mmodel::mass_model;
use crate::constant::*;
use crate::traits::{ChemElement};
use crate::nuclidedata::spinparity::SPIN_PARITY;





/// Efficient representation of nuclide
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nuclide {
     idx: usize,
}

impl std::fmt::Display for Nuclide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (z, n) = self.isotope();
        write!(f, "{}-{}", SYMBOL[z - 1], n)
    }
}

impl std::str::FromStr for Nuclide{

     type Err = &'static str;
     
  fn from_str(input: &str) -> Result<Self,Self::Err>{
      match Nuclide::new(input){
         Some(x) => Ok(x), 
         None => Err("Parsing error"),
      }
     } 
  
  }

impl Nuclide {

/// Initializes a new Nuclide from  a string representation.
/// Currently the input must be of the form {Symbol}-{Nucleon count}
/// ```
///         use ::Nuclide::Nuclide;
///          
///         let u235 = Nuclide::new("U-235").unwrap();
///         assert_eq!(u235.to_string(),"U-235");
/// ```
    pub fn new(input: &str) -> Option<Nuclide> {
        let [symbol, nucleons_str]: [&str; 2] = input.split('-')
            .collect::<Vec<&str>>()
            .try_into()
            .ok()?;

        let nucleons = nucleons_str.parse::<usize>().ok()?;

        let z_offset = SYMBOL.iter().position(|&element_str| element_str == symbol)?;
        let (start, a_min, a_max) = SYMBOL_INDEX[z_offset];
        (a_min..=a_max).contains(&nucleons)
            .then_some(Nuclide {idx: start + nucleons - a_min})
    }
    
    /// In : proton, neutron
    /// Out: Nuclide  
    pub fn from_nucleons_unchecked(protons: usize, neutrons: usize) -> Self{
        let (start, a_min, _) = SYMBOL_INDEX[protons - 1];
        let a = protons + neutrons;
        Nuclide {idx: start + a - a_min}
    }

    /// In: proton, neutron 
    /// Returns None if the Nuclide doesn't exist
    pub fn from_nucleons(protons: usize, neutrons: usize) -> Option<Self> {
        if protons == 0 {
            return None;
        }

        let (start, a_min, a_max) = *SYMBOL_INDEX.get(protons - 1)?;
        let a = protons + neutrons;
        (a_min..=a_max).contains(&a)
            .then_some(Nuclide {idx: start + a - a_min})
    }
    
    /// Construct a nuclide from the unique index. Avoid direct use as no checks are performed to ensure that it is valid
    pub fn assign(idx: usize) -> Self {
        Self { idx }
    }
    
   /// Transforms a nuclide from the unique index.
    pub fn change(&mut self, idx: usize) {
        self.idx = idx;
    }
    
    /// Returns the approximate mass and binding energy of a nuclide, theorectical or real, using the DZ-10 mass model.
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
        let z = self.atomic_num() as usize;
        let (start, a_min, _) = SYMBOL_INDEX[z - 1];
        let a = self.idx - start + a_min;
        (z, a)
    }

    ///Returns the element name.     
    pub fn element_name(&self) -> String {
        NAME[self.atomic_num() as usize - 1].to_string()
    }

    ///Returns the proton and neutron count
    pub fn proton_neutron(&self) -> (usize, usize) {
        let (z, a) = self.isotope();
        (z, a - z)
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
    /// Produces an iterator of all nuclides sorted by atomic number, e.g all hydrogen isotopes, all helium isotopes, ...
    pub fn list() -> Vec<Self> {
        (0..NUCLIDE_COUNT)
            .map(Nuclide::assign)
            .collect::<Vec<Self>>()
    }
    
    /// Produces a list of all nuclides that share the same atomic number as the selected nuclide
    pub fn isobar_list(&self) -> Vec<Self> {
        let table = Nuclide::list();
        let mut isobars = vec![];
        let a = self.proton_neutron().0 + self.proton_neutron().1;
        for i in table{
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

    ///Returns the isospin and parity in the form of a i8 pair, one of which is negatively signed for - parity
    pub fn spin_parity(&self) -> (i8, i8) {
        SPIN_PARITY[self.idx]
    }
    

    
}

