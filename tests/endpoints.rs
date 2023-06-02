use futures::stream::StreamExt;
use reddit_api::structs::{SearchCommentsSort, SearchPostsSort, SortRange, SubredditSort};
use reddit_api::RedditClient;

const NUM_ITEMS: usize = 100; // Large enough value to page a few times

#[tokio::test]
async fn subreddit_posts() {
    let client = RedditClient::new().unwrap();

    let query = client
        .subreddit_posts_query()
        .subreddit("dreamcatcher")
        .sort(SubredditSort::New)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn subreddit_posts_nsfw() {
    let client = RedditClient::new().unwrap();

    let query = client
        .subreddit_posts_query()
        .subreddit("nsfw")
        .sort(SubredditSort::New)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
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

    let query = client
        .search_posts_query()
        .query("dreamcatcher")
        .sort(SearchPostsSort::Relevance(SortRange::All))
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn search_posts_in_sub() {
    let client = RedditClient::new().unwrap();

    let query = client
        .search_posts_query()
        .query("jiu")
        .sort(SearchPostsSort::New)
        .subreddit("dreamcatcher")
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn search_posts_nsfw() {
    let client = RedditClient::new().unwrap();

    // Try searching for nsfw posts without enabling nsfw
    let query = client
        .search_posts_query()
        .query("nsfw:true")
        .sort(SearchPostsSort::New)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, 0);

    // Do the same but enable nsfw
    let query = client
        .search_posts_query()
        .nsfw(true)
        .query("nsfw:true")
        .sort(SearchPostsSort::New)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
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

    let query = client
        .search_comments_query()
        .query("dreamcatcher")
        .sort(SearchCommentsSort::Relevance)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(comment) = stream.next().await {
        let comment = comment.unwrap();
        dbg!(&comment);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}

#[tokio::test]
async fn search_comments_nsfw() {
    let client = RedditClient::new().unwrap();

    // Try searching for nsfw posts
    // Disabling nsfw but searching for "nsfw:true" doesn't seem to work correctly on Reddit's end
    let query = client
        .search_comments_query()
        .nsfw(true)
        .query("nsfw:true")
        .sort(SearchCommentsSort::New)
        .build();
    let mut stream = query.execute().await.take(NUM_ITEMS);
    let mut count = 0;
    while let Some(post) = stream.next().await {
        let post = post.unwrap();
        dbg!(&post);
        count += 1;
    }
    assert_eq!(count, NUM_ITEMS);
}
