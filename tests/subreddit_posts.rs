use futures::stream::StreamExt;
use reddit_api::structs::SubredditSort;
use reddit_api::RedditClient;

#[tokio::test]
async fn subreddit_posts() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .subreddit_posts("dreamcatcher", SubredditSort::New)
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
