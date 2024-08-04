
use crate::Nuclide;
use crate::traits::ChemElement;
use crate::nuclidedata::index::SYMBOL;
use crate::nuclidedata::elemental::*;
use crate::nuclidedata::ionization::IONIZATION_ENERGIES;


// TODO
// - calculate element mass const for each element
// - #![warn(missing_docs)]

#[derive(Clone)]
/// Struct representing a fractional composition of multiple isotopes
pub struct NuclideFraction {
    pub fractions: Vec<(Nuclide, f64)>
}

impl std::fmt::Display for NuclideFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = self.first().element_name();
        write!(f, "{}-[{:?}]", symbol, self.fractions)
    }
}

impl NuclideFraction {

    pub fn from_nucleon_fractions(protons: u8, nucleon_fractions: &[(u16, f64)]) -> Option<NuclideFraction> {
        
        // At least one nuclide
        if nucleon_fractions.is_empty() {
            return None;
        }

        // Ensure sum of fractions is very close to 1.0
        //let fraction_sum: f64 = nucleon_fractions.iter().map(|tup| tup.1).sum();
        //if dbg!((fraction_sum - 1.0).abs()) > 1e-2 {
        //    return None;
        //}

        nucleon_fractions.iter()
            .map(|&(nucleons, abundance)| {
                let p = protons.into();
                let n = Into::<usize>::into(nucleons) - p;
                Nuclide::from_nucleons(p, n).map(|nucl| (nucl, abundance))
            })
            .collect::<Option<Vec<_>>>()
            .map(|fractions| NuclideFraction {fractions})
    }

    pub fn from_nucleon_fractions_unchecked(protons: u8, neutron_fractions: &[(u16, f64)]) -> NuclideFraction {
        let fractions: Vec<_> = neutron_fractions.iter()
            .map(|&(nucleons, abundance)| {
                let p = protons.into();
                let n = Into::<usize>::into(nucleons) - p;
                let nuclide = Nuclide::from_nucleons_unchecked(p, n);
                (nuclide, abundance)
            })
            .collect();
        NuclideFraction {fractions}
    }

    pub fn from_natural_abundancies(protons: u8) -> NuclideFraction {
        Element::from_protons(protons).abundant_nuclides()
    }
    
    fn first(&self) -> &Nuclide {
        &self.fractions.first().unwrap().0
    }

    fn weighted_property<F: Fn(&Nuclide) -> f64>(&self, map: F) -> f64 {
        if self.fractions.len() == 1 {
            map(self.first())
        } else {
            self.fractions.iter()
                .map(|(nuclide, abundancy)| abundancy * map(nuclide))
                .sum()
        }
    }
}

// This is a bit silly, only the mass is a weighted property
impl ChemElement for NuclideFraction {
    fn atomic_num(&self) -> u64 {
        self.first().atomic_num()
    }

    fn am(&self) -> f64 {
        self.weighted_property(Nuclide::am)
    }

    fn electron_affinity(&self) -> f64 {
        self.first().electron_affinity()
    }

    fn ionization_energies(&self, level: usize) -> Option<f64> {
        self.first().ionization_energies(level)
    }

    fn electronegativity(&self) -> f64 {
        self.first().electronegativity()
    }

    fn mullikan_en(&self) -> f64 {
        self.first().mullikan_en()
    }

    fn allen_en(&self) -> f64 {
        self.first().allen_en()
    }

    fn pauling_en(&self) -> f64 {
        self.first().pauling_en()
    }

    fn covalent_radii(&self, bond: usize) -> Option<f64> {
        self.first().covalent_radii(bond)
    }

    fn ionic_radii(&self) -> f64 {
        self.first().ionic_radii()
    }

    fn vdr_crystal(&self) -> f64 {
        self.first().vdr_crystal()
    }

    fn vdr_isolated(&self) -> f64 {
        self.first().vdr_isolated()
    }
}

