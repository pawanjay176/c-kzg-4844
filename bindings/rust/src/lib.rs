
// extern crate blst;
include!(concat!(env!("OUT_DIR"), "/generated.rs"));


    pub fn verify_kzg_proof_wrapper(
    ) -> bool {
        let mut verified: bool = false;
        unsafe {
            verify_kzg_proof(
                &mut verified,
            );
                verified
        }
    }

