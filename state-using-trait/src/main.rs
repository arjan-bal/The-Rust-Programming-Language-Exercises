use state_using_trait::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I didn't eat a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I didn't eat a salad for lunch today", post.content());
}
