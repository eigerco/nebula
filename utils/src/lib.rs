use soroban_sdk::{Address, BytesN, TryFromVal, Val};

pub fn address_to_bytes_n32(addr: &Address) -> BytesN<32> {
    let env = addr.env();
    let payload: u64 = addr.as_val().get_payload();
    let mut from_buffer = [0; 32];
    from_buffer[..8].copy_from_slice(&payload.to_be_bytes());
    BytesN::from_array(env, &from_buffer)
}

pub fn bytes_n32_to_address(bytes: &BytesN<32>) -> Address {
    let mut to_buffer = [0; 8];
    to_buffer.copy_from_slice(&bytes.to_array()[..8]);
    let val = Val::from_payload(u64::from_be_bytes(to_buffer));
    Address::try_from_val(bytes.env(), &val).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_encoding_decoding_address_to_bytes32() {
        let env = Env::default();
        let address = Address::random(&env);
        let bytes = address_to_bytes_n32(&address);
        assert_eq!(address, bytes_n32_to_address(&bytes))
    }
}
