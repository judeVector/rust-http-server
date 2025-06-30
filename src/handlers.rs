use axum::{Json};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use solana_sdk::{
    pubkey::Pubkey,
};
use crate::types::*;    
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bs58;
use serde_json::json;
use std::str::FromStr;

pub async fn generate_keypair() -> Json<SuccessResponse<serde_json::Value>> {
    let keypair = Keypair::generate(&mut OsRng);
    let pubkey = bs58::encode(keypair.public).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    Json(SuccessResponse {
        success: true,
        data: json!({
            "pubkey": pubkey,
            "secret": secret
        }),
    })
}

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> Json<serde_json::Value> {
    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({"success": false, "error": "Invalid secret key"})),
    };
    if secret_bytes.len() != 64 {
        return Json(json!({"success": false, "error": "Secret key must be 64 bytes"}));
    }

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return Json(json!({"success": false, "error": "Failed to parse keypair"})),
    };

    let sig = keypair.sign(payload.message.as_bytes());
    Json(json!({
        "success": true,
        "data": {
            "signature": BASE64.encode(sig.to_bytes()),
            "public_key": bs58::encode(keypair.public).into_string(),
            "message": payload.message
        }
    }))
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Json<serde_json::Value> {
    let pubkey_bytes = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({"success": false, "error": "Invalid public key"})),
    };

    let signature_bytes = match BASE64.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({"success": false, "error": "Invalid signature encoding"})),
    };

    let public_key = match ed25519_dalek::PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({"success": false, "error": "Invalid public key"})),
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => return Json(json!({"success": false, "error": "Invalid signature"})),
    };

    let valid = public_key.verify(payload.message.as_bytes(), &signature).is_ok();

    Json(json!({
        "success": true,
        "data": {
            "valid": valid,
            "message": payload.message,
            "pubkey": payload.pubkey
        }
    }))
}

pub async fn create_token(Json(_payload): Json<CreateTokenRequest>) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "accounts": [
                {"pubkey": "mint", "is_signer": false, "is_writable": true}
            ],
            "instruction_data": "base64-mint-init-data"
        }
    }))

}

pub async fn mint_token(Json(_payload): Json<MintTokenRequest>) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "accounts": [
                {"pubkey": "mint", "is_signer": false, "is_writable": true},
                {"pubkey": "destination", "is_signer": false, "is_writable": true}
            ],
            "instruction_data": "base64-mint-to-data"
        }
    }))
}

pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> Json<serde_json::Value> {
    let from_pubkey = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({"success": false, "error": "Invalid sender pubkey"})),
    };

    let to_pubkey = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({"success": false, "error": "Invalid recipient pubkey"})),
    };

    let instruction = solana_sdk::system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);

    Json(json!({
        "success": true,
        "data": {
            "program_id": instruction.program_id.to_string(),
            "accounts": instruction.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
            "instruction_data": BASE64.encode(instruction.data)
        }
    }))
}

pub async fn send_token(Json(_payload): Json<SendTokenRequest>) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "accounts": [
                { "pubkey": "source", "isSigner": true },
                { "pubkey": "destination", "isSigner": false }
            ],
            "instruction_data": "base64-transfer-data"
        }
    }))
}
