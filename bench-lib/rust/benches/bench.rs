use criterion::{criterion_group, criterion_main, Criterion};
use next_gen_signatures::{Engine, BASE64_URL_SAFE_NO_PAD};
use signatures_bench::{prepare, zkp_issue, zkp_present, zkp_verify, zkp_verify_native};
use zkp_util::ecdsa_pops::halo2curves::ff::derive::byteorder::{BigEndian, WriteBytesExt};

use std::{
    hint::black_box,
    io::{BufWriter, Write},
    time::Instant,
};

fn criterion_benchmark(c: &mut Criterion) {
    let prep = signatures_bench::prepare();

    let vc = signatures_bench::zkp_issue_native(&prep.issuer, &prep.device_binding);
    let setup = signatures_bench::prepare_circuit();
    println!("{}", setup.len());

    let start = Instant::now();
    let presentation = signatures_bench::zkp_present_native(
        vc.clone(),
        &prep.issuer.public_key,
        &prep.proving_keys,
        prep.db_public_key,
        prep.message.clone(),
        prep.message_signature,
        Some(setup.clone()),
    );
    let presentation_json = BASE64_URL_SAFE_NO_PAD.decode(&presentation).unwrap();
    let presentation_obj: serde_json::Value = serde_json::from_slice(&presentation_json).unwrap();
    let mut buffer = BufWriter::new(Vec::new());
    let proof1 = BASE64_URL_SAFE_NO_PAD
        .decode(presentation_obj["proof"].as_str().unwrap())
        .unwrap();
    buffer.write_u64::<BigEndian>(proof1.len() as u64).unwrap();
    buffer.write_all(&proof1).unwrap();

    let proof2 = BASE64_URL_SAFE_NO_PAD
        .decode(presentation_obj["device_binding"].as_str().unwrap())
        .unwrap();
    buffer.write_u64::<BigEndian>(proof2.len() as u64).unwrap();
    buffer.write_all(&proof2).unwrap();
    let presentation_array = buffer.into_inner().unwrap();
    let presentation_len = presentation_array.len();
    // json!({
    //     "proof": BASE64_URL_SAFE_NO_PAD.encode(vp.proof.dataset().to_string()),
    //     "device_binding": db
    // })
    let end = Instant::now();
    println!("{}", (end - start).as_millis());

    c.bench_function("verification", |b| {
        b.iter(|| {
            black_box(zkp_verify_native(
                presentation.clone(),
                &prep.issuer.public_key,
                &prep.verifying_keys,
                prep.message.clone(),
                setup.clone(),
            ))
        })
    });
}

fn criterion_benchmark_old(c: &mut Criterion) {
    let prep = prepare();

    let vc = zkp_issue(&prep.issuer, &prep.device_binding);
    let start = Instant::now();
    let presentation = zkp_present(
        vc,
        &prep.issuer.public_key,
        &prep.proving_keys,
        prep.db_public_key,
        prep.message.clone(),
        prep.message_signature,
    );
    let end = Instant::now();
    println!("{}", (end - start).as_millis());

    c.bench_function("verification", |b| {
        b.iter(|| {
            black_box(zkp_verify(
                presentation.clone(),
                &prep.issuer.public_key,
                &prep.verifying_keys,
                prep.message.clone(),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_old);
criterion_main!(benches);
