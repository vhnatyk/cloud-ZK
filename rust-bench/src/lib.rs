use std::any::TypeId;
use std::mem::size_of;

use ark_ec::{msm::VariableBaseMSM, AffineCurve};
use ark_ff::{Field, PrimeField, ToBytes};
use num_bigint::BigUint;
pub use ::rust_rw_device::curve;
use rust_rw_device::rw_msm_to_dram;
use std::ops::Mul;

use curve::{Fq, Fq2, G1Affine, G2Affine};
use rw_msm_to_dram as device_g1; //TODO: unify to one crate
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

pub fn msm_cloud_generic<G: AffineCurve>(
    //TODO: the result conversion code looks very close for G1 and G2, should be unified
    //TODO: no biguint API, just Arkworks or raw byte array
    points: &[u8],
    scalars: &[u8],
) -> (G::Projective, u8) {
    let mut buff: Vec<u8>;
    let scalar_size = size_of::<G::ScalarField>();
    let size = scalars.len() / scalar_size;

    let label = {
        if TypeId::of::<G>() == TypeId::of::<G1Affine>() { 
            let (result, _, label) = device_g1::msm_calc(&points, &scalars, size);
            //TODO: this conversion should be part of Ingo API
            let proj_x_field = Fq::from_random_bytes(&result[0]).unwrap();
            let proj_y_field = Fq::from_random_bytes(&result[1]).unwrap();
            let proj_z_field = Fq::from_random_bytes(&result[2]).unwrap();
            let aff_x = proj_x_field.mul(proj_z_field.inverse().unwrap());
            let aff_y = proj_y_field.mul(proj_z_field.inverse().unwrap());
            buff = Vec::<u8>::with_capacity(size_of::<G1Affine>());
            aff_x.write(&mut buff).unwrap();
            aff_y.write(&mut buff).unwrap();
            label
        } else if TypeId::of::<G>() == TypeId::of::<G2Affine>() {         
            let (result, _, label) = device_g2::msm_calc(&points, &scalars, size);
            let proj_x_field =
                Fq2::from_random_bytes(&[result[5].to_vec(), result[2].to_vec()].concat()).unwrap();
            let proj_y_field =
                Fq2::from_random_bytes(&[result[4].to_vec(), result[1].to_vec()].concat()).unwrap();
            let proj_z_field =
                Fq2::from_random_bytes(&[result[3].to_vec(), result[0].to_vec()].concat()).unwrap();

            let aff_x = proj_x_field.mul(proj_z_field.inverse().unwrap());
            let aff_y = proj_y_field.mul(proj_z_field.inverse().unwrap());
            buff = Vec::<u8>::with_capacity(size_of::<G2Affine>());
            aff_x.write(&mut buff).unwrap();
            aff_y.write(&mut buff).unwrap();
            label
        } else {
            panic!("unsupported curve type")
        }
    };
    buff.push(0);
    (G::read(buff.as_slice()).unwrap().into_projective(), label)
}

pub fn msm_cloud<G: AffineCurve>(
    points: &[u8],
    scalars: &[u8],
) -> (Vec<Vec<u8>>, u8) {
    //TODO: no biguint API, just Arkworks or raw byte array
    let ret = {
        if TypeId::of::<G>() == TypeId::of::<G1Affine>() {
            device_g1::msm_calc(&points, &scalars, scalars.len())
        } else if TypeId::of::<G>() == TypeId::of::<G2Affine>() {
            device_g2::msm_calc(&points, &scalars, scalars.len())
        } else {
            panic!("unsupported curve type")
        }
    };

    (ret.0, ret.2)
}
