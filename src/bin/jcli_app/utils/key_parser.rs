use chain_crypto::bech32::{Bech32, Error};
use chain_crypto::{AsymmetricPublicKey, PublicKey};

pub fn parse_pub_key<A: AsymmetricPublicKey>(bech32_str: &str) -> Result<PublicKey<A>, Error> {
    Bech32::try_from_bech32_str(bech32_str)
}
