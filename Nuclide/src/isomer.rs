use crate::nuclidedata::nuclidestruct::Nuclide;
use crate::particle::Particle;

use crate::atom::Atom;

/// Currently identical to Nuclide
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Isomer {
    gs: Nuclide,
    lvl: usize,
}

impl Isomer {
    pub fn new(x: &str) -> Option<Self> {
        Nuclide::new(x).map(|x| Self { gs: x, lvl: 0usize })
    }
}

impl Atom for Isomer {
    fn atomic_num(&self) -> u64 {
        self.gs.atomic_num()
    }

    fn identity(&self) -> String {
        self.gs.identity()
    }

    fn am(&self) -> f64 {
        self.gs.am()
    }

    fn am_kg(&self) -> f64 {
        self.gs.am_kg()
    }

    //fn am_ev(&self) -> f64;

    fn mass_deficit(&self) -> f64 {
        self.gs.mass_deficit()
    }

    fn mass_deficit_kg(&self) -> f64 {
        self.gs.mass_deficit_kg()
    }

    fn mass_deficit_j(&self) -> f64 {
        self.gs.mass_deficit_j()
    }

    fn mass_deficit_ev(&self) -> f64 {
        self.gs.mass_deficit_ev()
    }

    fn binding_energy(&self) -> f64 {
        self.gs.binding_energy()
    }

    fn binding_energy_j(&self) -> f64 {
        self.gs.binding_energy_j()
    }

    fn spin_parity(&self) -> (i8, i8) {
        self.gs.spin_parity()
    }

    fn electron_affinity(&self) -> f64 {
        self.gs.electron_affinity()
    }

    fn electron_affinity_ev(&self) -> f64 {
        self.gs.electron_affinity()
    }

    fn ionization_energies(&self, level: usize) -> Option<f64> {
        self.gs.ionization_energies(level)
    }

    fn ionization_energies_ev(&self, level: usize) -> Option<f64> {
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

    fn half_life(&self) -> f64 {
        self.gs.half_life()
    }

    fn mean_lifetime(&self) -> f64 {
        self.gs.mean_lifetime()
    }

    fn decay_mode(&self) -> String {
        self.gs.decay_mode()
    }

    fn decay_constant(&self) -> f64 {
        self.gs.decay_constant()
    }

    fn decay_time(&self, time: f64) -> bool {
        self.gs.decay_time(time)
    }

    fn static_decay(&mut self, time: f64) -> (f64, Vec<Particle>) {
        self.gs.static_decay(time)
    }

    fn decay(&mut self, time: f64) -> (f64, Vec<Particle>) {
        self.gs.decay(time)
    }
}
