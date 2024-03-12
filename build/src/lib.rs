mod combined;
mod setup;

use crate::combined::combine;
use databake::{Bake, TokenStream};

pub fn generate() -> TokenStream {
    combine().bake(&Default::default())
}
