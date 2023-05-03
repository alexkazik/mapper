#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use crate::game::{SetupId, TileId};
use crate::list::Sort;
use crate::state::Page;
use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use state::State;
use yew::{html, Component, Context, Html};

mod game;
mod list;
mod setup;
mod state;

pub(crate) enum Msg {
    Page(Page),
    Setup(SetupId),
    ListToggleDiscovered(TileId),
    ListSetSort(Sort),
    ListToggleShowDiscovered,
    ListToggleTileSet,
    ListToggleThe,
}

pub(crate) struct App {
    state: State,
}

impl App {
    const STORAGE_KEY: &'static str = "app";
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: LocalStorage::get::<State>(Self::STORAGE_KEY)
                .unwrap_or_default()
                .validate(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Page(page) => self.state.page = page,
            Msg::Setup(id) => {
                self.state.list_tiles = id.get().tiles();
                self.state.sort();
                self.state.page = Page::List;
            }
            Msg::ListToggleDiscovered(id) => {
                for (i, d) in &mut self.state.list_tiles {
                    if *i == id {
                        *d ^= true;
                    }
                }
            }
            Msg::ListSetSort(sort) => {
                self.state.list_sort = sort;
                self.state.sort();
            }
            Msg::ListToggleShowDiscovered => self.state.list_show_discovered ^= true,
            Msg::ListToggleTileSet => self.state.list_tile_set ^= true,
            Msg::ListToggleThe => {
                self.state.list_the ^= true;
                self.state.sort();
            }
        }
        let _: Result<(), StorageError> = LocalStorage::set(Self::STORAGE_KEY, &self.state);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inner = match self.state.page {
            Page::Setup => self.view_setup(ctx),
            Page::List => self.view_list(ctx),
        };
        html! {
            <div class="container" style="text-align: center">
                <h1>{ "Unofficial The City of Kings Mapper" }</h1>
                { inner }
                <hr/>
                <h4>{ "Thanks to Frank West for the game." }</h4>
                <h5>{ "Written by Alex." }</h5>
                <p>
                    { "Version: " }{ env!("CARGO_PKG_VERSION") }<br/>
                    <a href="https://github.com/alexkazik/mapper">{ "Source" }</a>
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
