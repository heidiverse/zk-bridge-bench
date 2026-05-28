use std::collections::HashMap;

use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use ark_std::UniformRand;
use lazy_static::lazy_static;
use next_gen_signatures::{
    crypto::zkp::{
        self, serialize_public_key_uncompressed, serialize_signature, DBRequirement,
        DBVerificationParams, ProofRequirement,
    },
    Engine, BASE64_STANDARD,
};
use rand_core::OsRng;
use serde_json::json;
use zkp_util::{
    device_binding::{limbs_from_public_key, SecpFr},
    ecdsa_pops::{bincode, PoPNativeNizk},
    vc::requirements::DiscloseRequirement,
    EcdsaSignature, SECP_GEN,
};

#[no_mangle]
pub fn pasta_to() {}

lazy_static! {
    static ref REQUIREMENTS: Vec<ProofRequirement> = vec![
        ProofRequirement::Required(DiscloseRequirement {
            key: "https://schema.org/name".into(),
        }),
        ProofRequirement::Circuit {
            id: "https://zkp-ld.org/circuit/ubique/lessThanPublic".to_string(),
            private_var: "a".into(),
            private_key: "https://schema.org/birthDate".into(),
            public_var: "b".into(),
            public_val: rdf_util::Value::Typed(
                "2001-01-01T00:00:00Z".into(),
                "http://www.w3.org/2001/XMLSchema#dateTime".into(),
            ),
        },
    ];
    static ref REQUIREMENTS_NATIVE: Vec<ProofRequirement> =
        vec![ProofRequirement::Required(DiscloseRequirement {
            key: "https://schema.org/name".into(),
        }),];
}

// #[cfg(target_arch = "x86_64")]
#[no_mangle]
pub fn __rust_probestack() {}
#[no_mangle]
pub fn ___rust_probestack() {}

const STACK_SIZE: usize = 8 * 1024 * 1024;

#[uniffi::export]
pub fn hello() -> String {
    "Hello, World".to_string()
}

#[derive(uniffi::Record)]
pub struct Keypair {
    pub public_key: String,
    pub secret_key: String,
}

#[uniffi::export]
pub fn gen_keypair() -> Keypair {
    let (public_key, secret_key) = zkp::generate_keypair(&mut OsRng);
    Keypair {
        public_key,
        secret_key,
    }
}

#[derive(uniffi::Record)]
pub struct DeviceBinding {
    pub x_value: String,
    pub y_value: String,
}

