use mimir_crypto::{Signer, Signature};
use transact::Body;


/// raw transaction builder.
///
/// *note*: all unset values default to their empty/zero case.
///
#[derive(Default, Debug, Clone)]
pub struct RawTxBuilder<'a, S> {
    /// signer for this transaction.
    pub signer: S,

    /// nonce of sender
    pub nonce: Option<&'a [u8]>,

    /// gas price paid
    pub gas_price: Option<&'a [u8]>,

    /// maximum gas afforded
    pub gas_limit: Option<&'a [u8]>,

    /// destination address
    pub to: Option<&'a [u8]>,

    /// amount sent (in wei).
    pub value: Option<&'a [u8]>,

    /// transaction calldata
    pub data: Option<&'a [u8]>,
}


impl<'a, S> RawTxBuilder<'a, S> {
    /// get a new builder instance.
    ///
    pub fn new(signer: S) -> Self {
        RawTxBuilder {
            signer: signer,
            nonce: None,
            gas_price: None,
            gas_limit: None,
            to: None,
            value: None,
            data: None,
        }
    }

    /// set the value of the transaction nonce.
    ///
    pub fn nonce<T: ?Sized>(&mut self, nonce: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.nonce = Some(nonce.as_ref());
        self
    }

    /// set the value of the transaction gas price.
    ///
    pub fn gas_price<T: ?Sized>(&mut self, price: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.gas_price = Some(price.as_ref());
        self
    }

    /// set the value of the transaction gas limit.
    ///
    pub fn gas_limit<T: ?Sized>(&mut self, limit: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.gas_limit = Some(limit.as_ref());
        self
    }

    /// set the transaction destination address.
    ///
    pub fn to<T: ?Sized>(&mut self, to: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.to = Some(to.as_ref());
        self
    }

    /// set the value (in wei) to be sent with this transaction.
    ///
    pub fn value<T: ?Sized>(&mut self, value: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.value = Some(value.as_ref());
        self
    }

    /// set the transaction calldata.
    ///
    pub fn data<T: ?Sized>(&mut self, data: &'a T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.data = Some(data.as_ref());
        self
    }
}


impl<'a, S> RawTxBuilder<'a, S>
where
    S: Signer<Msg = [u8; 32], Sig = Signature>,
{
    /// finish building transaction, returning bytes of signed raw tx.
    ///
    pub fn finish(&self) -> Vec<u8> {
        let nonce = self.nonce.unwrap_or(&[]);
        let gas_price = self.gas_price.unwrap_or(&[]);
        let gas_limit = self.gas_limit.unwrap_or(&[]);
        let to = self.to.unwrap_or(&[]);
        let value = self.value.unwrap_or(&[]);
        let data = self.data.unwrap_or(&[]);
        let body = Body {
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
        };
        let signed = body.to_signed(&self.signer);
        signed.rlp()
    }
}
