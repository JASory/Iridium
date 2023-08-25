/// Emit K protons
pub struct ProtonEmission<const K : usize>;

/// Emit K neutrons
pub struct NeutronEmission<const K : usize>;

/// Emit K Alpha particles
pub struct AlphaEmission<const K : usize>;
/// Emit Alpha particle and K neutrons
pub struct AlphaNeutron<const K : usize>;
/// Emit Neutron and Deuteron
pub struct NeutronDeuteron;
/// Emit Neutron and Triton
pub struct NeutronTriton;
/// K Electron captures (EC)
pub struct ElectronCapture<const K: usize>;
/// K electron emissions (B-)
pub struct ElectronEmission<const K: usize>;
/// Electron emission and K neutrons (B- Kn)
pub struct ElectronNeutron<const K: usize>;
/// Electron emission and K protons (B- Kp)
pub struct ElectronProton<const K: usize>;
/// Electron emission and Alpha particle (B- A)
pub struct ElectronAlpha;
/// Electron emission and Deuteron particle (B- D)
pub struct ElectronDeuteron;
/// Electron emission and Triton particle (B- T)
pub struct ElectronTriton;
/// Electron emission and Spontaneous fission
pub struct ElectronFission;
///K Positron emissions (KB+)
pub struct PositronEmission<const K: usize>;
/// Positron emission and K protons (B+ Kp)
pub struct PositronProton<const K: usize>;
/// Positron emission and Alpha particle
pub struct PositronAlpha;
/// Positron emission and Spontaneous fission
pub struct PositronFission;
/// Spontaneous fission
pub struct SpontaneousFission;
/// Decay from nuclide index
pub struct ClusterDecay<const D: usize>;
/// Decay two clusters
pub struct DoubleCluster<const D: usize, const S: usize>;
/// Full set of possible decay modes
pub struct TotalDecay;

