use std::{
    path::{Component, PathBuf},
    str::FromStr,
};

use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::{
    application::member::{ArtifactVerifier, EntitlementVerifier, MemberServiceError},
    domain::member::ModuleId,
};

pub struct Ed25519TokenVerifier {
    verifying_key: VerifyingKey,
}

impl Ed25519TokenVerifier {
    pub fn new(verifying_key: [u8; 32]) -> Result<Self, MemberServiceError> {
        Ok(Self {
            verifying_key: VerifyingKey::from_bytes(&verifying_key)
                .map_err(|_| MemberServiceError::invalid("MEMBER_ENTITLEMENT_KEY_INVALID"))?,
        })
    }
}

impl EntitlementVerifier for Ed25519TokenVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), MemberServiceError> {
        let signature = Signature::from_slice(signature)
            .map_err(|_| MemberServiceError::invalid("MEMBER_ENTITLEMENT_SIGNATURE_INVALID"))?;
        self.verifying_key
            .verify_strict(payload, &signature)
            .map_err(|_| MemberServiceError::invalid("MEMBER_ENTITLEMENT_SIGNATURE_INVALID"))
    }
}

pub struct LocalArtifactVerifier {
    modules_root: PathBuf,
}

impl LocalArtifactVerifier {
    pub fn new(modules_root: PathBuf) -> Self {
        Self { modules_root }
    }
}

impl ArtifactVerifier for LocalArtifactVerifier {
    fn verify_sha256(&self, bytes: &[u8], expected_hex: &str) -> Result<(), MemberServiceError> {
        if expected_hex.len() != 64 || !expected_hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(MemberServiceError::invalid("MEMBER_MODULE_HASH_INVALID"));
        }

        let actual = format!("{:x}", Sha256::digest(bytes));
        if actual.eq_ignore_ascii_case(expected_hex) {
            Ok(())
        } else {
            Err(MemberServiceError::invalid("MEMBER_MODULE_HASH_MISMATCH"))
        }
    }

    fn module_path(
        &self,
        module_id: &ModuleId,
        version: &str,
    ) -> Result<PathBuf, MemberServiceError> {
        if !is_safe_segment(module_id.as_str()) || !is_safe_segment(version) {
            return Err(MemberServiceError::invalid("MEMBER_MODULE_PATH_INVALID"));
        }
        Ok(self.modules_root.join(module_id.as_str()).join(version))
    }

    fn archive_entry_path(&self, entry: &str) -> Result<PathBuf, MemberServiceError> {
        let entry = PathBuf::from_str(entry)
            .map_err(|_| MemberServiceError::invalid("MEMBER_MODULE_ENTRY_INVALID"))?;
        if entry.as_os_str().is_empty()
            || entry.components().any(|component| {
                matches!(
                    component,
                    Component::Prefix(_) | Component::RootDir | Component::ParentDir
                )
            })
        {
            return Err(MemberServiceError::invalid("MEMBER_MODULE_ENTRY_INVALID"));
        }
        Ok(self.modules_root.join(entry))
    }
}

fn is_safe_segment(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}
