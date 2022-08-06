pub struct PostSubmitData {
  
}

#[post("/submit")]
pub async fn submit(data: Form<RegisterData>, db: Connection<MainDatabase>, auth: Authentication) -> Redirect {
  
}
