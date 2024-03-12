use std::mem;

pub fn parse_file(contents: &str) -> Vec<RawStory> {
    let mut result = Builder {
        all: vec![],
        current_story: vec![],
        current_story_name: "",
        current_chapter: vec![],
        current_chapter_name: "",
    };
    contents.lines().for_each(|l| result.add_line(l));
    result.finish_story();
    result.all
}

struct Builder<'a> {
    all: Vec<RawStory>,
    current_story: Vec<RawChapter>,
    current_story_name: &'a str,
    current_chapter: Vec<&'a str>,
    current_chapter_name: &'a str,
}

pub struct RawStory {
    pub name: String,
    pub chapters: Vec<RawChapter>,
}

pub struct RawChapter {
    pub name: String,
    pub paragraphs: Vec<String>,
}

impl<'a> Builder<'a> {
    fn add_line(&mut self, line: &'a str) {
        if line.starts_with('$') {
            return;
        }
        if let Some(l) = line.strip_prefix("##") {
            self.finish_chapter();
            self.current_chapter_name = l.trim();
        } else if let Some(l) = line.strip_prefix('#') {
            self.finish_story();
            self.current_story_name = l.trim();
        } else {
            self.current_chapter.push(line.trim());
        }
    }

    fn finish_chapter(&mut self) {
        let paragraphs = self
            .current_chapter
            .split(|x| x.is_empty())
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>();

        if !paragraphs.is_empty() {
            self.current_story.push(RawChapter {
                name: self.current_chapter_name.to_string(),
                paragraphs,
            });
        }

        self.current_chapter.clear();
    }

    fn finish_story(&mut self) {
        self.finish_chapter();

        if !self.current_story.is_empty() {
            self.all.push(RawStory {
                name: self.current_story_name.to_string(),
                chapters: mem::take(&mut self.current_story),
            });
        }
    }
}
