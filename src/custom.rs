use crate::game::Tile;
use crate::state::Page;
use crate::{App, Msg};
use yew::{classes, html, Context, Html};

impl App {
    pub(crate) fn view_custom(&self, ctx: &Context<Self>) -> Html {
        let num_tiles = Tile::generate(&self.state.custom_tile_sets, &[]).len();
        let hint = match num_tiles {
            19 => "(This matches a 4*5 grid, including the City of Kings)",
            24 => "(This matches a 5*5 grid, including the City of Kings)",
            _ => "",
        };
        html! {
            <ul class="list-group" style="text-align: left">
                <li class="list-group-item">
                    <button class="btn btn-primary" onclick={ ctx.link().callback(move |_| Msg::Page(Page::Setup)) }>
                        { "Back" }
                    </button>
                </li>
                <li class="list-group-item">
                    { "Please select the used tile sets:" }
                </li>
                <li class="list-group-item">
                    <table>
                        <tbody>
                            { for (0..4).map(|l| self.view_custom_line(ctx, l)) }
                        </tbody>
                    </table>
                </li>
                <li class="list-group-item">
                    { "Tiles on the table:" }{ " " }{ num_tiles }{ " " }{ hint }
                </li>
                <li class="list-group-item">
                    <button class="btn btn-primary" onclick={ ctx.link().callback(move |_| Msg::CustomSetup) } disabled={num_tiles == 0}>
                        { "Setup" }
                    </button>
                </li>
            </ul>
        }
    }

    fn view_custom_line(&self, ctx: &Context<Self>, l: usize) -> Html {
        html! {
            <tr>
                { for (0..4).map(|c| self.view_custom_column(ctx, l*4+c+1)) }
            </tr>
        }
    }

    fn view_custom_column(&self, ctx: &Context<Self>, tile_set: usize) -> Html {
        let btn_class = if self.state.custom_tile_sets.contains(&tile_set) {
            "btn-success"
        } else {
            "btn-outline-success"
        };
        html! {
            <td>
                <button class={classes!("btn", btn_class)} onclick={ ctx.link().callback(move |_| Msg::CustomToggleTileSet(tile_set)) }>
                    { tile_set }
                </button>
            </td>
        }
    }
}
