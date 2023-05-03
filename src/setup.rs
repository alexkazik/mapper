use crate::game::{Setup, SetupId};
use crate::state::Page;
use crate::{App, Msg};
use yew::{html, Context, Html};

impl App {
    pub(crate) fn view_setup(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class="list-group" style="text-align: left">
                if !self.state.list_tiles.is_empty() {
                    <li class="list-group-item">
                        <button class="btn btn-primary" onclick={ ctx.link().callback(move |_| Msg::Page(Page::List)) }>
                            { "Back" }
                        </button>
                    </li>
                }
                { for Setup::all().map(|(i,s)| Self::view_setup_line(ctx, i, s)) }
                <li class="list-group-item">
                    <button class="btn btn-primary" onclick={ ctx.link().callback(move |_| Msg::Page(Page::Custom)) }>
                        { "Custom" }
                    </button>
                </li>
            </ul>
        }
    }

    fn view_setup_line(ctx: &Context<Self>, id: SetupId, setup: &Setup) -> Html {
        html! {
            <li class="list-group-item">
                { setup.name }
                { " " }
                <button class="btn btn-success" onclick={ ctx.link().callback(move |_| Msg::Setup(id)) }>
                    { "Setup" }
                </button>
            </li>
        }
    }
}
