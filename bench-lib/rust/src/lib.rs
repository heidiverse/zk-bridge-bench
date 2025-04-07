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
use zkp_util::{device_binding::SecpFr, EcdsaSignature, SECP_GEN};

lazy_static! {
    static ref REQUIREMENTS: Vec<ProofRequirement> = vec![
        ProofRequirement::Required {
            key: "https://schema.org/name".into(),
        },
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
}

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
        .thread_stack_size(8388608)
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
    ))
    .unwrap()
}

#[derive(uniffi::Record)]
pub struct Preparation {
    issuer: Keypair,

    proving_keys: HashMap<String, String>,
    verifying_keys: HashMap<String, String>,

    message: Vec<u8>,
    message_signature: Vec<u8>,

    db_public_key: Vec<u8>,
    device_binding: DeviceBinding,
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
pub fn zkp_present(
    vc: String,
    issuer_pk: &String,
    proving_keys: &HashMap<String, String>,
    public_key: Vec<u8>,
    message: Vec<u8>,
    message_signature: Vec<u8>,
) -> String {
    let device_binding = DBRequirement {
        public_key,
        message,
        message_signature,
        comm_key_secp_label: b"secp".to_vec(),
        comm_key_tom_label: b"tom".to_vec(),
        comm_key_bls_label: b"bls".to_vec(),
        bpp_setup_label: b"bpp".to_vec(),
        merlin_transcript_label: b"transcript",
        challenge_label: b"challenge",
    };

    zkp::present(
        &mut OsRng,
        vc,
        &REQUIREMENTS,
        Some(device_binding),
        proving_keys,
        issuer_pk,
        "did:example:issuer0",
        "did:example:issuer0#key001",
    )
    .unwrap()
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
        merlin_transcript_label: b"transcript",
        challenge_label: b"challenge",
    };

    zkp::verify(
        &mut OsRng,
        presentation,
        &REQUIREMENTS,
        Some(device_binding),
        verifying_keys,
        issuer_pk,
        "did:example:issuer0",
        "did:example:issuer0#key001",
    )
    .unwrap()
    .to_string()
}

uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_roundtrip() {
        let prep = super::prepare();

        let vc = super::zkp_issue(&prep.issuer, &prep.device_binding);

        let presentation = super::zkp_present(
            vc,
            &prep.issuer.public_key,
            &prep.proving_keys,
            prep.db_public_key,
            prep.message.clone(),
            prep.message_signature,
        );

        let result = super::zkp_verify(
            presentation,
            &prep.issuer.public_key,
            &prep.verifying_keys,
            prep.message,
        );

        println!("{result}")
    }
}
