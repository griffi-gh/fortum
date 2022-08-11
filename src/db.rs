use rocket_db_pools::{Database, Connection};
use sqlx::{self, PgPool, Row};
use argon2::{self, Config as ArgonConfig};
use rand::{Rng, thread_rng};
use crate::{consts::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX}, common::TemplatePost};
use crate::common::{executor, div_up, User, PostFilter, Stats, PostSort, SortDirection};

#[derive(Database)]
#[database("main")]
pub struct MainDatabase(PgPool);
impl MainDatabase {
  /// Returns token
  pub async fn register(db: &mut Connection<Self>, email: &str, username: &str, password: &str) -> Result<String, &'static str> {
    //Validate email, username and password
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !USERNAME_REGEX.is_match(username) {
      return Err("Invalid username");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
    //Check if email was used before
    let email_used: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM users WHERE email = $1 LIMIT 1")
      .bind(&email)
      .fetch_one(executor(db)).await
      .unwrap().get(0);
    if email_used {
      return Err("This email address is already in use");
    }
    //Register user
    let mut salt = [0u8; 16];
    thread_rng().fill(&mut salt);
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt[..], &ArgonConfig::default()).unwrap();
    let token = {
      let mut data = [0u8; 16];
      thread_rng().fill(&mut data);
      base64::encode_config(data, base64::URL_SAFE)
    };
    debug_assert!(token.len() == 24, "Invalid token length");
    sqlx::query("INSERT INTO users (username, email, password_hash, token) VALUES($1, $2, $3, $4);")
      .bind(&username)
      .bind(&email)
      .bind(&password_hash)
      .bind(&token)
      .execute(executor(db)).await
      .unwrap();
    sqlx::query("UPDATE stats SET users = users + 1").execute(executor(db)).await.unwrap();
    Ok(token)
  }  

  /// Returns token
  pub async fn login(db: &mut Connection<Self>, email: &str, password: &str) -> Result<String, &'static str> {
    //Verify stuff
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
    //Perform query and check if user exists
    let row = sqlx::query!("SELECT password_hash, token FROM users WHERE email = $1", email)
      .fetch_optional(executor(db)).await
      .unwrap()
      .ok_or("User doesn't exist")?;
    //Check hash (assuming it's is in valid format)
    match argon2::verify_encoded(&row.password_hash, password.as_bytes()).unwrap() { 
      true => Ok(row.token),
      false => Err("Incorrect password")
    }
  }

  /// Returns user id
  pub async fn get_user_id_by_token(db: &mut Connection<Self>, token: &str) -> Option<i32> {
    let result = sqlx::query("SELECT user_id FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(executor(db)).await
      .unwrap();
    result.map(|row| row.get(0))
  }

  pub async fn get_user(db: &mut Connection<Self>, user_id: u32) -> Option<User> {
    let result = sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE user_id = $1")
      .bind(user_id as i32)
      .fetch_optional(executor(db)).await
      .unwrap();
    result.map(|row| User {
      user_id: row.get(0),
      username: row.get(1),
      email: row.get(2),
      password_hash: row.get(3),
      created_on: row.get(4),
      last_activity: row.get(5),
      user_role: row.get(6),
      token: row.get(7),
    })
  }

  pub async fn get_user_by_token(db: &mut Connection<Self>, token: &str) -> Option<User> {
    sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(executor(db)).await
      .unwrap()
      .map(|row| User {
        user_id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        password_hash: row.get(3),
        created_on: row.get(4),
        last_activity: row.get(5),
        user_role: row.get(6),
        token: row.get(7),
      })
  }

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

  pub async fn fetch_posts<'a>(db: &mut Connection<Self>, sort: PostSort, filter: PostFilter<'a>, page: u32, max_results_on_page: u32) -> Vec<TemplatePost> {
    //This is pretty inefficient but hey it works!
    //OFFSET is slow btw, https://use-the-index-luke.com/no-offset
    assert!(max_results_on_page > 0);
    let filter_options = match filter {
      PostFilter::None => (0, None, None, None),
      PostFilter::ByTopicId(id) => (1, Some(id), None, None),
      PostFilter::ByTopicName(name) => (2, None, Some(name), None),
      PostFilter::ByUserId(id) => (3, None, None, Some(id)),
      #[allow(unreachable_patterns)]
      _ => unimplemented!("Filter type not implemented")
    };
    sqlx::query_as!(TemplatePost, r#"
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

  pub async fn count_pages<'a>(db: &mut Connection<Self>, filter: PostFilter<'a>, results_per_page: u32) -> u32 {
    //! //HACK Turn this into it's own query, this is extemely slow
    let post_count = Self::fetch_posts(db, PostSort::ByDate(SortDirection::Descending), filter, 0, u32::MAX).await.len();
    div_up(post_count, results_per_page as usize) as u32
  }

  //stats
  pub async fn get_stats(db: &mut Connection<Self>) -> Stats {
    sqlx::query_as!(Stats, "SELECT * FROM stats").fetch_one(executor(db)).await.unwrap()
  }
}
