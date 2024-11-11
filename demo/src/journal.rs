
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Journal{
    #[serde(skip)]
    pub pages: Vec<Page>,
    current_page_index: usize
}

impl Default for Page {
    fn default() -> Self {
        Page{
            title: "It's a good day".into(),
            content: "Hello testing 1 2 3".into(),
            date: (27,9,24),
            day_of_week: Friday,
            mode: PageMode::Lazy(0)
        }
    }
}

impl Default for Journal {
    fn default() -> Self {
        let pages = vec![
            Page::new("It's a good day","Hello testing 1 2 3",(27,9,24),Friday),
            Page{
                title: "It's not a good day".into(),
                content: "Such sadness".into(),
                date: (28,9,24),
                day_of_week: Saturday,
                mode: PageMode::Lazy(0)
            },
            Page{
                title: "It's amazing".into(),
                content: "Hello world".into(),
                date: (29,9,24),
                day_of_week: Sunday,
                mode: PageMode::Lazy(0)
            },
            Page{
                title: "'A' day".into(),
                content: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                date: (30,9,24),
                day_of_week: Monday,
                mode: PageMode::Lazy(0)
            }
        ];
        Journal {
            pages,
            current_page_index: 0,
        }
    }
}

impl Journal {
    pub fn new() -> Self{
        Self::default()
    }

    pub fn get_current_page(&self) -> Option<Page> {
        self.pages.get(self.current_page_index).cloned()
    }

    pub fn update(&mut self) {
        if let Some(page) = self.pages.get_mut(self.current_page_index){
            if let PageMode::Lazy(x) = &mut page.mode {
                if *x / 5 >= page.content.len() {
                    page.mode = PageMode::Loaded;
                } else {
                    *x+=1;
                }
            } 
        }
    }

    pub fn next_page(&mut self){
        if self.current_page_index < self.pages.len() - 1 {
            self.current_page_index += 1;
        }
    }

    pub fn prev_page(&mut self){
        if self.current_page_index > 0 {
            self.current_page_index -= 1;
        }
    }

    // pub fn render(&mut self){

    // }
}

#[derive(Clone)]
pub struct Page {
    pub title: String,
    pub content: String,
    pub date: (u8,u8,u8),
    pub day_of_week: Day,
    pub mode: PageMode
}

impl Page {
    pub fn new<S:Into<String>,S2:Into<String>>(title:S,content:S2,date:(u8,u8,u8),day_of_week: Day) -> Self{
        Self {
            title:title.into(),
            content: content.into(),
            date,
            day_of_week,
            mode: PageMode::Lazy(0)
        }
    }
}

#[derive(Clone)]
pub enum PageMode {
    Immediate,
    Lazy(usize),
    Loaded
} 

#[derive(serde::Deserialize, serde::Serialize,Clone)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

use Day::*;

impl Day {
    pub fn as_str(&self) -> &'static str{
        match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday"
        }
    }
}