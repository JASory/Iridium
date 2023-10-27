use crate::nstruct::Nuclide;
use crate::decay::decayimpl::DecayMode;
use crate::particle::Particle;

use crate::traits::{ChemElement,Isotope};


/// Currently identical to Nuclide
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EnergeticNuclide {
    gs: Nuclide,
    lvl: usize,
}

impl EnergeticNuclide {
    pub fn new(x: &str) -> Option<Self> {
        Nuclide::new(x).map(|x| Self { gs: x, lvl: 0usize })
    }
}

impl ChemElement for EnergeticNuclide{
    fn atomic_num(&self) -> u64 {
        self.gs.atomic_num()
    }


    fn am(&self) -> f64 {
        self.gs.am()
    }
    
    fn electron_affinity(&self) -> f64 {
        self.gs.electron_affinity()
    }


    fn ionization_energies(&self, level: usize) -> Option<f64> {
        self.gs.ionization_energies(level)
    }
    
    fn electronegativity(&self) -> f64 {
        self.gs.electronegativity()
    }

    fn mullikan_en(&self) -> f64 {
        self.gs.mullikan_en()
    }

    fn allen_en(&self) -> f64 {
        self.gs.allen_en()
    }

    fn pauling_en(&self) -> f64 {
        self.gs.pauling_en()
    }

    fn covalent_radii(&self, bond: usize) -> Option<f64> {
        self.gs.covalent_radii(bond)
    }

    fn ionic_radii(&self) -> f64 {
        self.gs.ionic_radii()
    }

    fn vdr_crystal(&self) -> f64 {
        self.gs.vdr_crystal()
    }

    fn vdr_isolated(&self) -> f64 {
        self.gs.vdr_isolated()
    }

}

impl Isotope for EnergeticNuclide{

   fn mass_deficit(&self) -> f64 {
        self.gs.mass_deficit()
    }
    
   fn binding_energy(&self) -> f64 {
        self.gs.binding_energy()
    } 
    
   fn half_life<T: DecayMode>(&self) -> f64 {
        self.gs.half_life::<T>()
    }

    fn mean_lifetime<T: DecayMode>(&self) -> f64 {
        self.gs.mean_lifetime::<T>()
    }

    fn decay_string(&self) -> String {
        self.gs.decay_string()
    }
    
    fn branching_ratio<T: DecayMode>(&self) -> f64{
        self.gs.branching_ratio::<T>()
    }

    fn decay_constant<T: DecayMode>(&self) -> f64 {
        self.gs.decay_constant::<T>()
    }
    
    fn decay_probability<T: DecayMode>(&self, time: f64) -> f64{
       self.gs.decay_probability::<T>(time)
    }

    fn decay_time<T: DecayMode>(&self, time: f64) -> bool {
        self.gs.decay_time::<T>(time)
    }
    
    fn daughter<T: DecayMode>(&self) -> Option<Self>{
        self.gs.daughter::<T>().map(|x| EnergeticNuclide{gs:x,lvl: 0usize})  
    }
    
    fn daughter_energetic<T: DecayMode>(&mut self) -> (f64,Vec<Particle>){
       self.gs.daughter_energetic::<T>()
    }
    
    fn decay_q<T: DecayMode>(&self) -> f64{
        self.gs.decay_q::<T>()
    } 

    fn decay<T: DecayMode>(&mut self, time: f64) -> (f64, Vec<Particle>) {
        self.gs.decay::<T>(time)
    }
}
