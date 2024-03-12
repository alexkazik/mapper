use crate::setup::{RawSetup, RAW_SETUPS};
use databake::{quote, Bake, CrateEnv, TokenStream};

#[derive(Bake)]
#[databake(path = crate::game::generated)]
pub(crate) struct Setup {
    pub(crate) name: &'static str,
    pub(crate) tile_sets: &'static [usize],
    pub(crate) discovered: &'static [usize],
}

impl From<&RawSetup> for Setup {
    fn from(value: &RawSetup) -> Self {
        Setup {
            name: value.name,
            tile_sets: value.tile_sets,
            discovered: value.discovered,
        }
    }
}

pub(crate) fn combine() -> VecAsRefSlice<Setup> {
    let result = RAW_SETUPS
        .iter()
        .map(|rs| rs.into())
        .collect::<Vec<Setup>>();

    VecAsRefSlice(result)
}

#[repr(transparent)]
pub struct StringAsStaticStr(pub String);

impl Bake for StringAsStaticStr {
    fn bake(&self, _ctx: &CrateEnv) -> TokenStream {
        let value = &self.0;
        quote!(#value)
    }
}

#[repr(transparent)]
pub struct VecAsRefSlice<T>(pub Vec<T>);

impl<T> Bake for VecAsRefSlice<T>
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let mut inner = TokenStream::new();
        for e in self.0.iter() {
            let e = e.bake(ctx);
            inner.extend(quote! {#e,});
        }

        quote! {&[#inner]}
    }
}
