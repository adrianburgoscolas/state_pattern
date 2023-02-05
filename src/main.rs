use state_pattern::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("hola que tal");
    println!("Added text");
    println!("{}", post.content());
    post.request_review();
    println!("Reviewd");
    println!("{}", post.content());
    post.approve();
    println!("Approved");
    println!("{}", post.content());
}
