use jwt_simple::{
    claims::Claims,
    common::VerificationOptions,
    prelude::{
        Duration, Ed25519KeyPair, Ed25519PublicKey, EdDSAKeyPairLike, EdDSAPublicKeyLike, HashSet,
    },
};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppError;

const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";
pub struct TokenSignVerify {
    key: Ed25519KeyPair,
    pkey: Ed25519PublicKey,
    duration: Duration,
}

impl TokenSignVerify {
    pub fn try_new(pkey: &str, key: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            key: Ed25519KeyPair::from_pem(key)?,
            pkey: Ed25519PublicKey::from_pem(pkey)?,
            duration: Duration::from_secs(60 * 60 * 24 * 7),
        })
    }
    pub fn sign<T: Serialize + DeserializeOwned>(&self, data: T) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(data, self.duration)
            .with_issuer(JWT_ISS)
            .with_audience(JWT_AUD);
        Ok(self.key.sign(claims)?)
    }

    pub fn verify<T: Serialize + DeserializeOwned>(&self, token: &str) -> Result<T, AppError> {
        let opts = VerificationOptions {
            allowed_issuers: Some([JWT_ISS].iter().map(|&v| v.to_owned()).collect()),
            allowed_audiences: Some(HashSet::from([JWT_AUD.to_owned()])),
            ..Default::default()
        };
        let claims = self.pkey.verify_token(token, Some(opts))?;
        Ok(claims.custom)
    }
}
