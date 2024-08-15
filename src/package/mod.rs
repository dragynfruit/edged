use ::flatbuffers::{buffer_has_identifier, Verifiable, Verifier};

use crate::flatbuffers::{self, executable::platforms::darwinn::Executable, Package};

pub struct PkgRegistry {
}
impl PkgRegistry {
    pub fn new() {
    }
    pub fn get_executables_from_binary(&self, bin: &[u8]) {
        if !buffer_has_identifier(bin, "DWN1", false) {
            panic!();
        }
        Executable::run_verifier(&mut Verifier::new(&Default::default(), bin), 4).unwrap();

        let pkg = ::flatbuffers::root::<Package>(bin).unwrap();
        
        let min_runtime = pkg.min_runtime_version();
        if min_runtime < 10 {
            panic!("invalid runtime version");
        } else if min_runtime > 14 {
            panic!("need a newer runtime version");
        }

        if pkg.virtual_chip_id() == -1 {
            panic!("multi-chip package");
        }

        if pkg.serialized_multi_executable().unwrap().len() == 0 {
            panic!("no executables to register");
        }

        Executable::input_layers(&self).unwrap().get(0);
    }
}
