use std::any::TypeId;

use ark_ec::msm::VariableBaseMSM;
use ark_ec::AffineCurve;
use ark_ff::PrimeField;
use num_bigint::BigUint;

use rust_rw_device::curve::{G1Affine, G2Affine};
use rust_rw_device::rw_msm_to_dram as device_g1; //TODO: unify to one crate
use rust_rw_device_G2::rw_msm_to_dram as device_g2; //TODO: unify to one crate

pub mod util;

pub fn msm_ark<G: AffineCurve>(
    points: &[G],
    scalars: &[<G::ScalarField as PrimeField>::BigInt],
) -> G::Projective {
    let npoints = points.len();
    if npoints != scalars.len() {
        panic!("length mismatch")
    }

    let ret = VariableBaseMSM::multi_scalar_mul(points, scalars);
    ret
}

pub fn msm_cloud<G: AffineCurve>(
    points: &Vec<BigUint>,
    scalars: &Vec<BigUint>,
) -> (Vec<Vec<u8>>, u8) {
    //TODO: no biguint API, just Arkworks or raw byte array
    let ret = {
        if TypeId::of::<G>() == TypeId::of::<G1Affine>() {
            assert_eq!(scalars.len(), points.len() / 2, "length mismatch");
            device_g1::msm_calc(&points, &scalars, scalars.len())
        } else if TypeId::of::<G>() == TypeId::of::<G2Affine>() {
            assert_eq!(scalars.len(), points.len() / 4, "length mismatch");
            device_g2::msm_calc(&points, &scalars, scalars.len())
        } else {
            panic!("unsupported curve type")
        }
    };

    (ret.0, ret.2)
}
