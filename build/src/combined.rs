use crate::read::{RawChapter, RawStory};
use crate::setup::{RawSetup, RAW_SETUPS};
use databake::{quote, Bake, CrateEnv, TokenStream};
use std::mem;

#[derive(Bake)]
#[databake(path = crate::game::generated)]
pub(crate) struct Setup {
    pub(crate) name: &'static str,
    pub(crate) tile_sets: &'static [usize],
    pub(crate) discovered: &'static [usize],
    pub(crate) stories: VecAsRefSlice<Story>,
}

impl From<&RawSetup> for Setup {
    fn from(value: &RawSetup) -> Self {
        Setup {
            name: value.name,
            tile_sets: value.tile_sets,
            discovered: value.discovered,
            stories: VecAsRefSlice(vec![]),
        }
    }
}

pub(crate) fn combine(
    mut language_stories: Vec<(Language, Vec<RawStory>)>,
) -> VecAsRefSlice<Setup> {
    let mut result = RAW_SETUPS
        .iter()
        .map(|rs| rs.into())
        .collect::<Vec<Setup>>();

    for setup in result.iter_mut() {
        'language_loop: for (language, stories) in language_stories.iter_mut() {
            for story in stories {
                if story.name == setup.name {
                    setup.stories.0.push(Story {
                        language: *language,
                        name: StringAsStaticStr(mem::take(&mut story.name)),
                        chapters: VecAsRefSlice(
                            mem::take(&mut story.chapters)
                                .into_iter()
                                .map(|x| x.into())
                                .collect(),
                        ),
                    });
                    continue 'language_loop;
                }
            }
            panic!("Missing story for {} in {:?}", setup.name, language);
        }
    }

    VecAsRefSlice(result)
}

#[derive(Copy, Clone, Debug, Bake)]
#[databake(path = crate::game::generated)]
pub enum Language {
    English,
    German,
}

#[derive(Bake)]
#[databake(path = crate::game::generated)]
pub struct Story {
    pub language: Language,
    pub name: StringAsStaticStr,
    pub chapters: VecAsRefSlice<Chapter>,
}

#[derive(Bake)]
#[databake(path = crate::game::generated)]
pub struct Chapter {
    pub name: StringAsStaticStr,
    pub paragraphs: VecAsRefSlice<StringAsStaticStr>,
}

impl From<RawChapter> for Chapter {
    fn from(value: RawChapter) -> Self {
        Chapter {
            name: StringAsStaticStr(value.name),
            paragraphs: VecAsRefSlice(
                value
                    .paragraphs
                    .into_iter()
                    .map(StringAsStaticStr)
                    .collect(),
            ),
        }
    }
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
