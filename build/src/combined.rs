use crate::read::{RawChapter, RawStory};
use crate::setup::{RawSetup, RAW_SETUPS};
use databake::converter::{AsStaticStr, VecAsRefSlice};
use databake::Bake;
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
            stories: vec![].into(),
        }
    }
}

pub(crate) fn combine(
    mut language_stories: Vec<(Language, Vec<RawStory>)>,
) -> VecAsRefSlice<Setup> {
    let mut result: VecAsRefSlice<Setup> = RAW_SETUPS.iter().map(|rs| rs.into()).collect();

    for setup in result.iter_mut() {
        'language_loop: for (language, stories) in language_stories.iter_mut() {
            for story in stories {
                if story.name == setup.name {
                    setup.stories.push(Story {
                        language: *language,
                        name: mem::take(&mut story.name).into(),
                        chapters: mem::take(&mut story.chapters)
                            .into_iter()
                            .map(|x| x.into())
                            .collect(),
                    });
                    continue 'language_loop;
                }
            }
            panic!("Missing story for {} in {:?}", setup.name, language);
        }
    }

    result
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
    pub name: AsStaticStr<String>,
    pub chapters: VecAsRefSlice<Chapter>,
}

#[derive(Bake)]
#[databake(path = crate::game::generated)]
pub struct Chapter {
    pub name: AsStaticStr<String>,
    pub paragraphs: VecAsRefSlice<AsStaticStr<String>>,
}

impl From<RawChapter> for Chapter {
    fn from(value: RawChapter) -> Self {
        Chapter {
            name: value.name.into(),
            paragraphs: value
                .paragraphs
                .into_iter()
                .map(AsStaticStr::from)
                .collect(),
        }
    }
}