#[uniffi::export]
pub fn zkp_issue(issuer: &Keypair, device_binding: &DeviceBinding) -> String {
    let rt = tokio::runtime::Builder::new_current_thread()
        .thread_stack_size(STACK_SIZE)
        .build()
        .unwrap();

    rt.block_on(zkp::issue(
        &mut OsRng,
        json!({
            "https://schema.org/name": "John, Doe",
            "https://schema.org/birthDate": {
                "@value": "2000-01-01T00:00:00Z",
                "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            },
            "https://schema.org/dog": {
                "https://schema.org/name": "Ricky"
            }
        }),
        &issuer.public_key,
        &issuer.secret_key,
        "did:example:issuer0",
        "did:example:issuer0#key001",
        None,
        None,
        None,
        Some((
            device_binding.x_value.clone(),
            device_binding.y_value.clone(),
        )),
        None,
    ))
    .unwrap()
}
#[uniffi::export]
pub fn zkp_issue_native(issuer: &Keypair, device_binding: &DeviceBinding) -> String {
    let rt = tokio::runtime::Builder::new_current_thread()
        .thread_stack_size(STACK_SIZE)
        .build()
        .unwrap();
    let (x1, x2) = limbs_from_public_key(&device_binding.x_value);

    rt.block_on(zkp::issue(
        &mut OsRng,
        json!({
            "https://schema.org/name": "John, Doe",
            "https://schema.org/birthDate": {
                "@value": "2000-01-01T00:00:00Z",
                "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            }
        }),
        &issuer.public_key,
        &issuer.secret_key,
        "did:example:issuer0",
        "did:example:issuer0#key001",
        None,
        None,
        None,
        Some((x1, x2)),
        None,
    ))
    .unwrap()
}

#[derive(uniffi::Record)]
pub struct Preparation {
    pub issuer: Keypair,

    pub proving_keys: HashMap<String, String>,
    pub verifying_keys: HashMap<String, String>,

    pub message: Vec<u8>,
    pub message_signature: Vec<u8>,

    pub db_public_key: Vec<u8>,
    pub device_binding: DeviceBinding,
}

#[uniffi::export]
pub fn prepare() -> Preparation {
    let mut rng = OsRng;

    let circuits = zkp::generate_circuits(&mut rng, &REQUIREMENTS);

    let db_sk = SecpFr::rand(&mut rng);
    let db_pk = (SECP_GEN * db_sk).into_affine();

    let db_x = BASE64_STANDARD.encode(db_pk.x().unwrap().into_bigint().to_bytes_be());
    let db_y = BASE64_STANDARD.encode(db_pk.y().unwrap().into_bigint().to_bytes_be());

    let message = SecpFr::rand(&mut rng);
    let message_signature = EcdsaSignature::new_prehashed(&mut rng, message, db_sk);

    Preparation {
        issuer: gen_keypair(),

        proving_keys: circuits.proving_keys,
        verifying_keys: circuits.verifying_keys,

        message: message.into_bigint().to_bytes_be(),
        message_signature: serialize_signature(&message_signature),

        db_public_key: serialize_public_key_uncompressed(&db_pk),
        device_binding: DeviceBinding {
            x_value: db_x,
            y_value: db_y,
        },
    }
}

#[uniffi::export]
pub fn prepare_circuit() -> Vec<u8> {
    let c = PoPNativeNizk::new("pop");
    bincode::serialize(&c).unwrap()
}

#[uniffi::export]
pub fn zkp_present(
    vc: String,
    issuer_pk: &String,
    proving_keys: &HashMap<String, String>,
    public_key: Vec<u8>,
    message: Vec<u8>,
    message_signature: Vec<u8>,
) -> String {
    let issuer_pk = issuer_pk.clone();
    let proving_keys = proving_keys.clone();
    let handle = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let device_binding = DBRequirement {
                public_key,
                message,
                message_signature,
                comm_key_secp_label: b"secp".to_vec(),
                comm_key_tom_label: b"tom".to_vec(),
                comm_key_bls_label: b"bls".to_vec(),
                bpp_setup_label: b"bpp".to_vec(),
                // merlin_transcript_label: b"transcript",
                // challenge_label: b"challenge",
            };

            zkp::present(
                &mut OsRng,
                vc,
                &REQUIREMENTS_NATIVE,
                Some(device_binding),
                &proving_keys,
                &issuer_pk,
                "did:example:issuer0",
                "did:example:issuer0#key001",
            )
            .unwrap()
        })
        .unwrap();

    handle.join().unwrap()
}

#[uniffi::export]
pub fn zkp_verify(
    presentation: String,
    issuer_pk: &String,
    verifying_keys: &HashMap<String, String>,
    message: Vec<u8>,
) -> String {
    let device_binding = DBVerificationParams {
        message,
        comm_key_secp_label: b"secp".to_vec(),
        comm_key_tom_label: b"tom".to_vec(),
        comm_key_bls_label: b"bls".to_vec(),
        bpp_setup_label: b"bpp".to_vec(),
        // merlin_transcript_label: b"transcript",
        // challenge_label: b"challenge",
    };

    zkp::verify(
        &mut OsRng,
        presentation,
        &REQUIREMENTS_NATIVE,
        Some(device_binding),
        verifying_keys,
        issuer_pk,
        "did:example:issuer0",
        "did:example:issuer0#key001",
    )
    .unwrap()
    .to_string()
}

