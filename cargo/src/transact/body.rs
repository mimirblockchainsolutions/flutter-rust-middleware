use mimir_crypto::{Keccak256, Signer, Signature};
use rlp::{Encodable, RlpStream};
use transact::{util, Signed};


/// body of an external ethereum transaction.
///
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Body<'a> {
    /// nonce of sender
    pub nonce: &'a [u8],

    /// gas price paid
    pub gas_price: &'a [u8],

    /// maximum gas afforded
    pub gas_limit: &'a [u8],

    /// destination address
    pub to: &'a [u8],

    /// amount sent (in wei).
    pub value: &'a [u8],

    /// transaction calldata
    pub data: &'a [u8],
}


impl<'a> Body<'a> {
    /// sign transaction with specified signer.
    ///
    pub fn to_signed<S>(self, signer: S) -> Signed<'a>
    where
        S: Signer<Msg = [u8; 32], Sig = Signature>,
    {
        let bytes = self.rlp();
        let hash = Keccak256::hash(&bytes);
        let signature = signer.sign(&hash);
        Signed::new(self, signature)
    }

    /// get transaction as rlp encoded bytes
    ///
    pub fn rlp(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        self.rlp_append(&mut stream);
        stream.out()
    }
}


impl<'a> Encodable for Body<'a> {
    fn rlp_append(&self, stream: &mut RlpStream) {
        let tokens: [&[u8]; 6] = [
            util::trim(&self.nonce),
            util::trim(&self.gas_price),
            util::trim(&self.gas_limit),
            util::trim(&self.to),
            util::trim(&self.value),
            &self.data,
        ];
        stream.begin_list(6);
        for token in tokens.iter() {
            stream.append(token);
        }
    }
}
