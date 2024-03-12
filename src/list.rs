use crate::game::TileId;
use crate::state::Page;
use crate::{App, Msg};
use serde::{Deserialize, Serialize};
use yew::{classes, html, Context, Html};

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum Sort {
    Name,
    Type,
}

impl App {
    pub(crate) fn view_list(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <table class="table table-borderless" style="text-align: left">
                <thead>
                    <tr>
                        <th></th>
                        if self.state.list_sort == Sort::Name {
                            <th>{ "Name" }<i class="bi bi-sort-alpha-down"></i></th>
                        } else {
                            <th onclick={ctx.link().callback(|_| Msg::ListSetSort(Sort::Name))}>{ "Name" }</th>
                        }
                        if self.state.list_sort == Sort::Type {
                            <th>{ "Type" }<i class="bi bi-sort-alpha-down"></i></th>
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
            <button class="btn btn-primary" onclick={ ctx.link().callback(move |_| Msg::Page(Page::Setup)) }>{ "New Adventure" }</button>
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
        let (btn_icon, btn_class) = if del {
            ("bi-arrow-counterclockwise", "btn-primary")
        } else {
            ("bi-compass", "btn-success")
        };
        let name = tile.name(self.state.list_the);
        html! {
            <tr class={ tile.color_class() } style={ if del {"color: #666"} else {""} } >
                <td align="center"> if self.state.list_tile_set { { tile.tile_set.to_string() } } </td>
                <td>{ name.0 }{ name.1 }</td>
                <td>{ tile.tile_type }</td>
                <td align="center">
                    <button class={ classes!("btn", btn_class) } onclick={ ctx.link().callback(move |_| Msg::ListToggleDiscovered(id)) }>
                        <i class={ classes!("bi", btn_icon) }></i>
                    </button>
                </td>
            </tr>
        }
    }
}