#[uniffi::export]
pub fn zkp_present_native(
    vc: String,
    issuer_pk: &String,
    proving_keys: &HashMap<String, String>,
    public_key: Vec<u8>,
    message: Vec<u8>,
    message_signature: Vec<u8>,
    setup: Option<Vec<u8>>,
) -> String {
    let issuer_pk = issuer_pk.clone();
    let proving_keys = proving_keys.clone();
    let handle = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let device_binding = DBRequirement {
                public_key,
                message,
                message_signature,
                comm_key_secp_label: b"secp".to_vec(),
                comm_key_tom_label: b"tom".to_vec(),
                comm_key_bls_label: b"bls".to_vec(),
                bpp_setup_label: b"bpp".to_vec(),
                // merlin_transcript_label: b"transcript",
                // challenge_label: b"challenge",
            };
            let setup = if let Some(setup) = setup {
                Some(bincode::deserialize(&setup).unwrap())
            } else {
                None
            };

            zkp::present_native(
                &mut OsRng,
                vc,
                &REQUIREMENTS_NATIVE,
                Some(device_binding),
                &proving_keys,
                &issuer_pk,
                "did:example:issuer0",
                "did:example:issuer0#key001",
                setup,
            )
            .unwrap()
        })
        .unwrap();

    handle.join().unwrap()
}

#[uniffi::export]
pub fn zkp_verify_native(
    presentation: String,
    issuer_pk: &String,
    verifying_keys: &HashMap<String, String>,
    message: Vec<u8>,
    setup: Vec<u8>,
) -> String {
    let device_binding = DBVerificationParams {
        message,
        comm_key_secp_label: b"secp".to_vec(),
        comm_key_tom_label: b"tom".to_vec(),
        comm_key_bls_label: b"bls".to_vec(),
        bpp_setup_label: b"bpp".to_vec(),
        // merlin_transcript_label: b"transcript",
        // challenge_label: b"challenge",
    };
    let setup: PoPNativeNizk = bincode::deserialize(&setup).unwrap();

    zkp::verify_native(
        &mut OsRng,
        presentation,
        &REQUIREMENTS_NATIVE,
        Some(device_binding),
        verifying_keys,
        issuer_pk,
        "did:example:issuer0",
        "did:example:issuer0#key001",
        setup,
    )
    .unwrap()
    .to_string()
}

uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use flate2::{bufread::DeflateEncoder, Compression};
    use next_gen_signatures::{Engine, BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
    use tokio::time::Instant;
    use zkp_util::ecdsa_pops::halo2curves::ff::derive::byteorder::{BigEndian, WriteBytesExt};

    #[test]
    pub fn test_roundtrip() {
        let prep = super::prepare();

        let vc = super::zkp_issue(&prep.issuer, &prep.device_binding);
        let start = Instant::now();
        let presentation = super::zkp_present(
            vc,
            &prep.issuer.public_key,
            &prep.proving_keys,
            prep.db_public_key,
            prep.message.clone(),
            prep.message_signature,
        );
        let end = Instant::now();
        println!("{}", (end - start).as_millis());

        let result = super::zkp_verify(
            presentation,
            &prep.issuer.public_key,
            &prep.verifying_keys,
            prep.message,
        );
    }
    #[test]
    pub fn test_roundtrip_native() {
        let prep = super::prepare();

        let vc = super::zkp_issue_native(&prep.issuer, &prep.device_binding);
        let setup = super::prepare_circuit();
        println!("{}", setup.len());

        let start = Instant::now();
        let presentation = super::zkp_present_native(
            vc.clone(),
            &prep.issuer.public_key,
            &prep.proving_keys,
            prep.db_public_key,
            prep.message.clone(),
            prep.message_signature,
            Some(setup.clone()),
        );
        let presentation_json = BASE64_URL_SAFE_NO_PAD.decode(&presentation).unwrap();
        let presentation_obj: serde_json::Value =
            serde_json::from_slice(&presentation_json).unwrap();
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

        let result = super::zkp_verify_native(
            presentation.clone(),
            &prep.issuer.public_key,
            &prep.verifying_keys,
            prep.message,
            setup,
        );
        use flate2::write::DeflateEncoder;
        use flate2::Compression;
        use std::io::prelude::*;

        // Vec<u8> implements Write to print the compressed bytes of sample string

        let mut e = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
        e.write_all(&presentation_array).unwrap();
        let compressed = e.finish().unwrap();
        println!();
        println!();
        println!("VC: {}kb", vc.len() / 1000);
        println!("Original: {}b", presentation_len);
        println!("Deflate: {}b", compressed.len());
        let jc = jabcode::write_jabcode(&compressed, &jabcode::WriteOptions::default()).unwrap();
        jc.save_with_format("./jab.png", image::ImageFormat::Png)
            .unwrap();
    }
}
