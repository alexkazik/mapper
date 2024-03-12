use crate::game::TileId;
use crate::state::Page;
use crate::{App, Msg};
use serde::{Deserialize, Serialize};
use yew::{html, Context, Html};
use yew_bootstrap::component::Button;
use yew_bootstrap::icons::BI;
use yew_bootstrap::util::Color;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum Sort {
    Name,
    Type,
}

impl App {
    pub(crate) fn view_list(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            if let Some(setup_id) = self.state.setup_id {
                <h4>
                    {setup_id.get().name}
                </h4>
                <ul class="nav nav-tabs justify-content-center">
                    <li class="nav-item">
                        <a class="nav-link active" aria-current="page">{"Map"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" onclick={ ctx.link().callback(move |_| Msg::Page(Page::Story)) }>{"Story"}</a>
                    </li>
                </ul>
            }
            <table class="table table-borderless" style="text-align: left">
                <thead>
                    <tr>
                        <th></th>
                        if self.state.list_sort == Sort::Name {
                            <th>{ "Name" }{BI::SORT_ALPHA_DOWN}</th>
                        } else {
                            <th onclick={ctx.link().callback(|_| Msg::ListSetSort(Sort::Name))}>{ "Name" }</th>
                        }
                        if self.state.list_sort == Sort::Type {
                            <th>{ "Type" }{BI::SORT_ALPHA_DOWN}</th>
                        } else {
                            <th onclick={ctx.link().callback(|_| Msg::ListSetSort(Sort::Type))}>{ "Type" }</th>
                        }
                        <th style="display: flex; justify-content: center"><div class="form-check form-switch"><input
                            type="checkbox"
                            role="switch"
                            class="form-check-input"
                            checked={ self.state.list_show_discovered }
                            onclick={ ctx.link().callback(|_| Msg::ListToggleShowDiscovered) }
                        /></div></th>
                    </tr>
                </thead>
                <tbody>
                { for self.state.list_tiles.iter().filter(|(_,d)|self.state.list_show_discovered || !d).map(|(t,d)|self.view_list_line(ctx, *t, *d)) }
                </tbody>
            </table>
            <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Page(Page::Setup)) }>{ "New Adventure" }</Button>
            <div style="display: flex">
                { "Settings: " }
                <div class="form-check form-switch"><input
                    type="checkbox"
                    role="switch"
                    id="settings_tile_set"
                    class="form-check-input"
                    checked={ self.state.list_tile_set }
                    onclick={ ctx.link().callback(|_| Msg::ListToggleTileSet) }
                /><label class="form-check-label" for="settings_tile_set">{ "Map tiles" }</label></div>
                <div class="form-check form-switch"><input
                    type="checkbox"
                    role="switch"
                    id="settings_the"
                    class="form-check-input"
                    checked={ self.state.list_the }
                    onclick={ ctx.link().callback(|_| Msg::ListToggleThe) }
                /><label class="form-check-label" for="settings_the">{ "\"The\"" }</label></div>
            </div>
            </>
        }
    }

    fn view_list_line(&self, ctx: &Context<Self>, id: TileId, del: bool) -> Html {
        let tile = id.get();
        let (btn_icon, btn_style) = if del {
            (BI::ARROW_COUNTERCLOCKWISE, Color::Primary)
        } else {
            (BI::COMPASS, Color::Success)
        };
        let name = tile.name(self.state.list_the);
        html! {
            <tr class={ tile.color_class() } style={ if del {"color: #666"} else {""} } >
                <td align="center"> if self.state.list_tile_set { { tile.tile_set.to_string() } } </td>
                <td>{ name.0 }{ name.1 }</td>
                <td>{ tile.tile_type }</td>
                <td align="center">
                    <Button style={btn_style} onclick={ ctx.link().callback(move |_| Msg::ListToggleDiscovered(id)) }>
                        {btn_icon}
                    </Button>
                </td>
            </tr>
        }
    }
}
