#[cfg(test)]
mod tests {
    extern crate two_party_eddsa_server;
    extern crate two_party_eddsa_client;

    use two_party_eddsa_client::api::*;
    use std::{thread, time};

    #[test]
    fn test_api() {
        // Rocket server is blocking, so we spawn a new thread.
        thread::spawn(move || {
            two_party_eddsa_server::api::get_server().launch();
        });

        let client_shim = ClientShim::new("http://localhost:8000".to_string());

        let five_seconds = time::Duration::from_millis(5000);
        thread::sleep(five_seconds);

        let (key_pair, key_agg, id) = two_party_eddsa_client::api::generate_key(&client_shim).unwrap();

        let message = BigInt::from(1234);
        let signature = two_party_eddsa_client::api::sign(&client_shim, message, &key_pair, &key_agg, &id)
            .expect("Error while signing");
        println!(
            "signature = (R: {}, s: {})",
            signature.R.bytes_compressed_to_big_int().to_hex(),
            signature.s.to_big_int().to_hex()
        );
    }
}
