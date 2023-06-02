use futures::stream::StreamExt;
use reddit_api::structs::SearchCommentsSort;
use reddit_api::RedditClient;

#[tokio::test]
async fn search_comments() {
    let client = RedditClient::new().unwrap();

    let mut stream = client
        .search_comments("dreamcatcher", SearchCommentsSort::Relevance)
        .await
        .take(100);
    let mut count = 0;
    while let Some(comment) = stream.next().await {
        let comment = comment.unwrap();
        dbg!(&comment);
        count += 1;
    }
    assert_eq!(count, 100);
}
