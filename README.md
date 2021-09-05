# Iridium
Atomic Physics Library

## Nuclide 
  This is a nuclide database meant to address two issues. The lack of any nuclide databases in Rust, and the generally limited
 and outdated information other databases have (frequently copying data from Wikipedia or T. Gray's periodic table, which are outdated in there own right). The current version of this library has data on 3584 nuclides in there ground-base states. The primary sources are NUBASE2020[8], the ENDSF[9], and various Nuclear Data Sheets.
  In cases of ambiguity, estimates are made by the author based on trends from the neighboring nuclides. 
  
                                                                               -Sory, J.A
#### Available Data
- Electronegative Values predicted for the thermochemical electronega-
tivity, currently the best predictor of experimental values[1].
- Pauling  Electronegativity using the Pauling scale, mostly kept for completeness as
the Pauling scale performs poorly in application [3].
- Allen Electronegativity using the Allen scale[2]
- Electron Affinities of the elements, in kilojoules/mol
- Ionization Energies of all known electron configurations[]
- Covalent Radii The covalent radii calculated from single[5], double[6] and triple[7] bonds
- Van der Waal radii approximated in crystalline structures and isolated atoms. 
Unlike most libraries and databases which use Bondi’s approximations to the van der Waal
radius, Batsanov’s approximations are used here [4].
- Half-life, mean lifetime and decay constant for each nuclide.
- Atomic mass, binding energy and mass defect for each nuclide.

#### Features                                        
In addition to providing the previous information this library has some other features. 
- Simple modeling of decay including decay particle energies
- Prediction of mass and binding energies of theorectical nuclides using Bethe-Weiszacker Liquid Drop model
See the periodic table repository for a simple look-up gui implementation. 


#### References
[1] Tantardini, C., Oganov, A.R. ”Thermochemical Electronegatives of the elements”. Nature Commu-
nications 12, 2087 (2021).doi:10.1038/s41467-021-22429-0

[2] Allen, L.C. ”Electronegativity is the average one-electron energy of the valence-shell electrons in
ground-state free atoms”. Journal of the American Chemical Society. 111(25),1989. pp. 9003-9014.
doi:10.1021/ja00207a003

[3] Lynne Reed Murphy, Terry L. Meek, A. Louis Allred, and Leland C. Allen. ”Evaluation and Test
of Pauling’s Electronegativity Scale”. The Journal of Physical Chemistry. vol. 104 (24), 2000. pp.
5867-5871 doi:10.1021/jp000288e

[4] Batsanov, S.S. ”Van der Waals Radii of Elements”.Inorganic Materials Vol 37, 2001. pp. 871-
885.doi:10.1023/A:1011625728803

[5] Pyykkö, P., Atsumi, M. (2009). Molecular Single-Bond Covalent Radii for Elements 1-118. Chemistry
 A European Journal, 15(1), 186–197. doi:10.1002/chem.200800987
 
[6] Pyykkö, P., Atsumi, M. (2009). Molecular Double-Bond Covalent Radii for Elements Li–E112. Chem-
istry - A European Journal, 15(46), 12770–12779. doi:10.1002/chem.200901472

[7] Pyykkö, P., Riedel, S., Patzschke, M. (2005). Triple-Bond Covalent Radii. Chemistry - A European
Journal, 11(12), 3511–3520. doi:10.1002/chem.20040129

[8] Kondev, F.G et al. ”The NUBASE2020 evaluation of nuclear physics properties”. 2021 Chinese Phys.
C45 030001

[9] Brookhaven National Laboratory. https://www.nndc.bnl.gov/ensdf/. Accessed Jul 16 2021.
