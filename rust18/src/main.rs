
pub trait Summarizble{
    fn summary(&self) -> String;
}
pub struct NewArticle{
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summarizble for NewArticle{
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool
}

impl Summarizble for Tweet{
    fn summary(&self) -> String{
        format!("{} : {}", self.username, self.content)
    }
}

fn main(){

    let tweet = Tweet {
        username : String::from("hores_ebooks"),
        content : String::from("of course, as you probably alreaady know, people"),
        reply : false,
        retweet : false
    };

    println!("1 new tweet : {}", tweet.summary());

    let new_article = NewArticle{
        headline : String::from("headline"),
        location : String::from("location"),
        author : String::from("author"),
        content : String::from("content"),
    };
    println!("new article : {}", new_article.summary());
}