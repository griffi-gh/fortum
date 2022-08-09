use std::path::{PathBuf, Path};
use rocket::{Request, fs::NamedFile, response::{self, Responder}, Response};

pub struct CachedFile(NamedFile);
impl<'r, 'o: 'r> Responder<'r, 'o> for CachedFile {
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
    Response::build_from(self.0.respond_to(req)?)
      .raw_header("Cache-control", "max-age=1209600") // 2 weeks (2*7*24*60*60)
      .ok()
  }
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<CachedFile> {
  NamedFile::open(Path::new("static/").join(file)).await.ok().map(|nf| CachedFile(nf))
}
