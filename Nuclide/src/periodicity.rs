use crate::element::Element;
use crate::traits::ChemElement;

/// Enum for periodic table groups. Naming (numbering) in accordance with IUPAC convention,
/// plus assignment of f-block (with "domestic" auxiliary ordinal) for lanthanides and actinides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Group {
    GNo(usize), // group number
    FB(usize),  // f-block with an auxiliary ordinal (for lanthanides and actinides)
}

use Group::*;

#[rustfmt::skip]
const GROUP : [Group;118] = [

    GNo(1),                                                                                                                                         GNo(18),
    GNo(1), GNo(2),                                                                                    GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
    GNo(1), GNo(2),                                                                                    GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
    GNo(1), GNo(2), GNo(3), GNo(4), GNo(5), GNo(6), GNo(7), GNo(8), GNo(9), GNo(10), GNo(11), GNo(12), GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
    GNo(1), GNo(2), GNo(3), GNo(4), GNo(5), GNo(6), GNo(7), GNo(8), GNo(9), GNo(10), GNo(11), GNo(12), GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
    GNo(1), GNo(2),
                    FB(1),  FB(2),  FB(3),  FB(4),  FB(5),  FB(6),  FB(7),  FB(8),   FB(9),   FB(10),  FB(11),  FB(12),  FB(13),  FB(14),  FB(15),
                            GNo(4), GNo(5), GNo(6), GNo(7), GNo(8), GNo(9), GNo(10), GNo(11), GNo(12), GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
    GNo(1), GNo(2),
                    FB(1),  FB(2),  FB(3),  FB(4),  FB(5),  FB(6),  FB(7),  FB(8),   FB(9),   FB(10),  FB(11),  FB(12),  FB(13),  FB(14),  FB(15),
                            GNo(4), GNo(5), GNo(6), GNo(7), GNo(8), GNo(9), GNo(10), GNo(11), GNo(12), GNo(13), GNo(14), GNo(15), GNo(16), GNo(17), GNo(18),
];

/// Enum for periodic table periods with separate value for lanthanides and actinides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Period {
    PNo(usize), // period number
    Lanth,      // lanthanides
    Actin,      // actinides
}

use Period::*;
use crate::quantum_electron::{ElectronSpin, EnergyState, Location, QuantumElectron};

#[rustfmt::skip]
const PERIOD : [Period;118] = [

    PNo(1),                                                                                                                                 PNo(1),
    PNo(2), PNo(2),                                                                                 PNo(2), PNo(2), PNo(2), PNo(2), PNo(2), PNo(2),
    PNo(3), PNo(3),                                                                                 PNo(3), PNo(3), PNo(3), PNo(3), PNo(3), PNo(3),
    PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4), PNo(4),
    PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5), PNo(5),
    PNo(6), PNo(6),
                    Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,  Lanth,
                            PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6), PNo(6),
    PNo(7), PNo(7),
                    Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,  Actin,
                            PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7), PNo(7),
];

