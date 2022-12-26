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

*/

use crate::atom::Atom;
//use crate::nuclidedata::*;

use crate::nuclidedata::index::*;

use crate::particle::*;

use crate::nuclidedata::nuclidestruct::nucleons_nuclide;

use crate::Nuclide;
use Pion::baryon::Baryon;
use Pion::lepton::AntiLepton;
use Pion::lepton::Lepton;

#[allow(non_upper_case_globals)]
const DALTON_MeV: f64 = 931.49410242;
//const NEUTRINO_RATIO: f64 = 0.999999765;

fn max(x: Nuclide, y: Nuclide) -> (Nuclide, Nuclide) {
    if x.am() > y.am() {
        return (x, y);
    }
    (y, x)
}

impl Nuclide {
    #[inline]
    pub fn proton_emission(&mut self, pcount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= pcount;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let p_mass = pcount as f64 * PROTONMASS;
        let scalar = (pcount as f64).recip();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (p_mass + d_mass).recip();
        let d_energia = (p_mass) * optimizer;
        let p_energia = (d_mass) * optimizer;
        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];
        for _ in 0..pcount {
            particle_vector.push(Particle::Baryon(Baryon::Proton(p_energia * scalar)));
        }
        (d_energia, particle_vector)
    }

    #[inline]
    pub fn neutron_emission(&mut self, ncount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.1 -= ncount;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let n_mass = ncount as f64 * NEUTRONMASS;
        let scalar = (ncount as f64).recip();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (n_mass + d_mass).recip();
        let d_energia = (n_mass) * optimizer;
        let p_energia = (d_mass) * optimizer;
        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];
        for _ in 0..ncount {
            particle_vector.push(Particle::Baryon(Baryon::Neutron(p_energia * scalar)));
        }
        (d_energia, particle_vector)
    }

    #[inline]
    pub fn alpha_emission(&mut self, acount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= acount << 1;
        izotop.1 -= acount << 1;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let a_mass = acount as f64 * ALPHAMASS;
        let scalar = (acount as f64).recip();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (a_mass + d_mass).recip();
        let d_energia = (a_mass) * optimizer;
        let p_energia = (d_mass) * optimizer;
        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];
        for _ in 0..acount {
            particle_vector.push(Particle::Alpha(p_energia * scalar));
        }
        (d_energia, particle_vector)
    }

    pub fn alpha_neutron(&mut self, ncount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= 2;
        izotop.1 -= ncount + 2;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let neu_mass = ncount as f64 * NEUTRONMASS;
        let scalar = (ncount as f64).recip();
        let totalenergia = (self.am() - d_mass) * DALTON_MeV;
        let particleenergia = totalenergia
            - ((ALPHAMASS + neu_mass) / (d_mass + neu_mass + ALPHAMASS)) * totalenergia;
        let neutronenergia =
            particleenergia - (ALPHAMASS / (ALPHAMASS + neu_mass)) * particleenergia;
        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];
        let energia = totalenergia - particleenergia;
        particle_vector.push(Particle::Alpha(particleenergia - neutronenergia));
        for _ in 0..ncount {
            particle_vector.push(Particle::Baryon(Baryon::Neutron(neutronenergia * scalar)));
        }
        (energia, particle_vector)
    } // possible reversal

    pub fn neutron_deuteron(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= 1;
        izotop.1 -= 2;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let totalenergia = (self.am() - d_mass) * DALTON_MeV;
        let particleenergia = totalenergia
            - ((NEUTRONMASS + DEUTERONMASS) / (d_mass + NEUTRONMASS + DEUTERONMASS)) * totalenergia;
        let d_energia =
            particleenergia - ((DEUTERONMASS) / (NEUTRONMASS + DEUTERONMASS)) * particleenergia;
        let energia = totalenergia - particleenergia;
        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Deuteron(d_energia),
            Particle::Baryon(Baryon::Neutron(particleenergia - d_energia)),
        ];
        (energia, particle_vector)
    }

    pub fn neutron_triton(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= 1;
        izotop.1 -= 2;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let totalenergia = (self.am() - d_mass) * DALTON_MeV;
        let particleenergia = totalenergia
            - ((NEUTRONMASS + TRITONMASS) / (d_mass + NEUTRONMASS + TRITONMASS)) * totalenergia;
        let t_energia =
            particleenergia - ((TRITONMASS) / (NEUTRONMASS + TRITONMASS)) * particleenergia;
        let energia = totalenergia - particleenergia;
        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Deuteron(t_energia),
            Particle::Baryon(Baryon::Neutron(particleenergia - t_energia)),
        ];
        (energia, particle_vector)
    }
    // fix possible
    #[inline]
    pub fn electron_capture(&mut self, ccount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= ccount;
        izotop.1 += ccount;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let neu_mass = NEUTRINOMASS * ccount as f64;
        let scalar = (ccount as f64).recip();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (neu_mass + d_mass).recip();
        let d_energia = (neu_mass) * optimizer;
        let p_energia = (d_mass) * optimizer;

        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];
        for _ in 0..ccount {
            particle_vector.push(Particle::Lepton(Lepton::ElectronNeutrino(
                p_energia * scalar,
            )));
        }
        (d_energia, particle_vector)
    }

    #[inline]
    pub fn electron_emission(&mut self, bcount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 += bcount;
        izotop.1 -= bcount;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let e_mass = ELECTRONMASS * bcount as f64;
        let neu_mass = NEUTRINOMASS * bcount as f64;
        let d_mass = daughter.am();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (d_mass + e_mass + neu_mass).recip();
        let d_energia = (e_mass + neu_mass) * optimizer;
        let e_energia = (e_mass + d_mass) * optimizer;
        let neu_energia = (neu_mass) * optimizer;
        //let delta_energy = (self.am() - (d_mass)) * DALTON_MeV;

        let scalar = (bcount as f64).recip();

        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];

        for _ in 0..bcount {
            particle_vector.push(Particle::Lepton(Lepton::Electron(e_energia * scalar)));
        }
        for _ in 0..bcount {
            particle_vector.push(Particle::AntiLepton(AntiLepton::ElectronNeutrino(
                neu_energia * scalar,
            )));
        }
        (d_energia, particle_vector)
    }

    pub fn electron_neutron(&mut self, ncount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 += 1;
        izotop.1 -= ncount + 1;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let n_mass = NEUTRONMASS * ncount as f64;
        let t_mass = d_mass + n_mass + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + n_mass) / t_mass)) * q;
        let n_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((n_mass + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + n_mass + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let scalar = (ncount as f64).recip();
        let mut particle_vector = vec![];

        particle_vector.push(Particle::Lepton(Lepton::Electron(e_energia)));
        particle_vector.push(Particle::AntiLepton(AntiLepton::ElectronNeutrino(
            neu_energia,
        )));
        for _ in 0..ncount {
            particle_vector.push(Particle::Baryon(Baryon::Neutron(n_energia * scalar)));
        }
        (d_energia, particle_vector)
    }

    pub fn electron_proton(&mut self, pcount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= pcount - 1;
        izotop.1 -= 1;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let p_mass = PROTONMASS * pcount as f64;
        let t_mass = d_mass + p_mass + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + p_mass) / t_mass)) * q;
        let p_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((p_mass + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + p_mass + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let scalar = (pcount as f64).recip();
        let mut particle_vector = vec![];

        particle_vector.push(Particle::AntiLepton(AntiLepton::Electron(e_energia)));
        particle_vector.push(Particle::Lepton(Lepton::ElectronNeutrino(neu_energia)));
        for _ in 0..pcount {
            particle_vector.push(Particle::Baryon(Baryon::Proton(p_energia * scalar)));
        }
        (d_energia, particle_vector)
    }

    pub fn electron_alpha(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= 1;
        izotop.1 -= 3;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let t_mass = d_mass + ALPHAMASS + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + ALPHAMASS) / t_mass)) * q;
        let a_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((ALPHAMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + ALPHAMASS + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Lepton(Lepton::Electron(e_energia)),
            Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energia)),
            Particle::Alpha(a_energia),
        ];
        (d_energia, particle_vector)
    }

    pub fn electron_deuteron(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.1 -= 2;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let t_mass = d_mass + ALPHAMASS + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + DEUTERONMASS) / t_mass)) * q;
        let deu_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((DEUTERONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + DEUTERONMASS + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Lepton(Lepton::Electron(e_energia)),
            Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energia)),
            Particle::Deuteron(deu_energia),
        ];

        (d_energia, particle_vector)
    }

    pub fn electron_triton(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.1 -= 3;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let t_mass = d_mass + ALPHAMASS + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + TRITONMASS) / t_mass)) * q;
        let tri_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((TRITONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + TRITONMASS + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Lepton(Lepton::Electron(e_energia)),
            Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energia)),
            Particle::Triton(tri_energia),
        ];
        (d_energia, particle_vector)
    }

    pub fn electron_fission(&mut self) -> (f64, Vec<Particle>) {
        let (energia, mut particle_vector) = self.electron_emission(1);
        let (s_energia, s_vector) = self.spontaneous_fission();
        particle_vector.push(s_vector[0].clone());
        (energia + s_energia, particle_vector)
    }

    #[inline]
    pub fn positron_emission(&mut self, bcount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= bcount;
        izotop.1 += bcount;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let e_mass = ELECTRONMASS * bcount as f64;
        let neu_mass = NEUTRINOMASS * bcount as f64;
        let d_mass = daughter.am();
        let optimizer = (self.am() - d_mass) * DALTON_MeV * (d_mass + e_mass + neu_mass).recip();
        let d_energia = (e_mass + neu_mass) * optimizer;
        let e_energia = (e_mass + d_mass) * optimizer;
        let neu_energia = (neu_mass) * optimizer;

        let scalar = (bcount as f64).recip();

        self.change(daughter.nuclide_index());
        let mut particle_vector = vec![];

        for _ in 0..bcount {
            particle_vector.push(Particle::AntiLepton(AntiLepton::Electron(
                e_energia * scalar,
            )));
        }
        for _ in 0..bcount {
            particle_vector.push(Particle::Lepton(Lepton::ElectronNeutrino(
                neu_energia * scalar,
            )));
        }
        (d_energia, particle_vector)
    }

    pub fn positron_proton(&mut self, pcount: usize) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= pcount + 1;
        izotop.1 += 1;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let p_mass = PROTONMASS * pcount as f64;
        let t_mass = d_mass + p_mass + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + p_mass) / t_mass)) * q;
        let p_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((p_mass + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + p_mass + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let scalar = (pcount as f64).recip();
        let mut particle_vector = vec![
            Particle::AntiLepton(AntiLepton::Electron(e_energia)),
            Particle::Lepton(Lepton::ElectronNeutrino(neu_energia)),
        ];

        for _ in 0..pcount {
            particle_vector.push(Particle::Baryon(Baryon::Proton(p_energia * scalar)));
        }
        (d_energia, particle_vector)
    }

    pub fn positron_alpha(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        izotop.0 -= 3;
        izotop.1 -= 1;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let d_mass = daughter.am();
        let t_mass = d_mass + ALPHAMASS + ELECTRONMASS + NEUTRINOMASS;

        let q = (self.am() - (d_mass)) * DALTON_MeV;
        let e_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + ALPHAMASS) / t_mass)) * q;
        let a_energia = (1.0 - ((ELECTRONMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let d_energia = (1.0 - ((ALPHAMASS + NEUTRINOMASS + d_mass) / t_mass)) * q;
        let neu_energia = (1.0 - ((ELECTRONMASS + ALPHAMASS + d_mass) / t_mass)) * q;

        self.change(daughter.nuclide_index());

        let particle_vector = vec![
            Particle::Lepton(Lepton::Electron(e_energia)),
            Particle::AntiLepton(AntiLepton::ElectronNeutrino(neu_energia)),
            Particle::Alpha(a_energia),
        ];

        (d_energia, particle_vector)
    }

    pub fn positron_fission(&mut self) -> (f64, Vec<Particle>) {
        let (energia, mut particle_vector) = self.positron_emission(1);
        let (s_energia, s_vector) = self.spontaneous_fission();
        particle_vector.push(s_vector[0].clone());
        (energia + s_energia, particle_vector)
    }

    #[inline]
    pub fn spontaneous_fission(&mut self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        let rn = fastrand::usize(..);

        let mut z = rn % izotop.0;

        if z == 0 {
            z = 1;
        }
        let start =SYMBOL_INDEX[z as usize - 1].1 -z;
        let stop = SYMBOL_INDEX[z as usize - 1].2 -z;
      
        let n = (rn%(stop-start-1) + start)%izotop.1;

        //let n = (SYMBOL_INDEX[z as usize - 1].1
         //   + rn % (SYMBOL_INDEX[z as usize - 1].2 - SYMBOL_INDEX[z as usize - 1].1-1))
          //  - z;

        let decay = Nuclide::assign(nucleons_nuclide(&(z, n)));

        let de_mass = decay.am();
        izotop.0 -= z;
        izotop.1 -= n;
        let daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let q = (self.am() - (daughter.am() + de_mass)) * DALTON_MeV;
        let (big, small) = max(daughter, decay);
        let mut particle_vector = vec![];
        let energia = q - (small.am() / (big.am() + small.am())) * q;
        particle_vector.push(Particle::Element(small, energia));
        self.change(big.nuclide_index());
        (q - energia, particle_vector)
    }

    #[inline]
    pub fn cluster_decay(&mut self, daughter: &Self) -> (f64, Vec<Particle>) {
        let mut izotop = self.proton_neutron();
        let dizotop = daughter.proton_neutron();
        izotop.0 -= dizotop.0;
        izotop.1 -= dizotop.1;
        let new_daughter = Nuclide::assign(nucleons_nuclide(&izotop));
        let totalenergia = (self.am() - new_daughter.am()) * DALTON_MeV;
        let particleenergia = totalenergia - (daughter.am() / self.am()) * totalenergia;
        let energia = totalenergia - particleenergia;
        self.change(new_daughter.nuclide_index());
        let particle_vector = vec![Particle::Element(*daughter, particleenergia)];

        (energia, particle_vector)
    }

    pub fn double_cluster(&mut self, daughter: &Self, son: &Self) -> (f64, Vec<Particle>) {
        let (energia, mut particle_vector) = self.cluster_decay(daughter);
        let (s_energia, s_vector) = self.cluster_decay(son);
        particle_vector.push(s_vector[0].clone());
        (energia + s_energia, particle_vector)
    }
}
