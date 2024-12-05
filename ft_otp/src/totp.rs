use std::time::{
  SystemTime,
  Duration,
  SystemTimeError
};
use openssl::{
  error::ErrorStack,
  hash::MessageDigest,
  pkey::{
    PKey,
    Private,
  },
  sign::Signer
};

fn generate_signer(key: &PKey<Private>, digest_type: MessageDigest)
  -> Result<Signer<'_>, ErrorStack> {
  let signer: Result<Signer<'_>, ErrorStack> = Signer::new(digest_type, key);

  signer
}

/*
 * Apply T math from RFC 6238
 */
fn math_time(time: &Duration) -> u64 {
  return time.as_secs() / 30;
}

 fn get_math_time() -> Result<Duration, SystemTimeError> {
  let res_time: Result<Duration, SystemTimeError> = SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH);

  res_time
}

/*
 * Generate TOTP accordance to RFC 4226
 */
fn generate_totp(res_hmac: &Result<Vec<u8>, ErrorStack>) -> bool {
  return match res_hmac {
    Ok(hmac) => {
      if hmac.len() != 20 {
        eprintln!("HMAC-SHA-1 should be of size 20, currently: {0}",
                  hmac.len());
        return false;
      }
      let offset =  (hmac[19] & 0x0f) as usize;
      let mut pick_hmac: [u8; 4] = [0u8; 4];

      pick_hmac[0] = hmac[offset] & 0xff;
      pick_hmac[1] = hmac[offset + 1] & 0xff;
      pick_hmac[2] = hmac[offset + 2] & 0xff;
      pick_hmac[3] = hmac[offset + 3] & 0xff;
      let val = u32::from_be_bytes(pick_hmac) & 0x7FFFFFFF;
      let binary:u32 = val % 1000000;
      let len = u32::ilog10(binary) + 1;

      for _ in len..6 {
        print!("0");
      }
      println!("{0}", binary);
      true
    },
    Err(e) => {
      eprintln!("Error: {e}");
      false
    },
  };
}

/*
 * Sign pkey
 * Add T time to signature
 * Call TOTP generation
 */
fn sign_hmac_sha1(key:&PKey<Private>, t: u64) -> bool {
  let res_signer = generate_signer(&key, MessageDigest::sha1());

  return match res_signer {
      Ok(mut sign) => {
        let res_size = sign.update(&t.to_be_bytes());

        if let Err(e) = res_size {
          eprintln!("Error: {e}");
          return false;
        }
        let res_hmac: Result<Vec<u8>, ErrorStack>
                    = sign.sign_to_vec();
        let ret: bool = generate_totp(&res_hmac);

        ret
      },
      Err(e) => {
        eprintln!("Error: {e}");
        false
      },
  }
}

/*
 * Generate Time
 * Generate hmac and call sign sha1
 */
pub fn start_totp(buf: &str) -> bool {
  let res_buf: Result<Vec<u8>, hex::FromHexError> = hex::decode(&buf);
  let get_math: Result<Duration, SystemTimeError> = get_math_time();
  let mut t = 0;
  let res = match get_math {
      Ok(math) => {
        t = math_time(&math);
        true
      },
      Err(e) => {
        eprintln!("Error: {e}");
        false
      }
  };

  if !res {
    return res;
  }
  return match res_buf {
      Ok(buf) => {
        let res_key: Result<PKey<Private>, ErrorStack> = PKey::hmac(&buf);

        return match res_key {
            Ok(key) => {
              return sign_hmac_sha1(&key, t);
            },
            Err(e) => {
              eprintln!("Error: {e}");
              false
          }
        };
      },
      Err(_) => {
        eprintln!("Key length is odd");
        false
      },
  }
}