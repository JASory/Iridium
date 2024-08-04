use Pion::AntiBaryon;
use Pion::Baryon;
use Pion::AntiLepton;
use Pion::Lepton;
//use Pion::photon::Photon;

use crate::nstruct::Nuclide;

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
            Particle::Element(x, z) => z.to_string() + &x.to_string(),
        }
    }
    
    #[allow(dead_code)]
    pub fn energy(&self) -> f64 {
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
