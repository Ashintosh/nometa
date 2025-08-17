
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, SaltString}, Argon2, PasswordHasher, PasswordVerifier};

pub(crate) fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();

    Ok(password_hash)
}

pub(crate) fn verify_password(password: &str, hash: &str) -> Result<(), Option<String>> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|e| e.to_string())?;
    
    if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
        return Ok(());
    }

    Err(None)
}