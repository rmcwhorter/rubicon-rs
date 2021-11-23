use std::hash::{Hash, Hasher};
use std::ops::BitXor;

/// a symmetric keypair that you can use as a key in a HashMap
/// The idea is that the pair (USDC, ETH) resolves to the same
/// thing as (ETH, USDC)
struct Pair<H: Clone + Hash + BitXor<Output = Self>>(H, H);
impl<H: Clone + Hash + BitXor<Output = Self>> Hash for Pair<H> {
    fn hash<T: Hasher>(&self, state: &mut T) {
        (self.0.clone() ^ self.1.clone()).hash(state);
    }
}
