use super::crypto::NovaCrypto;
use crate::error::Result;
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

pub struct HandshakeState {
    pub state: HandshakePhase,
    pub local_static: StaticSecret,
    pub local_public: PublicKey,
    pub remote_public: Option<PublicKey>,
    pub ephemeral_public: Option<PublicKey>,
    pub shared_secret: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakePhase {
    Init,
    SentEphemeral,
    ReceivedEphemeral,
    DerivedKeys,
    Complete,
}

impl HandshakeState {
    pub fn new() -> Self {
        let local_static = StaticSecret::random_from_rng(OsRng);
        let local_public = PublicKey::from(&local_static);

        Self {
            state: HandshakePhase::Init,
            local_static,
            local_public,
            remote_public: None,
            ephemeral_public: None,
            shared_secret: None,
        }
    }

    pub fn initiate(&mut self) -> Vec<u8> {
        let ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let ephemeral_public = PublicKey::from(&ephemeral);

        self.ephemeral_public = Some(ephemeral_public);
        self.state = HandshakePhase::SentEphemeral;

        let mut payload = Vec::new();
        payload.extend_from_slice(self.local_public.as_bytes());
        payload.extend_from_slice(ephemeral_public.as_bytes());

        payload
    }

    pub fn respond(
        &mut self,
        initiator_public: &[u8],
        initiator_ephemeral: &[u8],
    ) -> Result<Vec<u8>> {
        if initiator_public.len() != 32 || initiator_ephemeral.len() != 32 {
            return Err(crate::error::Error::Protocol(
                "Invalid public key length".to_string(),
            ));
        }

        let mut remote_public_bytes = [0u8; 32];
        remote_public_bytes.copy_from_slice(initiator_public);
        let remote_public = PublicKey::from(remote_public_bytes);

        let mut remote_ephemeral_bytes = [0u8; 32];
        remote_ephemeral_bytes.copy_from_slice(initiator_ephemeral);
        let remote_ephemeral = PublicKey::from(remote_ephemeral_bytes);

        self.remote_public = Some(remote_public);

        let ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let ephemeral_public = PublicKey::from(&ephemeral);

        let shared_secret = ephemeral.diffie_hellman(&remote_ephemeral);

        self.ephemeral_public = Some(ephemeral_public);
        self.shared_secret = Some(shared_secret.as_bytes().clone());
        self.state = HandshakePhase::DerivedKeys;

        let mut payload = Vec::new();
        payload.extend_from_slice(self.local_public.as_bytes());
        payload.extend_from_slice(ephemeral_public.as_bytes());

        Ok(payload)
    }

    pub fn complete(
        &mut self,
        responder_public: &[u8],
        responder_ephemeral: &[u8],
    ) -> Result<()> {
        if responder_public.len() != 32 || responder_ephemeral.len() != 32 {
            return Err(crate::error::Error::Protocol(
                "Invalid public key length".to_string(),
            ));
        }

        let mut remote_public_bytes = [0u8; 32];
        remote_public_bytes.copy_from_slice(responder_public);
        let remote_public = PublicKey::from(remote_public_bytes);

        let mut remote_ephemeral_bytes = [0u8; 32];
        remote_ephemeral_bytes.copy_from_slice(responder_ephemeral);
        let remote_ephemeral = PublicKey::from(remote_ephemeral_bytes);

        self.remote_public = Some(remote_public);

        let ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let shared_secret = ephemeral.diffie_hellman(&remote_ephemeral);

        self.shared_secret = Some(shared_secret.as_bytes().clone());
        self.state = HandshakePhase::Complete;

        Ok(())
    }

    pub fn get_shared_secret(&self) -> Result<[u8; 32]> {
        self.shared_secret.ok_or(crate::error::Error::Protocol(
            "Handshake not complete".to_string(),
        ))
    }
}

pub struct Handshake {
    state: HandshakeState,
}

impl Handshake {
    pub fn new() -> Self {
        Self {
            state: HandshakeState::new(),
        }
    }

    pub fn initiate(&mut self) -> Vec<u8> {
        self.state.initiate()
    }

    pub fn handle_initiate(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() != 64 {
            return Err(crate::error::Error::Protocol(
                "Invalid handshake data".to_string(),
            ));
        }

        self.state.respond(&data[..32], &data[32..])
    }

    pub fn handle_response(&mut self, data: &[u8]) -> Result<()> {
        if data.len() != 64 {
            return Err(crate::error::Error::Protocol(
                "Invalid handshake data".to_string(),
            ));
        }

        self.state.complete(&data[..32], &data[32..])
    }

    pub fn is_complete(&self) -> bool {
        self.state.state == HandshakePhase::Complete
    }

    pub fn get_crypto(&self) -> Result<NovaCrypto> {
        let shared_secret = self.state.get_shared_secret()?;
        Ok(NovaCrypto::new(shared_secret))
    }
}
