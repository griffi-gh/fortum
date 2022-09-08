use super::inner_prelude::*;
use crate::common::post::{Post, PostFilter, PostSort, SortDirection};
use crate::common::utils::div_up;

impl MainDatabase {
  // Assumes that user exists!
  pub async fn submit_post(db: &mut Connection<Self>, author: Option<i32>, topic_id: i32, title: &str, body: Option<&str>) -> Result<i32, &'static str> {
    if title.len() > 255 {
      return Err("Post title too long");
    }
    if let Some(body) = body {
      if body.len() > 3000 {
        return Err("Post body too long");
      }
    }
    if title.trim().is_empty() {
      return Err("Empty post title"); 
    }
    let mut body = body;
    if let Some(body_inner) = body {
      if body_inner.trim().is_empty() {
        body = None;
      }
    }
    let topic_exists: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM topics WHERE topic_id = $1 LIMIT 1")
      .bind(topic_id)
      .fetch_one(executor(db)).await.unwrap()
      .get(0);
    if !topic_exists {
      return Err("Topic doensn't exist");
    }
    let post_id = sqlx::query("INSERT INTO posts (author, topic, title, content) VALUES($1, $2, $3, $4) RETURNING post_id")
      .bind(author)
      .bind(topic_id)
      .bind(title)
      .bind(body)
      .fetch_one(executor(db)).await
      .unwrap().get(0);
    sqlx::query("UPDATE stats SET posts = posts + 1").execute(executor(db)).await.unwrap();
    Ok(post_id)
  }

  pub async fn fetch_posts(db: &mut Connection<Self>, sort: PostSort, filter: PostFilter<'_>, page: u32, max_results_on_page: u32) -> Vec<Post> {
    //This is pretty inefficient but hey it works!
    //OFFSET is slow: https://use-the-index-luke.com/no-offset
    assert!(max_results_on_page > 0);
    let filter_options = match filter {
      PostFilter::None => (0, None, None, None),
      PostFilter::ByTopicId(id) => (1, Some(id), None, None),
      PostFilter::ByTopicName(name) => (2, None, Some(name), None),
      PostFilter::ByUserId(id) => (3, None, None, Some(id)),
      #[allow(unreachable_patterns)]
      _ => unimplemented!("Filter type not implemented")
    };
    sqlx::query_as!(Post, r#"
        SELECT 
          users.username AS "username?", 
          users.profile_image AS "profile_image?",
          posts.title AS "title!", 
          posts.content AS "content?", 
          posts.created_on AS "created_on!", 
          topics.topic_name AS "topic_name!", 
          posts.votes AS "votes!",
          users.user_id AS "user_id?",
          posts.post_id as "post_id!"
        FROM posts
        LEFT JOIN users ON users.user_id = posts.author
        INNER JOIN topics ON topics.topic_id = posts.topic
        WHERE (
          $4 = 0 OR
          ($4 = 1 AND topics.topic_id = $5) OR
          ($4 = 2 AND topics.topic_name = $6) OR
          ($4 = 3 AND users.user_id = $7)
        )
        ORDER BY
          CASE WHEN $1 = 1 THEN posts.created_on END DESC, 
          CASE WHEN $1 = 2 THEN posts.created_on END ASC, 
          CASE WHEN $1 = 3 THEN posts.votes END DESC, 
          CASE WHEN $1 = 4 THEN posts.votes END ASC
        LIMIT $2 OFFSET $3
      "#, 
      match sort {
        PostSort::ByDate(ord) => match ord {
          SortDirection::Descending => 1,
          SortDirection::Ascending => 2,
        },
        PostSort::ByVotes(ord) => match ord {
          SortDirection::Descending => 3,
          SortDirection::Ascending => 4,
        },
        #[allow(unreachable_patterns)]
        _ => unimplemented!("Sort type not implemented")
      }, 
      max_results_on_page as i64,
      (page as i64) * (max_results_on_page as i64),
      filter_options.0,
      filter_options.1,
      filter_options.2,
      filter_options.3
    ).fetch_all(executor(db)).await.unwrap()
  }

  pub async fn count_pages(db: &mut Connection<Self>, filter: PostFilter<'_>, results_per_page: u32) -> u32 {
    //! //HACK Turn this into it's own query, this is extemely slow
    let post_count = Self::fetch_posts(db, PostSort::ByDate(SortDirection::Descending), filter, 0, u32::MAX).await.len();
    div_up(post_count, results_per_page as usize) as u32
  }

  //TODO somehow `join` instead of awaiting. Requires two mut references to db
  pub async fn fetch_posts_and_count_pages(db: &mut Connection<Self>, sort: PostSort, filter: PostFilter<'_>, page: u32, max_results_on_page: u32) -> (Vec<Post>, u32) {
    let posts = Self::fetch_posts(db, sort, filter, page, max_results_on_page).await;
    let pages = Self::count_pages(db, filter, max_results_on_page).await;
    (posts, pages)
  }

  pub async fn get_post(db: &mut Connection<Self>, id: i32) -> Option<Post> {
    sqlx::query_as!(Post, r#"
      SELECT 
        users.username AS "username?", 
        users.profile_image AS profile_image,
        posts.title AS title, 
        posts.content AS content, 
        posts.created_on AS created_on, 
        topics.topic_name AS topic_name, 
        posts.votes AS votes,
        users.user_id AS "user_id?",
        posts.post_id as post_id
      FROM posts
      LEFT JOIN users ON users.user_id = posts.author
      INNER JOIN topics ON topics.topic_id = posts.topic
      WHERE posts.post_id = $1
      ORDER BY votes DESC
    "#, id).fetch_optional(executor(db)).await.unwrap()
  }
}
