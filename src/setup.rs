use crate::game::{Setup, SetupId};
use crate::state::Page;
use crate::{App, Msg};
use yew::{html, Context, Html};
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

impl App {
    pub(crate) fn view_setup(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class="list-group" style="text-align: left">
                if !self.state.list_tiles.is_empty() {
                    <li class="list-group-item">
                        <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Page(Page::List)) }>
                            { "Back" }
                        </Button>
                    </li>
                }
                { for Setup::all().map(|(i,s)| Self::view_setup_line(ctx, i, s)) }
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Page(Page::Custom)) }>
                        { "Custom" }
                    </Button>
                </li>
            </ul>
        }
    }

    fn view_setup_line(ctx: &Context<Self>, id: SetupId, setup: &Setup) -> Html {
        html! {
            <li class="list-group-item">
                { setup.name }
                { " " }
                <Button style={Color::Success} onclick={ ctx.link().callback(move |_| Msg::Setup(id)) }>
                    { "Setup" }
                </Button>
            </li>
        }
    }
}