#[rustfmt::skip]
  const NON_AUFBAU_CONFIGS : [(u8, u8, &[(u8, u8, i8, i8)]);19] = [ // (element Z, noble-gas Z, [(n, l, mu, me)])
    (24, 18, &[(4,0,0,-1), (3,2,-2,-1), (3,2,-1,-1), (3,2,0,-1),  (3,2,1,-1), (3,2,2,-1)]), //Cr -> [Ar] 4s1 3d5
    (29, 18, &[(4,0,0,-1), (3,2,-2,-1), (3,2,-1,-1), (3,2,0,-1),  (3,2,1,-1), (3,2,2,-1),  (3,2,-2,1), (3,2,-1,1), (3,2,0,1),  (3,2,1,1),  (3,2,2,1)]), //Cu -> [Ar] 4s1 3d10
    (41, 36, &[(5,0,0,-1), (4,2,-2,-1), (4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1)]), //Nb -> [Kr] 5s1 4d4
    (42, 36, &[(5,0,0,-1), (4,2,-2,-1), (4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1), (4,2,2,-1)]), //Mo -> [Kr] 5s1 4d5
    (44, 36, &[(5,0,0,-1), (4,2,-2,-1), (4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1), (4,2,2,-1),  (4,2,-2,1), (4,2,-1,1)]), //Ru -> [Kr] 5s1 4d7
    (45, 36, &[(5,0,0,-1), (4,2,-2,-1), (4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1), (4,2,2,-1),  (4,2,-2,1), (4,2,-1,1), (4,2,0,1)]), //Rh -> [Kr] 5s1 4d8
    (46, 36, &[(4,2,-2,-1),(4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1),  (4,2,2,-1), (4,2,-2,1),  (4,2,-1,1), (4,2,0,1),  (4,2,1,1),  (4,2,2,1)]), //Pd -> [Kr] 4d10
    (47, 36, &[(5,0,0,-1), (4,2,-2,-1), (4,2,-1,-1), (4,2,0,-1),  (4,2,1,-1), (4,2,2,-1),  (4,2,-2,1), (4,2,-1,1), (4,2,0,1),  (4,2,1,1),  (4,2,2,1)]), //Ag -> [Kr] 5s1 4d10
    (57, 54, &[(6,0,0,-1), (6,0,0,1),   (5,2,-2,-1)]), //La -> [Xe] 6s2 5d1
    (58, 54, &[(6,0,0,-1), (6,0,0,1),   (4,3,-3,-1), (5,2,-2,-1)]), //Ce -> [Xe] 6s2 4f1 5d1
    (64, 54, &[(6,0,0,-1), (6,0,0,1),   (4,3,-3,-1), (4,3,-2,-1), (4,3,-1,-1), (4,3,0,-1), (4,3,1,-1), (4,3,2,-1), (4,3,3,-1), (5,2,-2,-1)]), //Gd -> [Xe] 6s2 4f7 5d1
    (78, 54, &[(6,0,0,-1), (4,3,-3,-1), (4,3,-2,-1), (4,3,-1,-1), (4,3,0,-1),  (4,3,1,-1), (4,3,2,-1), (4,3,3,-1), (4,3,-3,1), (4,3,-2,1), (4,3,-1,1), (4,3,0,1), (4,3,1,1), (4,3,2,1), (4,3,3,1), (5,2,-2,-1), (5,2,-1,-1), (5,2,0,-1), (5,2,1,-1), (5,2,2,-1), (5,2,-2,1), (5,2,-1,1), (5,2,0,1), (5,2,1,1)]), //Pt -> [Xe] 6s1 4f14 5d9
    (79, 54, &[(6,0,0,-1), (4,3,-3,-1), (4,3,-2,-1), (4,3,-1,-1), (4,3,0,-1),  (4,3,1,-1), (4,3,2,-1), (4,3,3,-1), (4,3,-3,1), (4,3,-2,1), (4,3,-1,1), (4,3,0,1), (4,3,1,1), (4,3,2,1), (4,3,3,1), (5,2,-2,-1), (5,2,-1,-1), (5,2,0,-1), (5,2,1,-1), (5,2,2,-1), (5,2,-2,1), (5,2,-1,1), (5,2,0,1), (5,2,1,1), (5,2,2,1)]), //Au -> [Xe] 6s1 4f14 5d10
    (89, 86, &[(7,0,0,-1), (7,0,0,1),   (6,2,-2,-1)]), //Ac -> [Rn] 7s2 6d1
    (90, 86, &[(7,0,0,-1), (7,0,0,1),   (6,2,-2,-1), (6,2,-1,-1)]), //Th -> [Rn] 7s2 6d2
    (91, 86, &[(7,0,0,-1), (7,0,0,1),   (5,3,-3,-1), (5,3,-2,-1), (6,2,-2,-1)]), //Pa -> [Rn] 7s2 5f2 6d1
    (92, 86, &[(7,0,0,-1), (7,0,0,1),   (5,3,-3,-1), (5,3,-2,-1), (5,3,-1,-1), (6,2,-2,-1)]), //U -> [Rn] 7s2 5f3 6d1
    (93, 86, &[(7,0,0,-1), (7,0,0,1),   (5,3,-3,-1), (5,3,-2,-1), (5,3,-1,-1), (5,3,0,-1), (6,2,-2,-1)]), //Np -> [Rn] 7s2 5f4 6d1
    (96, 86, &[(7,0,0,-1), (7,0,0,1),   (5,3,-3,-1), (5,3,-2,-1), (5,3,-1,-1), (5,3,0,-1), (5,3,1,-1), (5,3,2,-1), (5,3,3,-1), (6,2,-2,-1)]), //Cm -> [Rn] 7s2 5f7 6d1
  ];

const REQUIRED_SHELLS_FOR_TOTAL_ELECTRONS: [(u8, u8);7] = [(1, 2), (2, 10), (3, 18), (4, 36), (5, 54), (6, 86), (7, 118)]; //partitioned by noble gases

