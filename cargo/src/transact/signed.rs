use mimir_crypto::Signature;
use transact::{Body, util};
use rlp::{Encodable, RlpStream};


/// signed external ethereum transaction.
///
pub struct Signed<'a> {
    pub body: Body<'a>,
    pub sig: Signature,
}


impl<'a> Signed<'a> {
    /// build new signed transaction from parts.
    ///
    pub fn new(body: Body<'a>, sig: Signature) -> Self {
        Signed { body, sig }
    }

    /// get as rlp encoded bytes
    ///
    pub fn rlp(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        self.rlp_append(&mut stream);
        stream.out()
    }
}


impl<'a> Encodable for Signed<'a> {
    fn rlp_append(&self, stream: &mut RlpStream) {
        let (v, r, s) = (self.sig.get_v(), self.sig.get_r(), self.sig.get_s());
        let tokens: [&[u8]; 9] = [
            util::trim(&self.body.nonce),
            util::trim(&self.body.gas_price),
            util::trim(&self.body.gas_limit),
            util::trim(&self.body.to),
            util::trim(&self.body.value),
            &self.body.data,
            &[v],
            util::trim(&r),
            util::trim(&s),
        ];
        stream.begin_list(9);
        for token in tokens.iter() {
            stream.append(token);
        }
    }
}
