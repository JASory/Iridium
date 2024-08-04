use crate::particle::Particle;
use crate::decay::decayimpl::DecayMode;

/// General chemical element properties
pub trait ChemElement: Clone{
    /// Atomic number
    fn atomic_num(&self) -> u64;
    /// Atomic mass in Daltons
    fn am(&self) -> f64;
    /// Electron affinity in kj/mol
    fn electron_affinity(&self) -> f64;    
    ///Returns the ionization energies for all known levels. Values are in kj/mol
    fn ionization_energies(&self, level: usize) -> Option<f64>;
    /// Returns Oganov-Tantardini values, the current best evaluation
    fn electronegativity(&self) -> f64;
    /// Mullikan electronegativity
    fn mullikan_en(&self) -> f64;
    /// Allen electronegativity
    fn allen_en(&self) -> f64;
    /// Pauling electronegativity
    fn pauling_en(&self) -> f64;
    /// Covalent radii of the first three bonds
    fn covalent_radii(&self, bond: usize) -> Option<f64>;
    /// Ionic radii
    fn ionic_radii(&self) -> f64;
    /// Van der Waal radius in crystalline structure
    fn vdr_crystal(&self) -> f64;
    /// Van der Waal radius in isolated atoms
    fn vdr_isolated(&self) -> f64;
    
}

/// Nuclear properties that vary between isotopes
pub trait Isotope: ChemElement{

    /// Mass defect or the difference between the empirical mass and the mass of the constituents, in Daltons
    fn mass_deficit(&self) -> f64;
    
    /// Binding energy in MeV
    fn binding_energy(&self) -> f64;
    
    /// Half-life of nuclide/isomer in seconds. TotalDecay mode returns the half-life, all other modes return  the partial half-life 
    /// # Nan
    /// Particle is unstable but the selected decay mode is not supported
    /// # Inf
    /// Particle is stable
    fn half_life<T: DecayMode>(&self) -> f64;
    
    /// The mean lifetime of nuclide in seconds
    fn mean_lifetime<T: DecayMode>(&self) -> f64;
    
    /// Returns the probable decay modes as a string
    fn decay_string(&self) -> String;
    
    /// Returns the daughter with decay energy
    fn daughter_energetic<T: DecayMode>(&mut self) -> (f64, Vec<Particle>);
    
    /// Returns the daughter nuclide 
    /// # None
    /// If nuclide has not been observed to decay by the mode, returns None
    fn daughter<T: DecayMode>(&self) -> Option<Self>;
    
    /// Returns the daughter nuclide,regardless of whether it has been observed 
    /// # None
    /// If impossible to decay by the provided mode
    fn daughter_theoretical<T: DecayMode>(&self) -> Option<Self>;
    
    /// Probability of the provided Decay mode being taken
    /// # NAN
    /// If Decay Mode is not observed return NAN
    fn branching_ratio<T: DecayMode>(&self) -> f64;
    
    /// Decay constant in seconds of nuclide/isomer. TotalDecay mode returns the decay constant, all other modes return  the partial decay constant 
    /// # Nan
    /// Particle is unstable but the selected decay mode is not supported
    /// # Inf
    /// Particle is stable
    fn decay_constant<T: DecayMode>(&self) -> f64;
    
    /// Returns the probability of the nuclide to decay after the time in seconds provided
    fn decay_probability<T: DecayMode>(&self, time: f64) -> f64;
    
    /// Checks if nuclide probably decay in the selected time.
    fn decay_time<T: DecayMode>(&self, time: f64) -> bool;

    /// Continously performs decay throughout the time selected, collecting all particles into a vector with decay energies.
    fn decay<T: DecayMode>(&mut self, time: f64) -> (f64, Vec<Particle>);
    
    /// Q-value (total energy) of a nuclear decay, regardless of whether it is observed
    /// # NAN
    /// Returns NAN if this decay mode results in a nonexistent nuclide
    fn decay_q<T: DecayMode>(&self) -> f64;
}
