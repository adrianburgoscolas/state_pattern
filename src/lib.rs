pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content<'a>(&'a self) -> &'a str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().editable() {
            self.content.push_str(text);
        }  
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn editable(&self) -> bool {
        false
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State>;
    
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn editable(&self) -> bool {
        true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {count: 1})
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {
    count: u8
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.count == 0 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview {count: self.count - 1})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_post_draft_state() {
        let mut post = Post::new();
        post.add_text("hola que tal");
        assert_eq!("", post.content());
    }

    #[test]
    fn test_post_pendingreview_state() {
        let mut post = Post::new();
        post.add_text("hola que tal");
        post.request_review();
        assert_eq!("", post.content());
    }

    #[test]
    fn test_post_published_state() {
        let mut post = Post::new();
        post.add_text("hola que tal las cosas");
        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("hola que tal las cosas", post.content());
    }

    #[test]
    fn test_no_changeback_to_pendingreview_from_published() {
        let mut post = Post::new();
        post.add_text("hola que tal por alla");
        post.request_review();
        post.approve();
        post.approve();
        post.request_review();
        post.reject();
        assert_eq!("hola que tal por alla", post.content());
    }

    #[test]
    fn test_post_rejected_after_reviewed() {
        let mut post = Post::new();
        post.add_text("nuevo post hola que tal por alla");
        post.request_review();
        post.reject();
        post.approve();
        post.approve();
        assert_eq!("", post.content());
    }

    #[test]
    fn test_can_only_edit_draft() {
        let mut post = Post::new();
        post.add_text("hola que");
        post.request_review();
        post.add_text(" tal");
        post.reject();
        post.add_text(", como va todo?");
        post.request_review();
        post.add_text("asdf");
        post.approve();
        post.approve();
        post.add_text("las cosas");
        assert_eq!("hola que, como va todo?", post.content());
    }
}
