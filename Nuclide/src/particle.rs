use Pion::baryon::AntiBaryon;
use Pion::baryon::Baryon;
use Pion::lepton::AntiLepton;
use Pion::lepton::Lepton;
//use Pion::photon::Photon;

use crate::nuclidedata::nuclidestruct::Nuclide;

use crate::atom::Atom;

pub const PROTONMASS: f64 = 1.007276466621f64;
pub const NEUTRONMASS: f64 = 1.00866491588f64;
pub const ELECTRONMASS: f64 = 5.48579909070E-4;
pub const ALPHAMASS: f64 = 4.001506179127f64;
pub const TRITONMASS: f64 = 3.015500980600021;
pub const DEUTERONMASS: f64 = 2.013553389273434;

pub const NEUTRINOMASS: f64 = 1.288738236E-10;
/// Generalized representation of potential decay particles
#[derive(Debug, Clone)]
pub enum Particle {
    /// Photon and frequency
    Photon(f64),
    /// Leptons including Electrons, Neutrinos, and AntiNeutrinos  
    Lepton(Lepton),
    /// AntiLeptons  
    AntiLepton(AntiLepton),
    /// Proton or Neutron  
    Baryon(Baryon),
    /// AntiBaryons   
    AntiBaryon(AntiBaryon),
    /// Alpha particle    
    Alpha(f64),
    /// Deuterium nucleus    
    Deuteron(f64),
    /// Tritium nucleus     
    Triton(f64),
    /// Elemental nucleus    
    Element(Nuclide, f64),
    // IsoElement(Isomer,f64),
}

impl Particle {
    pub fn identity(&self) -> String {
        match self {
            Particle::Photon(x) => x.to_string() + " MeV γ",
            Particle::Lepton(x) => match x {
                Lepton::Electron(y) => y.to_string() + " MeV β-",
                Lepton::Muon(y) => y.to_string() + " MeV μ",
                Lepton::Tau(y) => y.to_string() + " MeV τ",
                Lepton::ElectronNeutrino(y) => y.to_string() + " MeV νe",
                Lepton::MuNeutrino(y) => y.to_string() + " MeV νμ",
                Lepton::TauNeutrino(y) => y.to_string() + " MeV ντ",
            },
            Particle::AntiLepton(x) => match x {
                AntiLepton::Electron(y) => y.to_string() + " MeV β+",
                AntiLepton::Muon(y) => y.to_string() + " MeV -μ",
                AntiLepton::Tau(y) => y.to_string() + " MeV -τ",
                AntiLepton::ElectronNeutrino(y) => y.to_string() + " MeV -ve",
                AntiLepton::MuNeutrino(y) => y.to_string() + " MeV -νμ",
                AntiLepton::TauNeutrino(y) => y.to_string() + " MeV -ντ",
            },

            Particle::Baryon(x) => match x {
                Baryon::Proton(y) => y.to_string() + " MeV p",
                Baryon::Neutron(y) => y.to_string() + " MeV n",
            },
            Particle::AntiBaryon(x) => match x {
                AntiBaryon::Proton(y) => y.to_string() + " MeV -p",
                AntiBaryon::Neutron(y) => y.to_string() + " MeV -n",
            },
            Particle::Alpha(x) => x.to_string() + " MeV α",
            Particle::Deuteron(x) => x.to_string() + " MeV d",
            Particle::Triton(x) => x.to_string() + " MeV t",
            Particle::Element(x, z) => z.to_string() + &x.identity(),
        }
    }
    #[allow(dead_code)]
    fn energy(&self) -> f64 {
        match self {
            Particle::Photon(x) => *x,
            Particle::Lepton(x) => match x {
                Lepton::Electron(y) => *y,
                Lepton::Muon(y) => *y,
                Lepton::Tau(y) => *y,
                Lepton::ElectronNeutrino(y) => *y,
                Lepton::MuNeutrino(y) => *y,
                Lepton::TauNeutrino(y) => *y,
            },
            Particle::AntiLepton(x) => match x {
                AntiLepton::Electron(y) => *y,
                AntiLepton::Muon(y) => *y,
                AntiLepton::Tau(y) => *y,
                AntiLepton::ElectronNeutrino(y) => *y,
                AntiLepton::MuNeutrino(y) => *y,
                AntiLepton::TauNeutrino(y) => *y,
            },

            Particle::Baryon(x) => match x {
                Baryon::Proton(y) => *y,
                Baryon::Neutron(y) => *y,
            },
            Particle::AntiBaryon(x) => match x {
                AntiBaryon::Proton(y) => *y,
                AntiBaryon::Neutron(y) => *y,
            },
            Particle::Alpha(x) => *x,
            Particle::Deuteron(x) => *x,
            Particle::Triton(x) => *x,
            Particle::Element(_, z) => *z,
        }
    }
}
