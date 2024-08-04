use crate::traits::ChemElement;
use crate::nstruct::core::Nuclide;
use crate::nuclidedata::atomic_mass::ATOMIC_MASS;
use crate::nuclidedata::elemental::*;
use crate::nuclidedata::ionization::IONIZATION_ENERGIES;

//use crate::constant::*;
use crate::nuclidedata::index::SYMBOL_INDEX;



impl ChemElement for Nuclide{
    fn atomic_num(&self) -> u64 {
        SYMBOL_INDEX.partition_point(|&tup| tup.0 <= self.nuclide_index()) as u64
    }
    
    /// Returns the atomic mass in daltons
    fn am(&self) -> f64 {
        ATOMIC_MASS[self.nuclide_index()]
    }

    fn electron_affinity(&self) -> f64 {
        ELECTRON_AFFINITY[self.atomic_num() as usize - 1]
    }

    fn ionization_energies(&self, level: usize) -> Option<f64> {
        if self.atomic_num() > 110 || level == 0 || level > self.atomic_num() as usize {
            return None;
        }

        Some(
            IONIZATION_ENERGIES[((((self.atomic_num() * (self.atomic_num() + 1)) >> 1)
                - self.atomic_num())
                + level as u64
                - 1) as usize],
        )
    }
    
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
         
    fn ionic_radii(&self) -> f64 {
        IONIC_RADII[self.atomic_num() as usize - 1]
    }

    
    fn vdr_crystal(&self) -> f64 {
        VAN_DER_WAAL_CRYSTAL[self.atomic_num() as usize - 1]
    }
    
    
    fn vdr_isolated(&self) -> f64 {
        VAN_DER_WAAL_ISOLATED[self.atomic_num() as usize - 1]
    } 
    
}
