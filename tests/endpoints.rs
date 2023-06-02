use futures::stream::StreamExt;
use reddit_api::structs::{SearchCommentsSort, SearchPostsSort, SortRange, SubredditSort};
use reddit_api::RedditClient;

const NUM_ITEMS: usize = 100;

#[tokio::test]
async fn subreddit_posts() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .subreddit_posts("dreamcatcher", SubredditSort::New)
        .await
        .take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn search_posts() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .search_posts("dreamcatcher", SearchPostsSort::Relevance(SortRange::All))
        .await
        .take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn search_comments() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .search_comments("dreamcatcher", SearchCommentsSort::Relevance)
        .await
        .take(NUM_ITEMS);
    let mut count = 0;
    while let Some(comment) = stream.next().await {
        let comment = comment.unwrap();
        dbg!(&comment);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}
