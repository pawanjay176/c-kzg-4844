/* automatically generated by rust-bindgen 0.61.0 */

include!("./consts.rs");

use std::ops::{Deref, DerefMut};

use libc::FILE;

pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_PROOF: usize = 48;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_BLOB: usize = FIELD_ELEMENTS_PER_BLOB * BYTES_PER_FIELD_ELEMENT;

pub type byte = u8;
pub type limb_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_scalar {
    pub b: [byte; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_fr {
    pub l: [limb_t; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp {
    pub l: [limb_t; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp2 {
    pub fp: [blst_fp; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp6 {
    pub fp2: [blst_fp2; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp12 {
    pub fp6: [blst_fp6; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_p1 {
    pub x: blst_fp,
    pub y: blst_fp,
    pub z: blst_fp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_p1_affine {
    pub x: blst_fp,
    pub y: blst_fp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_p2 {
    pub x: blst_fp2,
    pub y: blst_fp2,
    pub z: blst_fp2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct blst_p2_affine {
    pub x: blst_fp2,
    pub y: blst_fp2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BLSFieldElement {
    pub bytes: [u8; 32usize],
}

impl Deref for BLSFieldElement {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Blob {
    bytes: [u8; BYTES_PER_BLOB],
}

impl Deref for Blob {
    type Target = [u8; BYTES_PER_BLOB];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KZGProof {
    bytes: [u8; 48usize],
}

impl Deref for KZGProof {
    type Target = [u8; 48];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

pub const FIAT_SHAMIR_PROTOCOL_DOMAIN: &[u8; 17usize] = b"FSBLOBVERIFY_V1_\0";
pub type g1_t = blst_p1;
pub type g2_t = blst_p2;
pub type fr_t = blst_fr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KZGCommitment {
    bytes: [u8; 48usize],
}

impl Deref for KZGCommitment {
    type Target = [u8; 48];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

#[repr(u32)]
#[doc = " The common return type for all routines in which something can go wrong."]
#[doc = ""]
#[doc = " @warning In the case of @p C_KZG_OK or @p C_KZG_BADARGS, the caller can assume that all memory allocated by the"]
#[doc = " called routines has been deallocated. However, in the case of @p C_KZG_ERROR or @p C_KZG_MALLOC being returned, these"]
#[doc = " are unrecoverable and memory may have been leaked."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum C_KZG_RET {
    #[doc = "< Success!"]
    C_KZG_OK = 0,
    #[doc = "< The supplied data is invalid in some way"]
    C_KZG_BADARGS = 1,
    #[doc = "< Internal error - this should never occur and may indicate a bug in the library"]
    C_KZG_ERROR = 2,
    #[doc = "< Could not allocate memory"]
    C_KZG_MALLOC = 3,
}
#[doc = " Stores the setup and parameters needed for performing FFTs."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FFTSettings {
    #[doc = "< The maximum size of FFT these settings support, a power of 2."]
    max_width: u64,
    #[doc = "< Ascending powers of the root of unity, size `width + 1`."]
    expanded_roots_of_unity: *const fr_t,
    #[doc = "< Descending powers of the root of unity, size `width + 1`."]
    reverse_roots_of_unity: *const fr_t,
    #[doc = "< Powers of the root of unity in bit-reversal permutation, size `width`."]
    roots_of_unity: *const fr_t,
}

#[doc = " Stores the setup and parameters needed for computing KZG proofs."]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KZGSettings {
    #[doc = "< The corresponding settings for performing FFTs"]
    fs: *const FFTSettings,
    #[doc = "< G1 group elements from the trusted setup, in Lagrange form bit-reversal permutation"]
    g1_values: *const g1_t,
    #[doc = "< G2 group elements from the trusted setup; both arrays have FIELD_ELEMENTS_PER_BLOB elements"]
    g2_values: *const g2_t,
}

/// Safety: FFTSettings is initialized once on calling `load_trusted_setup`. After
/// that, the struct is never modified. The memory for the arrays within `FFTSettings` and
/// `g1_values` and `g2_values` are only freed on calling `free_trusted_setup` which only happens
/// when we drop the struct.
unsafe impl Sync for KZGSettings {}
unsafe impl Send for KZGSettings {}

extern "C" {
    #[doc = " Interface functions"]
    pub fn load_trusted_setup(
        out: *mut KZGSettings,
        g1_bytes: *const u8, /* n1 * 48 bytes */
        n1: usize,
        g2_bytes: *const u8, /* n2 * 96 bytes */
        n2: usize,
    ) -> C_KZG_RET;
}
extern "C" {
    pub fn load_trusted_setup_file(out: *mut KZGSettings, in_: *mut FILE) -> C_KZG_RET;
}
extern "C" {
    pub fn free_trusted_setup(s: *mut KZGSettings);
}
extern "C" {
    pub fn compute_aggregate_kzg_proof(
        out: *mut KZGProof,
        blobs: *const u8, // pointer to the first byte in a 2D array ([[u8; BYTES_PER_BLOB]])
        n: usize,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
}
extern "C" {
    pub fn verify_aggregate_kzg_proof(
        out: *mut bool,
        blobs: *const u8, // pointer to the first byte in a 2D array ([[u8; BYTES_PER_BLOB]])
        expected_kzg_commitments: *const KZGCommitment,
        n: usize,
        kzg_aggregated_proof: *const KZGProof,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
}
extern "C" {
    pub fn blob_to_kzg_commitment(
        out: *mut KZGCommitment,
        blob: *mut u8,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
}
extern "C" {
    pub fn verify_kzg_proof(
        out: *mut bool,
        polynomial_kzg: *const KZGCommitment,
        z: *const u8,
        y: *const u8,
        kzg_proof: *const KZGProof,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
}
