use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
struct Seed {
    pub seed: u64,
}

#[derive(Deserialize, Debug)]
struct Keys {
    key1: Option<String>,
    key2: Option<String>,
    key3: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Notes {
    #[serde(rename = "1")]
    pub notes1: String,
    #[serde(rename = "2")]
    pub notes2: String,
    #[serde(rename = "3")]
    pub notes3: String,
}

fn get_seed() -> Result<u64, DownloadError> {
    let url = "https://everybody.codes/api/user/me";
    let cookie = std::env::var("EVERYBODY_CODES")?;

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header("Cookie", format!("everybody-codes={}", cookie))
        .send();
    let seed = resp?.json::<Seed>()?;
    Ok(seed.seed)
}

fn get_aes_keys(event_or_story: usize, quest: usize) -> Result<Keys, DownloadError> {
    let cookie = std::env::var("EVERYBODY_CODES")?;
    let url = format!("https://everybody.codes/api/event/{event_or_story}/quest/{quest}");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header("Cookie", format!("everybody-codes={}", cookie))
        .send();
    Ok(resp?.json::<Keys>()?)
}

fn get_notes(seed: u64, event_or_story: usize, quest: usize) -> Result<Notes, DownloadError> {
    let url = format!(
        "https://everybody-codes.b-cdn.net/assets/{event_or_story}/{quest}/input/{seed}.json"
    );
    let client = reqwest::blocking::get(&url);
    Ok(client?.json::<Notes>()?)
}

fn decrypt(key: &str, encrypted_text: &str) -> Result<String, DownloadError> {
    let mut encrypted_bytes = hex::decode(encrypted_text)?;
    let key_bytes = key.as_bytes();
    let iv_bytes = &key[..16].as_bytes();

    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    let cipher = Aes256CbcDec::new_from_slices(key_bytes, iv_bytes)
        .map_err(|_| DownloadError::InvalidLength)?;

    let decrypted_bytes = cipher
        .decrypt_padded_mut::<Pkcs7>(encrypted_bytes.as_mut_slice())
        .map_err(|_| DownloadError::PaddingError)?;

    let decrypted = String::from_utf8(decrypted_bytes.to_vec())?;

    Ok(decrypted)
}

/// TODO: Refactor
pub fn download_notes(
    event_or_story: usize,
    quest: usize,
    part: usize,
) -> Result<String, DownloadError> {
    let cache_path = format!("input/{event_or_story}/{quest}/{part}.txt");
    if let Ok(content) = std::fs::read_to_string(&cache_path) {
        return Ok(content);
    }

    let seed = get_seed()?;
    let keys = get_aes_keys(event_or_story, quest)?;
    let notes = get_notes(seed, event_or_story, quest)?;
    for (part, (key, note)) in [&keys.key1, &keys.key2, &keys.key3]
        .iter()
        .zip([&notes.notes1, &notes.notes2, &notes.notes3].iter())
        .enumerate()
    {
        let cache_path = format!("input/{}/{}/{}.txt", event_or_story, quest, part + 1);
        if std::fs::metadata(&cache_path).is_ok() {
            continue;
        }
        if let Some(k) = key {
            let decrypted = decrypt(k, note);
            if decrypted.is_ok() {
                std::fs::create_dir_all(format!("input/{}/{}", event_or_story, quest)).ok();
                std::fs::write(&cache_path, &decrypted.as_ref().unwrap()).ok();
            }
        }
    }
    match part {
        1 if keys.key1.is_some() => decrypt(&keys.key1.as_ref().unwrap(), &notes.notes1),
        2 if keys.key2.is_some() => decrypt(&keys.key2.as_ref().unwrap(), &notes.notes2),
        3 if keys.key3.is_some() => decrypt(&keys.key3.as_ref().unwrap(), &notes.notes3),
        _ => {
            return Err(DownloadError::NotesNotAvailable);
        }
    }
}

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid key or IV length")]
    InvalidLength,
    #[error("Padding error")]
    PaddingError,
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Hex decode error: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Notes not available")]
    NotesNotAvailable,
}
