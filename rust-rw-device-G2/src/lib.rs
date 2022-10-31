use std::{ops::{Mul, Add}, str::FromStr};

use num_bigint::BigUint;

use crate::rw_msm_to_dram::*;

pub mod rw_msm_to_dram;

use ark_ec::{ProjectiveCurve, AffineCurve};
use ark_ff::{PrimeField, Field, BigInteger, Zero, QuadExtField, One};
use ark_bn254::{G2Projective as G2Projective, G2Affine as G2Affine, Fr, g2, Fq};
use ark_std::{UniformRand};
use pbr::ProgressBar;

pub fn result_check_biguint(result: [Vec<u8>; 6], msm_result:G2Affine) {
    let proj_x_field_c0 = Fq::from_le_bytes_mod_order(&result[5]);
    let proj_x_field_c1 = Fq::from_le_bytes_mod_order(&result[2]);
    let proj_x_field = QuadExtField::new(proj_x_field_c0,proj_x_field_c1);
    let proj_y_field_c0 = Fq::from_le_bytes_mod_order(&result[4]);
    let proj_y_field_c1 = Fq::from_le_bytes_mod_order(&result[1]);
    let proj_y_field = QuadExtField::new(proj_y_field_c0,proj_y_field_c1);
    let proj_z_field_c0 = Fq::from_le_bytes_mod_order(&result[3]);
    let proj_z_field_c1 = Fq::from_le_bytes_mod_order(&result[0]);
    let proj_z_field = QuadExtField::new(proj_z_field_c0,proj_z_field_c1);

    let aff_x = proj_x_field.mul(proj_z_field.inverse().unwrap());
    let aff_y = proj_y_field.mul(proj_z_field.inverse().unwrap());
    let point = G2Affine::new(aff_x,aff_y,false);

    // let point_tmp = G2Projective::new(proj_x_field, proj_y_field,proj_z_field);
    // let point = point_tmp.into_affine();
    
    println!("point.x Re bytes {:02X?}", &point.x.c0.into_repr().to_bytes_le());
    println!("point.x Re bytes {:02X?}", &point.x.c1.into_repr().to_bytes_le());
    println!("point.y Re bytes {:02X?}", &point.y.c0.into_repr().to_bytes_le());
    println!("point.y Im bytes {:02X?}", &point.y.c1.into_repr().to_bytes_le());


    println!("Is point on the curve {}",point.is_on_curve());
    println!("Is Result Equal To Expected {}", point.to_string() == msm_result.to_string());
}

pub fn input_generator_biguint(nof_elements: usize) -> (Vec<BigUint>, Vec<BigUint>, G2Affine, Vec<G2Affine>, Vec<Fr>) {
    let mut rng = ark_std::rand::thread_rng();
    let mut points: Vec<BigUint> = Vec::new();
    let mut scalars: Vec<BigUint>  = Vec::new();
    let mut points_ga: Vec<G2Affine> = Vec::new();
    let mut scalars_fr: Vec<Fr>  = Vec::new();
    let mut msm_result = G2Affine::zero();
    // let mut pb = ProgressBar::new(nof_elements.try_into().unwrap());
    // pb.format("╢▌▌░╟");
    for i in 0..nof_elements{
        // pb.inc();
        // let x0 = BigUint::from_str("10857046999023057135944570762232829481370756359578518086990519993285655852781").unwrap();
        // let x1 = BigUint::from_str("11559732032986387107991004021392285783925812861821192530917403151452391805634").unwrap();
        // let y0 = BigUint::from_str("8495653923123431417604973247489272438418190587263600148770280649306958101930").unwrap();
        // let y1 = BigUint::from_str("4082367875863433681332203403145435568316851327593401208105741076214120093531").unwrap();
        // let x0_field = Fq::from_le_bytes_mod_order(&x0.to_bytes_le());
        // let x1_field = Fq::from_le_bytes_mod_order(&x1.to_bytes_le());
        // let y0_field = Fq::from_le_bytes_mod_order(&y0.to_bytes_le());
        // let y1_field = Fq::from_le_bytes_mod_order(&y1.to_bytes_le());
        // let aff = G2Affine::new(QuadExtField::new(x0_field,x1_field),QuadExtField::new(y0_field,y1_field),false);

        let aff = G2Projective::rand(&mut rng).into_affine();

        // println!("x Re {}", Fq::from_le_bytes_mod_order(&aff.y.c1.into_repr().to_bytes_le()));
        // println!("x Im {}",  Fq::from_le_bytes_mod_order(&aff.x.c1.into_repr().to_bytes_le()));
        // println!("y Re {}",  Fq::from_le_bytes_mod_order(&aff.y.c0.into_repr().to_bytes_le()));
        // println!("y Im {}",  Fq::from_le_bytes_mod_order(&aff.x.c0.into_repr().to_bytes_le()));

        println!("aff.y Im bytes {:02X?}", &aff.y.c1.into_repr().to_bytes_le());
        println!("aff.x Im bytes {:02X?}", &aff.x.c1.into_repr().to_bytes_le());
        println!("aff.y Re bytes {:02X?}", &aff.y.c0.into_repr().to_bytes_le());
        println!("aff.x Re bytes {:02X?}", &aff.x.c0.into_repr().to_bytes_le());

        points.push(BigUint::from_bytes_le(&aff.y.c1.into_repr().to_bytes_le()));
        points.push(BigUint::from_bytes_le(&aff.x.c1.into_repr().to_bytes_le()));
        points.push(BigUint::from_bytes_le(&aff.y.c0.into_repr().to_bytes_le()));
        points.push(BigUint::from_bytes_le(&aff.x.c0.into_repr().to_bytes_le()));

        // let mut scalar = Fr::zero();
        // if i % 2 == 1{
        //     scalar = Fr::rand(&mut rng);
        // }

        let mut scalar = Fr::from_le_bytes_mod_order(&Fr::rand(&mut rng).into_repr().to_bytes_le()[0..31]);
        println!("scalar bytes {:02X?}", &scalar.into_repr().to_bytes_le());
        scalars.push(BigUint::from_bytes_le(&scalar.into_repr().to_bytes_le()));
        
        if msm_result.is_zero(){
            msm_result = aff.mul(scalar).into_affine();
        }
        else{
            msm_result = msm_result.add(aff.mul(scalar).into_affine());
        }

        points_ga.push(aff);
        scalars_fr.push(scalar);
    }
    // pb.finish_print("Done Generation...");



    println!("msm_result.y Im bytes {:02X?}", &msm_result.y.c1.into_repr().to_bytes_le());
    println!("msm_result.x Im bytes {:02X?}", &msm_result.x.c1.into_repr().to_bytes_le());
    println!("msm_result.y Re bytes {:02X?}", &msm_result.y.c0.into_repr().to_bytes_le());
    println!("msm_result.x Re bytes {:02X?}", &msm_result.x.c0.into_repr().to_bytes_le());
    println!("Point on curve {}", msm_result.is_on_curve());

    (points, scalars, msm_result, points_ga, scalars_fr)
}
