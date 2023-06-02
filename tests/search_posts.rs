use futures::stream::StreamExt;
use reddit_api::structs::{SearchPostsSort, SortRange};
use reddit_api::RedditClient;

#[tokio::test]
async fn search_posts() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .search_posts("dreamcatcher", SearchPostsSort::Relevance(SortRange::All))
        .await
        .take(100);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, 100);
}
