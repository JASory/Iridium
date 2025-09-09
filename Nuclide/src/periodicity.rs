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
const SUBSHELLS : [(&str, u8);19] = [
    ("1s", 2),
    ("2s", 2), ("2p", 6),
    ("3s", 2), ("3p", 6),
    ("4s", 2), ("3d", 10), ("4p", 6),
    ("5s", 2), ("4d", 10), ("5p", 6),
    ("6s", 2), ("4f", 14), ("5d", 10), ("6p", 6),
    ("7s", 2), ("5f", 14), ("6d", 10), ("7p", 6),
];

//TODO: maybe just some kind of "diff" instead of the whole (mostly redundant) configuration, e.g.:
//(24, &[("*4s",1), ("*3d",5)]), //Cr
//...
//(57, &[("*4f",0), ("+5d",1)]), //La
//where '*' means "modified subshell", and '+' means "added subshell"?
#[rustfmt::skip]
const NON_AUFBAU_CONFIGS : [(u8, &[(&str, u8)]);19] = [ // (an, [(subshell, no_electrons)])
    (24, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",1), ("3d",5)]), //Cr
    (29, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",1), ("3d",10)]), //Cu
    (41, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",1), ("4d",4)]), //Nb
    (42, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",1), ("4d",5)]), //Mo
    (44, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",1), ("4d",7)]), //Ru
    (45, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",1), ("4d",8)]), //Rh
    (46, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",0), ("4d",10)]), //Pd
    (47, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",1), ("4d",10)]), //Ag
    (57, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",0),  ("5d",1)]), //La
    (58, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",1),  ("5d",1)]), //Ce
    (64, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",7),  ("5d",1)]), //Gd
    (78, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",1), ("4f",14), ("5d",9)]), //Pt
    (79, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",1), ("4f",14), ("5d",10)]), //Au
    (89, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 0), ("6d",1)]), //Ac
    (90, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 0), ("6d",2)]), //Th
    (91, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 2), ("6d",1)]), //Pa
    (92, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 3), ("6d",1)]), //U
    (93, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 4), ("6d",1)]), //Np
    (96, &[("1s",2), ("2s",2), ("2p",6), ("3s",2), ("3p",6), ("4s",2), ("3d",10), ("4p",6), ("5s",2), ("4d",10), ("5p",6), ("6s",2), ("4f",14), ("5d",10), ("6p",6), ("7s",2), ("5f", 7), ("6d",1)]), //Cm
];

//TODO: noble gas notation
#[allow(dead_code)]
const NOBLE_GASES : [u8; 7] = [2, 10, 18, 36, 54, 86, 118];

/// Struct representing electron configuration of a nuclide.
/// Currently, it can be constructed only for Element objects.
/// Examples:
/// ```
/// let elem = ElectronConfig::new(Element::He);
/// println!("Helium electron configuration is: {}", elem.as_el_conf_spdf_notation())
/// ```
///
pub struct ElectronConfig<'a> {
    subshells: Vec<(&'a str, u8)>,
}

impl ElectronConfig<'_> {
    pub fn new<'a>(element: Element) -> ElectronConfig<'a> {
        // REMARK: for elements above 103 there's little or no experimental verification of the electron configurations
        let mut subshells: Vec<(&str, u8)> = Vec::new();

        // Check whether we've got "non-standard" (aufbau principle-wise) electron configuration
        for exception in NON_AUFBAU_CONFIGS {
            if element.protons() == exception.0 {
                return ElectronConfig { subshells: exception.1.iter().map(|ss| *ss).collect() }
            }
        }
        // no, so it's aufbau principle-conformant electron configuration, let's "calculate"
        let mut no_el: i8 = element.protons() as i8;
        for subshell in SUBSHELLS {
            no_el = no_el - subshell.1 as i8;
            if no_el <= 0 {
                subshells.push((subshell.0, (subshell.1 as i8 + no_el) as u8));
                break;
            } else {
                subshells.push(subshell);
            }
        }
        ElectronConfig { subshells }
    }

    /// Returns electron configuration as a simple text, e.g. "1s2 2s2 2p2 " for Carbon
    pub fn as_el_conf_simple_text(&self) -> String {
        self.subshells.iter().map(|x| format!("{}{} ", x.0, x.1)).collect()
    }

    /// Returns electron configuration in spdf notation, e.g. "1s²2s²2p²" for Carbon
    pub fn as_el_conf_spdf_notation(&self) -> String {
        self.subshells.iter()
            .map(|x| format!("{}{}", x.0, Self::num_to_unicode_superscripts(x.1)))
            .collect()
    }

    /// Returns electron configuration in noble gas notation, e.g. "[He]2s²2p²" for Carbon
    pub fn as_el_conf_noble_notation(&self) -> String {
        todo!()
    }

    fn num_to_unicode_superscripts(number: u8) -> String {
        #[rustfmt::skip]
        const UNICODE_SUPERSCRIPT_CHARS : [&[u8]; 10] = [
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
            .map(|c| str::from_utf8(UNICODE_SUPERSCRIPT_CHARS[c]).unwrap())
            .collect()
    }
}

/// Periodic table related properties and qualities
pub trait Periodicity: Clone {
    /// Group (family) in the periodic table (in accordance with IUPAC numbering)
    fn group_iupac(&self) -> Group;

    /// Period in the periodic table
    fn period(&self) -> Period;

    /// Electron configuration in ground state
    fn orbitals_gs<'a>(&self) -> ElectronConfig;
}

impl Periodicity for Element {
    fn group_iupac(&self) -> Group {
        GROUP[self.atomic_num() as usize - 1]
    }

    fn period(&self) -> Period {
        PERIOD[self.atomic_num() as usize - 1]
    }

    fn orbitals_gs<'a>(&self) -> ElectronConfig {
        ElectronConfig::new(*self)
    }
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
        let el_conf = ElectronConfig::new(elem);
        assert_eq!(el_conf.subshells, vec![("1s", 2)]);
        assert_eq!(el_conf.as_el_conf_simple_text(), "1s2 ");
        assert_eq!(el_conf.as_el_conf_spdf_notation(), "1s²");
    }

    #[test]
    fn argon_electron_config() {
        let elem = Element::Ar;
        assert_eq!(GNo(18), elem.group_iupac());
        assert_eq!(PNo(3), elem.period());
        let el_conf = ElectronConfig::new(elem);
        assert_eq!(el_conf.subshells, vec![("1s", 2), ("2s", 2), ("2p", 6), ("3s", 2), ("3p", 6)]);
        assert_eq!(el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 ");
        assert_eq!(el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶");
    }

    #[test]
    fn copper_electron_config() {
        let elem = Element::Cu;
        assert_eq!(GNo(11), elem.group_iupac());
        assert_eq!(PNo(4), elem.period());
        let el_conf = ElectronConfig::new(elem);
        assert_eq!(el_conf.subshells, vec![("1s", 2), ("2s", 2), ("2p", 6), ("3s", 2), ("3p", 6), ("4s", 1), ("3d", 10)]);
        assert_eq!(el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 4s1 3d10 ");
        assert_eq!(el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶4s¹3d¹⁰");
    }

    #[test]
    fn gadolinium_electron_config() {
        let elem = Element::Gd;
        assert_eq!(FB(8), elem.group_iupac());
        assert_eq!(Lanth, elem.period());
        let el_conf = ElectronConfig::new(elem);
        assert_eq!(el_conf.subshells, vec![("1s", 2), ("2s", 2), ("2p", 6), ("3s", 2), ("3p", 6), ("4s", 2), ("3d", 10),
                                           ("4p", 6), ("5s", 2), ("4d",10), ("5p", 6), ("6s", 2), ("4f", 7), ("5d", 1)]);
        assert_eq!(el_conf.as_el_conf_simple_text(), "1s2 2s2 2p6 3s2 3p6 4s2 3d10 4p6 5s2 4d10 5p6 6s2 4f7 5d1 ");
        assert_eq!(el_conf.as_el_conf_spdf_notation(), "1s²2s²2p⁶3s²3p⁶4s²3d¹⁰4p⁶5s²4d¹⁰5p⁶6s²4f⁷5d¹");
    }

}