/// Enum for general chemical  element properties
///
/// ```
///   use ::Nuclide::{Element,ChemElement};
///    let lithium = Element::Li;
///    let am = lithium.am();
///    let ionization_lvl2 = lithium.ionization_energies(2).unwrap();
/// ```
///
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Element {
    H=1, He , Li , Be , B  , C  , N  , O  , F ,
    Ne , Na , Mg , Al , Si , P  , S  , Cl , Ar,
    K  , Ca , Sc , Ti , V  , Cr , Mn , Fe , Co,
    Ni , Cu , Zn , Ga , Ge , As , Se , Br , Kr,
    Rb , Sr , Y  , Zr , Nb , Mo , Tc , Ru , Rh,
    Pd , Ag , Cd , In , Sn , Sb , Te , I  , Xe,
    Cs , Ba , La , Ce , Pr , Nd , Pm , Sm , Eu,
    Gd , Tb , Dy , Ho , Er , Tm , Yb , Lu , Hf,
    Ta , W  , Re , Os , Ir , Pt , Au , Hg , Tl,
    Pb , Bi , Po , At , Rn , Fr , Ra , Ac , Th,
    Pa , U  , Np , Pu , Am , Cm , Bk , Cf , Es,
    Fm , Md , No , Lr , Rf , Db , Sg , Bh , Hs,
    Mt , Ds , Rg , Cn , Nh , Fl , Mc , Lv , Ts,
    Og ,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

// At which index does this element's list of isotope abundancies end?
// NOTE: Starts at index of previous element, if equal this element does not 
// exist naturally on Earth
#[rustfmt::skip]
const ABUNDANCE_LOOKUP_TABLE: [u16; 119] = [
    0, 2, /* H */ 4, /* He */ 6, /* Li */ 7, /* Be */ 9, /* B */ 12, /* C */
    14, /* N */ 17, /* O */ 18, /* F */ 21, /* Ne */ 22, /* Na */ 25, /* Mg */
    26, /* Al */ 29, /* Si */ 30, /* P */ 34, /* S */ 36, /* Cl */ 39, /* Ar */
    42, /* K */ 48, /* Ca */ 49, /* Sc */ 54, /* Ti */ 56, /* V */ 60, /* Cr */
    61, /* Mn */ 65, /* Fe */ 66, /* Co */ 71, /* Ni */ 73, /* Cu */ 78, /* Zn */
    80, /* Ga */ 85, /* Ge */ 86, /* As */ 92, /* Se */ 94, /* Br */ 100, /* Kr */
    102, /* Rb */ 106, /* Sr */ 107, /* Y */ 112, /* Zr */ 113, /* Nb */
    120, /* Mo */ 121, /* Tc */ 128, /* Ru */ 129, /* Rh */ 135, /* Pd */
    137, /* Ag */ 145, /* Cd */ 147, /* In */ 157, /* Sn */ 159, /* Sb */
    167, /* Te */ 168, /* I */  177, /* Xe */ 178, /* Cs */ 185, /* Ba */
    187, /* La */ 191, /* Ce */ 192, /* Pr */ 199, /* Nd */ 200, /* Pm */
    207, /* Sm */ 209, /* Eu */ 216, /* Gd */ 217, /* Tb */ 224, /* Dy */
    225, /* Ho */ 231, /* Er */ 232, /* Tm */ 239, /* Yb */ 241, /* Lu */
    247, /* Hf */ 249, /* Ta */ 254, /* W */  256, /* Re */ 263, /* Os */
    265, /* Ir */ 271, /* Pt */ 272, /* Au */ 279, /* Hg */ 281, /* Tl */
    285, /* Pb */ 286, /* Bi */ 287, /* Po */ 288, /* At */ 289, /* Rn */
    290, /* Fr */ 291, /* Ra */ 292, /* Ac */ 293, /* Th */ 294, /* Pa */
    297, /* U */  298, /* Np */ 299, /* Pu */ 300, /* Am */ 301, /* Cm */
    302, /* Bk */ 303, /* Cf */ 304, /* Es */ 305, /* Fm */ 306, /* Md */
    307, /* No */ 308, /* Lr */ 309, /* Rf */ 310, /* Db */ 311, /* Sg */
    312, /* Bh */ 313, /* Hs */ 314, /* Mt */ 315, /* Ds */ 316, /* Rg */
    317, /* Cn */ 318, /* Nh */ 319, /* Fl */ 320, /* Mc */ 321, /* Lv */
    322, /* Ts */ 323, /* Og */
];

const ABUNDANCE_TABLE: [(u16, f64); 323] = [
    (1, 0.999885), // 1H
    (2, 0.000115), // 2H
    (3, 0.00000134), // 3He
    (4, 0.99999866), // 4He
    (6, 0.0759), // 6Li
    (7, 0.9241), // 7Li
    (9, 1.0), // 9Be
    (10, 0.199), // 10B
    (11, 0.801), // 11B
    (12, 0.9893), // 12C
    (13, 0.0107), // 13C
    (14, 0.0), // 14C
    (14, 0.99636), // 14N
    (15, 0.00364), // 15N
    (16, 0.99757), // 16O
    (17, 0.00038), // 17O
    (18, 0.00205), // 18O
    (19, 1.0), // 19F
    (20, 0.9048), // 20Ne
    (21, 0.0027), // 21Ne
    (22, 0.0925), // 22Ne
    (23, 1.0), // 23Na
    (24, 0.7899), // 24Mg
    (25, 0.1000), // 25Mg
    (26, 0.1101), // 26Mg
    (27, 1.0), // 27Al
    (28, 0.92223), // 28Si
    (29, 0.04685), // 29Si
    (30, 0.03092), // 30Si
    (31, 1.0), // 31P
    (32, 0.9499), // 32S
    (33, 0.0075), // 33S
    (34, 0.0425), // 34S
    (36, 0.0001), // 36S
    (35, 0.7576), // 35Cl
    (37, 0.2424), // 37Cl
    (36, 0.003336), // 36Ar
    (38, 0.000629), // 38Ar
    (40, 0.996035), // 40Ar
    (39, 0.932581), // 39K
    (40, 0.000117), // 40K
    (41, 0.067302), // 41K
    (40, 0.96941), // 40Ca
    (42, 0.00647), // 42Ca
    (43, 0.00135), // 43Ca
    (44, 0.02086), // 44Ca
    (46, 0.00004), // 46Ca
    (48, 0.00187), // 48Ca
    (45, 1.0), // 45Sc
    (46, 0.0825), // 46Ti
    (47, 0.0744), // 47Ti
    (48, 0.7372), // 48Ti
    (49, 0.0541), // 49Ti
    (50, 0.0518), // 50Ti
    (50, 0.00250), // 50V
    (51, 0.99750), // 51V
    (50, 0.04345), // 50Cr
    (52, 0.83789), // 52Cr
    (53, 0.09501), // 53Cr
    (54, 0.02365), // 54Cr
    (55, 1.0), // 55Mn
    (54, 0.05845), // 54Fe
    (56, 0.91754), // 56Fe
    (57, 0.02119), // 57Fe
    (58, 0.00282), // 58Fe
    (59, 1.0), // 59Co
    (58, 0.68077), // 58Ni
    (60, 0.26223), // 60Ni
    (61, 0.011399), // 61Ni
    (62, 0.036346), // 62Ni
    (64, 0.009255), // 64Ni
    (63, 0.6915), // 63Cu
    (65, 0.3085), // 65Cu
    (64, 0.4917), // 64Zn
    (66, 0.2773), // 66Zn
    (67, 0.0404), // 67Zn
    (68, 0.1845), // 68Zn
    (70, 0.0061), // 70Zn
    (69, 0.60108), // 69Ga
    (71, 0.39892), // 71Ga
    (70, 0.2057), // 70Ge
    (72, 0.2745), // 72Ge
    (73, 0.0775), // 73Ge
    (74, 0.3650), // 74Ge
    (76, 0.0773), // 76Ge
    (75, 1.0), // 75As
    (74, 0.0089), // 74Se
    (76, 0.0937), // 76Se
    (77, 0.0763), // 77Se
    (78, 0.2377), // 78Se
    (80, 0.4961), // 80Se
    (82, 0.0873), // 82Se
    (79, 0.5069), // 79Br
    (81, 0.4931), // 81Br
    (78, 0.00355), // 78Kr
    (80, 0.02286), // 80Kr
    (82, 0.11593), // 82Kr
    (83, 0.11500), // 83Kr
    (84, 0.56987), // 84Kr
    (86, 0.17279), // 86Kr
    (85, 0.7217), // 85Rb
    (87, 0.2783), // 87Rb
    (84, 0.0056), // 84Sr
    (86, 0.0986), // 86Sr
    (87, 0.0700), // 87Sr
    (88, 0.8258), // 88Sr
    (89, 1.0), // 89Y
    (90, 0.5145), // 90Zr
    (91, 0.1122), // 91Zr
    (92, 0.1715), // 92Zr
    (94, 0.1738), // 94Zr
    (96, 0.0280), // 96Zr
    (93, 1.0), // 93Nb
    (92, 0.1453), // 92Mo
    (94, 0.0915), // 94Mo
    (95, 0.1584), // 95Mo
    (96, 0.1667), // 96Mo
    (97, 0.0960), // 97Mo
    (98, 0.2439), // 98Mo
    (100, 0.0982), // 100Mo
    (98, 1.0), // 98Tc
    (96, 0.0554), // 96Ru
    (98, 0.0187), // 98Ru
    (99, 0.1276), // 99Ru
    (100, 0.1260), // 100Ru
    (101, 0.1706), // 101Ru
    (102, 0.3155), // 102Ru
    (104, 0.1862), // 104Ru
    (103, 1.0), // 103Rh
    (102, 0.0102), // 102Pd
    (104, 0.1114), // 104Pd
    (105, 0.2233), // 105Pd
    (106, 0.2733), // 106Pd
    (108, 0.2646), // 108Pd
    (110, 0.1172), // 110Pd
    (107, 0.51839), // 107Ag
    (109, 0.48161), // 109Ag
    (106, 0.0125), // 106Cd
    (108, 0.0089), // 108Cd
    (110, 0.1249), // 110Cd
    (111, 0.1280), // 111Cd
    (112, 0.2413), // 112Cd
    (113, 0.1222), // 113Cd
    (114, 0.2873), // 114Cd
    (116, 0.0749), // 116Cd
    (113, 0.0429), // 113In
    (115, 0.9571), // 115In
    (112, 0.0097), // 112Sn
    (114, 0.0066), // 114Sn
    (115, 0.0034), // 115Sn
    (116, 0.1454), // 116Sn
    (117, 0.0768), // 117Sn
    (118, 0.2422), // 118Sn
    (119, 0.0859), // 119Sn
    (120, 0.3258), // 120Sn
    (122, 0.0463), // 122Sn
    (124, 0.0579), // 124Sn
    (121, 0.5721), // 121Sb
    (123, 0.4279), // 123Sb
    (120, 0.0009), // 120Te
    (122, 0.0255), // 122Te
    (123, 0.0089), // 123Te
    (124, 0.0474), // 124Te
    (125, 0.0707), // 125Te
    (126, 0.1884), // 126Te
    (128, 0.3174), // 128Te
    (130, 0.3408), // 130Te
    (127, 1.0), // 127I
    (124, 0.000952), // 124Xe
    (126, 0.000890), // 126Xe
    (128, 0.019102), // 128Xe
    (129, 0.264006), // 129Xe
    (130, 0.040710), // 130Xe
    (131, 0.212324), // 131Xe
    (132, 0.269086), // 132Xe
    (134, 0.104357), // 134Xe
    (136, 0.088573), // 136Xe
    (133, 1.0), // 133Cs
    (130, 0.00106), // 130Ba
    (132, 0.00101), // 132Ba
    (134, 0.02417), // 134Ba
    (135, 0.06592), // 135Ba
    (136, 0.07854), // 136Ba
    (137, 0.11232), // 137Ba
    (138, 0.71698), // 138Ba
    (138, 0.0008881), // 138La
    (139, 0.9991119), // 139La
    (136, 0.00185), // 136Ce
    (138, 0.00251), // 138Ce
    (140, 0.88450), // 140Ce
    (142, 0.11114), // 142Ce
    (141, 1.0), // 141Pr
    (142, 0.27152), // 142Nd
    (143, 0.12174), // 143Nd
    (144, 0.23798), // 144Nd
    (145, 0.08293), // 145Nd
    (146, 0.17189), // 146Nd
    (148, 0.05756), // 148Nd
    (150, 0.05638), // 150Nd
    (145, 1.0), // 145Pm
    (144, 0.0307), // 144Sm
    (147, 0.1499), // 147Sm
    (148, 0.1124), // 148Sm
    (149, 0.1382), // 149Sm
    (150, 0.0738), // 150Sm
    (152, 0.2675), // 152Sm
    (154, 0.2275), // 154Sm
    (151, 0.4781), // 151Eu
    (153, 0.5219), // 153Eu
    (152, 0.0020), // 152Gd
    (154, 0.0218), // 154Gd
    (155, 0.1480), // 155Gd
    (156, 0.2047), // 156Gd
    (157, 0.1565), // 157Gd
    (158, 0.2484), // 158Gd
    (160, 0.2186), // 160Gd
    (159, 1.0), // 159Tb
    (156, 0.00056), // 156Dy
    (158, 0.00095), // 158Dy
    (160, 0.02329), // 160Dy
    (161, 0.18889), // 161Dy
    (162, 0.25475), // 162Dy
    (163, 0.24896), // 163Dy
    (164, 0.28260), // 164Dy
    (165, 1.0), // 165Ho
    (162, 0.00139), // 162Er
    (164, 0.01601), // 164Er
    (166, 0.33503), // 166Er
    (167, 0.22869), // 167Er
    (168, 0.26978), // 168Er
    (170, 0.14910), // 170Er
    (169, 1.0), // 169Tm
    (168, 0.00123), // 168Yb
    (170, 0.02982), // 170Yb
    (171, 0.1409), // 171Yb
    (172, 0.2168), // 172Yb
    (173, 0.16103), // 173Yb
    (174, 0.32026), // 174Yb
    (176, 0.12996), // 176Yb
    (175, 0.97401), // 175Lu
    (176, 0.02599), // 176Lu
    (174, 0.0016), // 174Hf
    (176, 0.0526), // 176Hf
    (177, 0.1860), // 177Hf
    (178, 0.2728), // 178Hf
    (179, 0.1362), // 179Hf
    (180, 0.3508), // 180Hf
    (180, 0.0001201), // 180Ta
    (181, 0.9998799), // 181Ta
    (180, 0.0012), // 180W
    (182, 0.2650), // 182W
    (183, 0.1431), // 183W
    (184, 0.3064), // 184W
    (186, 0.2843), // 186W
    (185, 0.3740), // 185Re
    (187, 0.6260), // 187Re
    (184, 0.0002), // 184Os
    (186, 0.0159), // 186Os
    (187, 0.0196), // 187Os
    (188, 0.1324), // 188Os
    (189, 0.1615), // 189Os
    (190, 0.2626), // 190Os
    (192, 0.4078), // 192Os
    (191, 0.373), // 191Ir
    (193, 0.627), // 193Ir
    (190, 0.00012), // 190Pt
    (192, 0.00782), // 192Pt
    (194, 0.3286), // 194Pt
    (195, 0.3378), // 195Pt
    (196, 0.2521), // 196Pt
    (198, 0.07356), // 198Pt
    (197, 1.0), // 197Au
    (196, 0.0015), // 196Hg
    (198, 0.0997), // 198Hg
    (199, 0.1687), // 199Hg
    (200, 0.2310), // 200Hg
    (201, 0.1318), // 201Hg
    (202, 0.2986), // 202Hg
    (204, 0.0687), // 204Hg
    (203, 0.2952), // 203Tl
    (205, 0.7048), // 205Tl
    (204, 0.014), // 204Pb
    (206, 0.241), // 206Pb
    (207, 0.221), // 207Pb
    (208, 0.524), // 208Pb
    (209, 1.0), // 209Bi
    (209, 1.0), // 209Po
    (210, 1.0), // 210At
    (222, 1.0), // 222Rn
    (223, 1.0), // 223Fr
    (226, 1.0), // 226Ra
    (227, 1.0), // 227Ac
    (232, 1.0), // 232Th
    (231, 1.0), // 231Pa
    (234, 0.000054), // 234U
    (235, 0.007204), // 235U
    (238, 0.992742), // 238U
    (237, 1.0), // 237Np
    (244, 1.0), // 244Pu
    (243, 1.0), // 243Am
    (247, 1.0), // 247Cm
    (247, 1.0), // 247Bk
    (251, 1.0), // 251Cf
    (252, 1.0), // 252Es
    (257, 1.0), // 257Fm
    (258, 1.0), // 258Md
    (259, 1.0), // 259No
    (262, 1.0), // 262Lr
    (267, 1.0), // 267Rf
    (268, 1.0), // 268Db
    (271, 1.0), // 271Sg
    (272, 1.0), // 272Bh
    (270, 1.0), // 270Hs
    (276, 1.0), // 276Mt
    (281, 1.0), // 281Ds
    (280, 1.0), // 280Rg
    (285, 1.0), // 285Cn
    (286, 1.0), // 286Nh
    (290, 1.0), // 290Fl
    (290, 1.0), // 290Mc
    (293, 1.0), // 293Lv
    (294, 1.0), // 294Ts
    (294, 1.0), // 294Og
];

impl Element {
    pub const fn protons(&self) -> u8 {
        *self as u8
    }

    pub const fn from_protons(protons: u8) -> Element {
        if protons == 0 || protons > Element::Og.protons() {
            panic!("Invalid number of protons for an element");
        }

        unsafe { std::mem::transmute(protons) }
    }

    pub const fn symbol(&self) -> &'static str {
        SYMBOL[*self as usize - 1]
    }

    /// Fraction as measured from samples on Earth. For non-abundant elements,
    /// returns only the most stable isotope.
    pub fn abundant_nuclides(&self) -> NuclideFraction {
        NuclideFraction::from(*self)
    }

    /// Iterate through all elements
    pub fn iter() -> impl Iterator<Item = Element> {
        (1..=Element::Og.protons()).map(Element::from_protons)
    }
}

