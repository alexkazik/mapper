use crate::state::Page;
use crate::{App, Msg};
use yew::{html, Context, Html};
use yew_bootstrap::component::Button;
use yew_bootstrap::icons::BI;
use yew_bootstrap::util::Color;

impl App {
    pub(crate) fn view_story(&self, ctx: &Context<Self>) -> Html {
        if let Some(setup_id) = self.state.setup_id {
            let stories = setup_id.get().stories;
            let story = stories
                .iter()
                .find(|x| x.language == self.state.language)
                .unwrap_or(&stories[0]);
            let story_chapter = if self.state.story_chapter < story.chapters.len() {
                self.state.story_chapter
            } else {
                0
            };
            let c = &story.chapters[story_chapter];

            let p = c.paragraphs.iter().enumerate().map(|(i, p)| {
                if i == 0 && c.name != "The End" {
                    html! {
                        <p><b>{p}</b></p>
                    }
                } else {
                    html! {
                        <p>{p}</p>
                    }
                }
            });

            let languages = stories.iter().filter(|s| s.language != story.language).map(|s|{
                html!{ <li><a class="dropdown-item" onclick={ctx.link().callback(move |_| Msg::Language(s.language))}>{s.language.as_str()}</a></li> }
            });

            return html! {
                <>
                    <h4>
                        {story.name}
                    </h4>
                    <ul class="nav nav-tabs justify-content-center">
                        <li class="nav-item">
                            <a class="nav-link" onclick={ ctx.link().callback(move |_| Msg::Page(Page::List)) }>{"Map"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link active" aria-current="page">{"Story"}</a>
                        </li>
                    </ul>
                    <div class="btn-group" role="group" style="padding: 10px">
                        if story.chapters.len() > 1 {
                            <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Chapter(story_chapter.saturating_sub(1)))} disabled={story_chapter == 0}>{BI::CHEVRON_LEFT}</Button>
                        }
                        if stories.len() > 1 {
                            <div class="btn-group" role="group">
                                <button type="button" class="btn btn-primary dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                                    {story.language.as_str()}
                                </button>
                                <ul class="dropdown-menu">
                                    {for languages}
                                    <li><a class="dropdown-item" href="https://github.com/alexkazik/mapper#translation" target="_blank">{"Info"}</a></li>
                                </ul>
                            </div>
                        }
                        if story.chapters.len() > 1 {
                            <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Chapter(story_chapter.saturating_add(1)))} disabled={story_chapter >= story.chapters.len()-1}>{BI::CHEVRON_RIGHT}</Button>
                        }
                    </div>
                    <h4>
                        {c.name}
                    </h4>
                    <div class="container d-flex align-items-center justify-content-center">
                        <div style="font-size: 1.3rem; width: 50%">
                            {for p}
                        </div>
                    </div>
                </>
            };
        }
        html! {
            <Button style={Color::Primary} onclick={ ctx.link().callback(move |_| Msg::Page(Page::List)) }>{ "Back" }</Button>
        }
    }
}
