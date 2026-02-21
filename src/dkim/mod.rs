mod dkim_error;

use crate::config::dkim_config::DkimConfig;
use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD as B64};
use rsa::pkcs1v15::SigningKey;
use sha2::{Digest, Sha256};
use std::str::from_utf8;

pub struct DkimSigner {
    domain: String,
    selector: String,
    algorithm: Algorithm,
    private_key_pem: Vec<u8>,
    headers_to_sign: Vec<String>,
}

enum Algorithm {
    RsaSha256,
    Ed25519Sha256,
}

impl DkimSigner {
    pub fn from_config(config: &DkimConfig) -> Result<Self> {
        let pem = std::fs::read(&config.private_key_path)?;

        let algorithm = match config.alogrithm.as_str() {
            "rsa-sha256" => Algorithm::RsaSha256,
            "ed25519-sha256" => Algorithm::Ed25519Sha256,
            other => anyhow::bail!("Algoritmo DKIM desconhecido: {}", other),
        };

        Ok(Self {
            domain: config.domain.clone(),
            selector: config.selector.clone(),
            algorithm,
            private_key_pem: pem,
            headers_to_sign: config.headers.clone(),
        })
    }

    pub fn sign(&self, raw_message: &str) -> Result<String> {
        let (headers_str, body) = split_message(raw_message);

        let body_hash = {
            let canonical_body = relaxed_body(body);
            let mut h = Sha256::new();
            h.update(canonical_body.as_bytes());
            B64.encode(h.finalize())
        };

        // Monta hedaer
        let dkim_header_partial = format!(
            "DKIM-Signature: v=1; a={}; c=relaxed/relaxed; d={}; s={};\r\n\th={};\r\n\tbh={};\r\r\ttb=",
            self.algorithm_string(),
            self.domain,
            self.selector,
            self.headers_to_sign.join(":"),
            body_hash
        );

        let signing_heders = collect_headers(&headers_str, &self.headers_to_sign);
        let data_to_sign = format!("{}{}", signing_heders, dkim_header_partial);

        let signature = self.compute_signature(data_to_sign.as_bytes())?;

        Ok(format!("{}{}", dkim_header_partial, signature))
    }

    fn algorithm_string(&self) -> &str {
        match self.algorithm {
            Algorithm::RsaSha256 => "rsa-sha256",
            Algorithm::Ed25519Sha256 => "ed25519-sha256",
        }
    }

    fn compute_signature(&self, data: &[u8]) -> Result<String> {
        match self.algorithm {
            Algorithm::RsaSha256 => {
                use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, signature::SignatureEncoding};
                let key = RsaPrivateKey::from_pkcs8_pem(from_utf8(&self.private_key_pem)?)?;

                let signing_key = SigningKey::<Sha256>::new(key);
                use rsa::signature::RandomizedSigner;
                let mut rng = rand::thread_rng();
                let sig = signing_key.sign_with_rng(&mut rng, data);
                Ok(B64.encode(sig.to_bytes()))
            }
            Algorithm::Ed25519Sha256 => {
                use ed25519_dalek::{Signer, SigningKey};

                let pem_str = from_utf8(&self.private_key_pem)?;
                let b64_body: String = pem_str.lines().filter(|l| !l.starts_with("----")).collect();

                let key_bytes = B64.decode(b64_body.trim())?;
                let signing_key = SigningKey::try_from(key_bytes.as_slice())?;
                let signature = signing_key.sign(data);
                Ok(B64.encode(signature.to_bytes()))
            }
        }
    }
}

fn split_message(raw: &str) -> (&str, &str) {
    if let Some(pos) = raw.find("\r\n\r\n") {
        (&raw[..pos], &raw[pos + 4..])
    } else if let Some(pos) = raw.find("\n\n") {
        (&raw[..pos], &raw[pos + 2..])
    } else {
        (raw, "")
    }
}

fn relaxed_body(body: &str) -> String {
    let mut lines: Vec<String> = body
        .lines()
        .map(|l| {
            let collapsed: String = l.split_whitespace().collect::<Vec<_>>().join(" ");
            collapsed
        })
        .collect();

    while lines.last().map(|l: &String| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\r\n") + "\r\n"
}

fn collect_headers(headers_section: &str, to_sign: &[String]) -> String {
    let mut result = String::new();
    for h in to_sign {
        let lower = h.to_lowercase();
        for line in headers_section.lines() {
            if line.to_lowercase().starts_with(&format!("{}:", lower)) {
                result.push_str(line);
                result.push_str("\r\n");
                break;
            }
        }
    }

    result
}