impl ChemElement for Element {
    fn atomic_num(&self) -> u64 {
        *self as u64
    }

    fn am(&self) -> f64 {
        NuclideFraction::from(*self).am()
    }

    fn electron_affinity(&self) -> f64 {
        ELECTRON_AFFINITY[*self as usize - 1]
    }

    fn ionization_energies(&self, level: usize) -> Option<f64> {
        let z = self.atomic_num();

        if z > 110 || level == 0 || level > z as usize {
            return None
        }

        Some(
            IONIZATION_ENERGIES[((((z * (z + 1)) >> 1)
                - z)
                + level as u64
                - 1) as usize],
        )
    }

    fn electronegativity(&self) -> f64 {
        THERMOCHEMICAL_ELECTRO_NEGATIVE[*self as usize - 1]
    }

    fn mullikan_en(&self) -> f64 {
        (self.ionization_energies(1).unwrap() + ELECTRON_AFFINITY[*self as usize - 1])
            * 1.97E-3
            + 0.19
    }

    fn allen_en(&self) -> f64 {
        ALLEN_ELECTRO[*self as usize - 1]
    }

    fn pauling_en(&self) -> f64 {
        PAULING_ELECTRO[*self as usize - 1]
    }

    fn covalent_radii(&self, bond: usize) -> Option<f64> {
        if bond > 0 && bond < 4 {
            Some(COVALENT_RADII[(*self as usize - 1) * 3 + bond - 1])
        } else {
            None
        }
    }

    fn ionic_radii(&self) -> f64 {
        IONIC_RADII[*self as usize - 1]
    }

    fn vdr_crystal(&self) -> f64 {
        VAN_DER_WAAL_CRYSTAL[*self as usize - 1]
    }

    fn vdr_isolated(&self) -> f64 {
        VAN_DER_WAAL_ISOLATED[*self as usize - 1]
    }
}

/// Fraction as measured from samples on Earth. For non-abundant elements,
/// returns only the most stable isotope.
impl From<Element> for NuclideFraction {
    fn from(element: Element) -> Self {
        let z = element as usize;
        let start = ABUNDANCE_LOOKUP_TABLE[z - 1] as usize;
        let end = ABUNDANCE_LOOKUP_TABLE[z] as usize;
        NuclideFraction::from_nucleon_fractions(
            element as u8, 
            &ABUNDANCE_TABLE[start..end]
        ).expect("Valid abundance table for these elements")
    }
}
