use std::cmp::Ordering;

const SUBSHELL_SYMBOLS : [&str;6] = ["s", "p", "d", "f", "g", "h"];

/// Enum for electron spin (spin quantum number) values
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ElectronSpin  {
    MinusOneHalf = -1,
    PlusOneHalf = 1,
}

impl ElectronSpin  {
    pub fn flip_flop(&mut self) {
        *self = match self {
            ElectronSpin::MinusOneHalf => ElectronSpin::PlusOneHalf,
            ElectronSpin::PlusOneHalf => ElectronSpin::MinusOneHalf,
        }
    }
}

/// Enum for electron "location" characteristic
/// TODO: add and handle Valence
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Location  {
    Core,
    Outer,
}

/// Enum for electron energy state
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnergyState  {
    GroundState,
    Excited(u8),
}

/// Struct representing electron in terms of its basic quantum features (quantum numbers).
/// 
/// Also, two additional properties are provided that should specify electron's location
/// (inner or outer), and its excitation state.
#[derive(Debug, Copy, Clone)]
pub struct QuantumElectron {
    pub p_qn: u8, //Principal Quantum Number
    pub oam_qn: u8, //Orbital Angular Momentum Quantum Number,
    m_qn: i8, //Magnetic Quantum Number
    spin: ElectronSpin,
    loc : Location,
    state: EnergyState,
}

impl QuantumElectron {
    pub fn new(p_qn: u8, oam_qn: u8, m_qn: i8, spin: ElectronSpin, loc: Location, state: EnergyState) -> QuantumElectron {
        if p_qn == 0 || oam_qn >= p_qn || m_qn.abs() as u8 >= p_qn {
            panic!("Invalid quantum electron");
        }
        QuantumElectron { p_qn, oam_qn, m_qn, spin, loc, state }
    }

    pub fn subshell_symbol(&self) -> &'static str {
        SUBSHELL_SYMBOLS[self.oam_qn as usize]
    }
}

impl PartialOrd for QuantumElectron {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QuantumElectron {
    /// Default ordering in accordance with Madelung rule
    fn cmp(&self, other: &Self) -> Ordering {
        (self.p_qn + self.oam_qn).cmp(&(other.p_qn + other.oam_qn))
            .then(self.p_qn.cmp(&other.p_qn))
            .then(self.m_qn.cmp(&other.m_qn))
            .then(self.spin.cmp(&other.spin))

    }
}

impl PartialEq for QuantumElectron {
    fn eq(&self, other: &Self) -> bool {
        self.p_qn == other.p_qn && self.oam_qn == other.oam_qn && self.m_qn == other.m_qn
            && self.spin == other.spin && self.loc == other.loc && self.state == other.state
    }
}

impl Eq for QuantumElectron {}