/// Struct representing electron configuration of a nuclide.
/// 
/// Currently, it can be constructed only for Element objects.
/// In case of methods generating textual representation of the configuration, by following
/// the general convention the orbitals are not written in accordance with Madelung rule, but rather
/// in order of increasing *n* (Principal Quantum Number) and then in order of increasing *l*
/// (Orbital Angular Momentum Quantum Number, a.k.a. Azimuthal Quantum Number).
///
/// Examples:
/// ```
/// let elem = QuantumElectronConfig::from_element(Element::He);
/// println!("Helium electron configuration is: {}", elem.as_el_conf_spdf_notation())
/// ```
///
pub struct ElectronConfig {
    electrons: Vec<QuantumElectron>,
}

impl ElectronConfig {
    //TODO: for isotope, ion, etc., not only for elements
    /// Creates electron configuration for the given element.
    ///
    /// REMARK: for elements above 103 there's little or no experimental verification of the electron configurations
    pub fn from_element(element: Element) -> ElectronConfig {
        let mut electrons: Vec<QuantumElectron> = Vec::new();

        // Check whether we've got "non-standard" (aufbau principle-wise) electron configuration
        // If so, re-generate configuration based on lookup table (construct for noble gas, and add outer electrons)
        for exception in NON_AUFBAU_CONFIGS {
            if element.protons() == exception.0 {
                let mut electrons = ElectronConfig::from_element(Element::from_protons(exception.1)).electrons;
                let mut outer_electrons: Vec<QuantumElectron> = exception.2
                    .iter()
                    .map(|x| QuantumElectron::new(x.0, x.1, x.2,
                                                  if x.3 == -1 {ElectronSpin::MinusOneHalf} else {ElectronSpin::PlusOneHalf},
                                                  Location::Outer, EnergyState::GroundState))
                    .collect();
                electrons.append(&mut outer_electrons);

                return ElectronConfig { electrons }
            }
        }
        // no, so it's aufbau principle-conformant electron configuration, let's "calculate"
        let mut no_el: i8 = element.protons() as i8;
        let shells = Self::no_of_shells(no_el as u8);
        let mut subshells: Vec<(u8, u8)> = Vec::new();
        // Generate all "empty" subshells
        for n in 1..=shells {
            for l in 0..n {
                subshells.push((n,l));
            }
        }
        // Sort subshells according to Madelung rule
        subshells.sort_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)).then(a.0.cmp(&b.0)));

        'main_loop: for ss in subshells {
            let mut me = ElectronSpin::PlusOneHalf;
            for _ in 0..=1 {
                me.flip_flop();
                for ml in -(ss.1 as i8)..=(ss.1 as i8) {
                    //TODO: Location
                    electrons.push(QuantumElectron::new(ss.0, ss.1, ml, me.clone(), Location::Core, EnergyState::GroundState));
                    no_el -= 1;
                    if no_el == 0 {
                        break 'main_loop;
                    }
                }
            }
        }

        ElectronConfig { electrons }
    }

    /// Electron configuration as a list of consecutive electrons
    pub fn as_electrons(&self) -> Vec<QuantumElectron> {
        self.electrons.clone()
    }

    /// Electron configuration grouped into consecutive subshells (ordering with Madelung rule)
    pub fn as_subshells(&self) -> Vec<Vec<QuantumElectron>> {
        let mut electrons = self.electrons.clone();
        electrons.sort();
        Self::get_subshells(electrons)
    }

    /// Electron configuration grouped into consecutive shells
    pub fn as_shells(&self) -> Vec<Vec<QuantumElectron>> {
        let mut electrons = self.electrons.clone();
        electrons.sort_by_key(|e| e.p_qn);
        Self::get_shells(electrons)
    }

    /// Returns electron configuration as a simple text, e.g. "1s2 2s2 2p2 " for Carbon
    pub fn as_el_conf_simple_text(&self) -> String {
        Self::grouped_electrons_to_simplified_subshells_sorted_for_printing(self.as_subshells()).iter()
            .map(|x| format!("{}{} ", x.0.to_string() + x.2, x.3))
            .collect()
    }

    /// Returns electron configuration in spdf notation, e.g. "1s²2s²2p²" for Carbon
    pub fn as_el_conf_spdf_notation(&self) -> String {
        Self::grouped_electrons_to_simplified_subshells_sorted_for_printing(self.as_subshells()).iter()
            .map(|x| format!("{}{}", x.0.to_string() + x.2, num_to_unicode_superscripts(x.3 as u8)))
            .collect()
    }

    /// Returns electron configuration in noble gas notation, e.g. "\[He\]2s²2p²" for Carbon
    pub fn as_el_conf_noble_gas_notation(&self) -> String {
        let mut electrons = self.electrons.clone();
        electrons.sort();
        let partition = Self::get_config_partitioned_at_noble_gas(electrons);
        let outer_spdf: String = Self::grouped_electrons_to_simplified_subshells_sorted_for_printing(Self::get_subshells(partition.1)).iter()
            .map(|x| format!("{}{}", x.0.to_string() + x.2, num_to_unicode_superscripts(x.3 as u8)))
            .collect();
        match partition.0 {
            None => outer_spdf,
            Some(x) => format!("[{}]{}", x.symbol(), outer_spdf)
        }
    }

    fn no_of_shells(no_el: u8) -> u8 {
        // Computing analytically from the number of electrons is harder than just using simple lookup table
        REQUIRED_SHELLS_FOR_TOTAL_ELECTRONS.iter().filter(|n| n.1 >= no_el).min_by_key(|x| x.1).map(|n| n.0).unwrap()
    }

    fn get_subshells(electrons: Vec<QuantumElectron>) -> Vec<Vec<QuantumElectron>> {
        Self::get_partitioned_configuration(
            electrons,
            |e1, e2| e1.p_qn != e2.p_qn || e1.oam_qn != e2.oam_qn)
    }

    fn get_shells(electrons: Vec<QuantumElectron>) -> Vec<Vec<QuantumElectron>> {
        Self::get_partitioned_configuration(
            electrons,
            |e1, e2| e1.p_qn != e2.p_qn)
    }

    fn get_partitioned_configuration<F: Fn(QuantumElectron, QuantumElectron) -> bool>(
        electrons: Vec<QuantumElectron>, partition_condition: F) -> Vec<Vec<QuantumElectron>> {
        let mut result: Vec<Vec<QuantumElectron>> = Vec::new();
        let mut iter = electrons.iter().peekable();
        let mut subshell: Vec<QuantumElectron> = Vec::new();
        while let Some(&el) = iter.next() {
            subshell.push(el.clone());
            if let Some(&next_el) = iter.peek() {
                if partition_condition(el, *next_el) {
                    result.push(subshell);
                    subshell = Vec::new();
                }
            }
        }
        result.push(subshell);
        result
    }

    fn get_config_partitioned_at_noble_gas(mut electrons: Vec<QuantumElectron>) -> (Option<Element>, Vec<QuantumElectron>) {
        match electrons.len() {
            1|2 => (None, electrons),
            _ => {
                let noble_gas_no_el = *REQUIRED_SHELLS_FOR_TOTAL_ELECTRONS.iter()
                    .filter(|n| n.1 < electrons.len() as u8)
                    .max()
                    .unwrap();
                electrons.sort();
                let outer_electrons = electrons.drain(noble_gas_no_el.1 as usize..).collect();
                (Some(Element::from_protons(noble_gas_no_el.1)), outer_electrons)
            }
        }
    }

    fn grouped_electrons_to_simplified_subshells_sorted_for_printing<'a>(grouped_electrons: Vec<Vec<QuantumElectron>>) -> Vec<(u8, u8, &'a str, usize)> {
        let mut subshells: Vec<(u8, u8, &str, usize)> = grouped_electrons.iter()
            .map(|x| (x[0].p_qn, x[0].oam_qn, x[0].subshell_symbol(), x.len()))
            .collect();
        subshells.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        subshells
    }

}

