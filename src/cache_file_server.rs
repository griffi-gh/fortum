use std::path::{PathBuf, Path};
use rocket::{Request, fs::NamedFile, response::{self, Responder}, Response};
pub struct CachedFile<const AGE: usize>(NamedFile);
impl<'r, 'o: 'r, const AGE: usize> Responder<'r, 'o> for CachedFile<AGE> {
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
    let mut builder = Response::build_from(self.0.respond_to(req)?);
    #[cfg(not(debug_assertions))] //Don't cache on debug builds
    builder.raw_header("Cache-control", format!("max-age={}", AGE)); //Allocates, potential perf issue 
    builder.ok()
  }
}

#[get("/<file..>")]
pub async fn files_long(file: PathBuf) -> Option<CachedFile<1209600>> { // 2 weeks (14*24*60*60 seconds)
  NamedFile::open(Path::new("static/").join(file)).await.ok().map(|nf| CachedFile(nf))
}
#[get("/<file..>")]
pub async fn files_short(file: PathBuf) -> Option<CachedFile<300>> { // 5 minutes (5*60 seconds)
  NamedFile::open(Path::new("static/").join(file)).await.ok().map(|nf| CachedFile(nf))
}
