fn main() {
    println!("Run the tests with `cargo test`");
}

pub mod example3 {
    include!(concat!(env!("OUT_DIR"), "/example3.rs"));
}

pub mod example2 {
    include!(concat!(env!("OUT_DIR"), "/example2.rs"));
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, io::Read, vec};

    use super::*;
    use bytes::Bytes;
    use prost::Message;

    #[test]
    fn empty_bytes() {
        let hs2 = example2::NoiseHandshakePayload {
            identity_key: Some(vec![]),
            identity_sig: Some(vec![]),
            extensions: None,
        };
        let b2 = hs2.encode_to_vec();

        let hs3 = example3::NoiseHandshakePayload {
            identity_key: vec![],
            identity_sig: vec![],
            extensions: None,
        };
        let b3 = hs3.encode_to_vec();

        // These are different on the wire
        assert_ne!(b2, b3);
        assert_eq!(b2, vec![10, 0, 18, 0]);
        assert_eq!(b3, vec![]);

        // But we should be able to interpret them regardless of proto2 vs proto3
        let b2to3 = example3::NoiseHandshakePayload::decode(bytes::Bytes::from(b2))
            .expect("should decode bytes from 2");
        assert_eq!(b2to3, hs3);

        let b3to2 = example2::NoiseHandshakePayload::decode(bytes::Bytes::from(b3))
            .expect("should decode bytes from 3");

        // proto2 code will think this was unset since it was the default val (empty byte vec)
        assert_eq!(b3to2, example2::NoiseHandshakePayload::default());
    }

    #[test]
    fn none_vals() {
        let hs2 = example2::NoiseHandshakePayload {
            identity_key: None,
            identity_sig: None,
            extensions: None,
        };
        let b2 = hs2.encode_to_vec();

        let hs3 = example3::NoiseHandshakePayload {
            identity_key: vec![],
            identity_sig: vec![],
            extensions: None,
        };
        let b3 = hs3.encode_to_vec();

        // These are the same on the wire
        assert_eq!(b2, b3);

        // But we should be able to interpret them regardless of proto2 vs proto3
        let b2to3 = example3::NoiseHandshakePayload::decode(bytes::Bytes::from(b2))
            .expect("should decode bytes from 2");
        assert_eq!(b2to3, hs3);

        let b3to2 = example2::NoiseHandshakePayload::decode(bytes::Bytes::from(b3))
            .expect("should decode bytes from 3");

        assert_eq!(b3to2, hs2);
    }

    #[test]
    fn some_vals() {
        let hs2 = example2::NoiseHandshakePayload {
            identity_key: Some(vec![1, 2, 3]),
            identity_sig: Some(vec![4, 5, 6]),
            extensions: Some(example2::NoiseExtensions {
                webtransport_certhashes: vec![vec![0, 1, 2]],
            }),
        };
        let b2 = hs2.encode_to_vec();

        let hs3 = example3::NoiseHandshakePayload {
            identity_key: vec![1, 2, 3],
            identity_sig: vec![4, 5, 6],
            extensions: Some(example3::NoiseExtensions {
                webtransport_certhashes: vec![vec![0, 1, 2]],
            }),
        };
        let b3 = hs3.encode_to_vec();

        // These are the same on the wire
        assert_eq!(b2, b3);

        // But we should be able to interpret them regardless of proto2 vs proto3
        let b2to3 = example3::NoiseHandshakePayload::decode(bytes::Bytes::from(b2))
            .expect("should decode bytes from 2");
        assert_eq!(b2to3, hs3);

        let b3to2 = example2::NoiseHandshakePayload::decode(bytes::Bytes::from(b3))
            .expect("should decode bytes from 3");

        assert_eq!(b3to2, hs2);
    }
}