/// Periodic table related properties and qualities
pub trait Periodicity: Clone {
    /// Electron configuration
    fn electron_config<'a>(&self) -> ElectronConfig;

    /// Group (family) in the periodic table (in accordance with IUPAC numbering)
    fn group_iupac(&self) -> Group;

    /// Period in the periodic table
    fn period(&self) -> Period;
}

impl Periodicity for Element {
    /// Electron configuration of the given element
    fn electron_config<'a>(&self) -> ElectronConfig {
        ElectronConfig::from_element(*self)
    }

    /// Group can be easily established based on electron configuration. Still, e.g. for performance
    /// reasons, it is much simpler and faster to use lookup table instead of first generating
    /// the complete configuration.
    fn group_iupac(&self) -> Group {
        GROUP[self.atomic_num() as usize - 1]
    }

    /// Period can be easily established based on electron configuration. Still, e.g. for performance
    /// reasons, it is much simpler and faster to use lookup table instead of first generating
    /// the complete configuration.
    fn period(&self) -> Period {
        PERIOD[self.atomic_num() as usize - 1]
    }
}

fn num_to_unicode_superscripts(number: u8) -> String {
    #[rustfmt::skip]
        const UNICODE_SUPERSCRIPT_CHARS_AS_UTF8 : [&[u8]; 10] = [
            &[0xe2, 0x81, 0xb0], //U+2070 superscript 0
            &[0xc2, 0xb9],       //U+00b9 superscript 1
            &[0xc2, 0xb2],       //U+00b2 superscript 2
            &[0xc2, 0xb3],       //U+00b3 superscript 3
            &[0xe2, 0x81, 0xb4], //U+2074 superscript 4
            &[0xe2, 0x81, 0xb5], //U+2075 ...
            &[0xe2, 0x81, 0xb6], //U+2076 ...
            &[0xe2, 0x81, 0xb7], //U+2077 ...
            &[0xe2, 0x81, 0xb8], //U+2078 ...
            &[0xe2, 0x81, 0xb9], //U+2079 superscript 9
        ];
    number.to_string().chars().into_iter()
        .map(|c| c as usize - 48)
        .map(|c| str::from_utf8(UNICODE_SUPERSCRIPT_CHARS_AS_UTF8[c]).unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use Element;
    use super::*;

    #[test]
    fn helium_periodicity() {
        let elem = Element::He;
        assert_eq!(GNo(18), elem.group_iupac());
        assert_eq!(PNo(1), elem.period());
        let quant_el_conf = ElectronConfig::from_element(elem);
        assert_eq!(quant_el_conf.as_el_conf_simple_text(), "1s2 ");
        assert_eq!(quant_el_conf.as_el_conf_spdf_notation(), "1s²");
        assert_eq!(quant_el_conf.as_el_conf_noble_gas_notation(), "1s²");
    }

    #[test]
    fn lithium_periodicity() {
        let elem = Element::Li;
        assert_eq!(GNo(1), elem.group_iupac());
        assert_eq!(PNo(2), elem.period());
        let quant_el_conf = ElectronConfig::from_element(elem);
        assert_eq!(quant_el_conf.as_el_conf_simple_text(), "1s2 2s1 ");
        assert_eq!(quant_el_conf.as_el_conf_spdf_notation(), "1s²2s¹");
        assert_eq!(quant_el_conf.as_el_conf_noble_gas_notation(), "[He]2s¹");
    }

    #[test]
    fn argon_electron_config() {
        let elem = Element::Ar;
        assert_eq!(GNo(18), elem.group_iupac());
        assert_eq!(PNo(3), elem.period());
        let quant_el_conf = ElectronConfig::from_element(elem);
        assert_eq!(quant_el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 ");
        assert_eq!(quant_el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶");
        assert_eq!(quant_el_conf.as_el_conf_noble_gas_notation(), "[Ne]3s²3p⁶");
    }

    #[test]
    fn copper_electron_config() {
        let elem = Element::Cu;
        assert_eq!(GNo(11), elem.group_iupac());
        assert_eq!(PNo(4), elem.period());
        let quant_el_conf = ElectronConfig::from_element(elem);
        assert_eq!(quant_el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 3d10 4s1 ");
        assert_eq!(quant_el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶3d¹⁰4s¹");
        assert_eq!(quant_el_conf.as_el_conf_noble_gas_notation(), "[Ar]3d¹⁰4s¹");
    }

    #[test]
    fn gadolinium_electron_config() {
        let elem = Element::Gd;
        assert_eq!(FB(8), elem.group_iupac());
        assert_eq!(Lanth, elem.period());
        let quant_el_conf = ElectronConfig::from_element(elem);
        assert_eq!(quant_el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 3d10 4s2 4p6 4d10 4f7 5s2 5p6 5d1 6s2 ");
        assert_eq!(quant_el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶3d¹⁰4s²4p⁶4d¹⁰4f⁷5s²5p⁶5d¹6s²");
        assert_eq!(quant_el_conf.as_el_conf_noble_gas_notation(), "[Xe]4f⁷5d¹6s²");
    }

}
