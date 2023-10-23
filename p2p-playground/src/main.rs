use p256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret, NistP256};
use rand_core::OsRng;

struct MySecret {
    private: EphemeralSecret,
    // encode: EncodedPoint,
    public: PublicKey,
}

fn create_keys() -> MySecret {
    let eph_secret = EphemeralSecret::random(&mut OsRng);
    let epoint = EncodedPoint::from(eph_secret.public_key());
    let pubkey = PublicKey::from_sec1_bytes(epoint.as_ref()).expect("Invalid public key");

    MySecret { 
        private : eph_secret, 
        public  : pubkey, 
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sharedkeys() {
        let alice = create_keys();
        let bob = create_keys();

        let alice_shared = alice.private.diffie_hellman(&bob.public);
        let bob_shared = bob.private.diffie_hellman(&alice.public);

        let alice_shared_hex = hex::encode(alice_shared.raw_secret_bytes());
        let bob_shared_hex = hex::encode(bob_shared.raw_secret_bytes());

        println!("Shared keys: {:?}", alice_shared_hex);
        println!("Shared keys: {:?}", bob_shared_hex);
        assert_eq!(alice_shared_hex, bob_shared_hex);
    
    }
}