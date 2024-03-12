mod combined;
mod read;
mod setup;

use crate::combined::{combine, Language};
use crate::read::parse_file;
use databake::{Bake, TokenStream};

pub fn generate() -> TokenStream {
    combine(vec![
        (
            Language::English,
            parse_file(include_str!("../../stories.txt")),
        ),
        (
            Language::German,
            parse_file(include_str!("../../stories_de.txt")),
        ),
    ])
    .bake(&Default::default())
}
