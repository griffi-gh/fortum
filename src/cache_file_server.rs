use std::path::PathBuf;
use rocket::http::Method;
use rocket::{Request, Response, Data, Route};
use rocket::http::uri::{Segments, fmt::Path};
use rocket::route::{Handler, Outcome};
use rocket::response::{self, Responder};
use rocket::fs::NamedFile;

#[allow(unused)]
pub struct CachedFile {
  file: NamedFile,
  age: usize
}
#[allow(unused)]
impl CachedFile {
  #[inline]
  pub fn new(file: NamedFile, age: usize) -> Self {
    Self { file, age }
  }
  #[inline]
  pub async fn open(path: PathBuf, age: usize) -> Option<Self> {
    Some(Self { 
      file: NamedFile::open(path).await.ok()?,
      age
    })
  }
}
impl<'r, 'o: 'r> Responder<'r, 'o> for CachedFile {
  #[inline]
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
    let mut builder = Response::build_from(self.file.respond_to(req)?);
    //Don't cache on debug builds
    #[cfg(not(debug_assertions))]{ 
      builder.raw_header("Cache-control", format!("max-age={}", self.age)); //Allocates, potential perf issue 
    }
    builder.ok()
  }
}

// Some code stolen from 
// https://github.com/SergioBenitez/Rocket/blob/42a0fb8afe0ad94024cbb2a131390b53b99937c1/core/lib/src/fs/server.rs
#[derive(Default, Clone, Copy)]
pub struct CacheFileServerOptions {
  pub allow_dotfiles: bool
}

const DEFAULT_RANK: isize = 10;

#[derive(Clone)]
pub struct CacheFileServer {
  pub options: CacheFileServerOptions,
  pub root: PathBuf,
  pub age: usize,
  pub rank: isize,
}
#[allow(unused)]
impl CacheFileServer {
  #[inline]
  pub fn new(root: impl Into<PathBuf>, age: usize) -> Self {
    Self::with_rank(root, age, DEFAULT_RANK)
  }
  #[inline]
  pub fn with_rank(root: impl Into<PathBuf>, age: usize, rank: isize) -> Self {
    Self { 
      options: CacheFileServerOptions::default(),
      root: root.into(),
      age,
      rank,
    }
  }
}
#[rocket::async_trait]
impl Handler for CacheFileServer {
  #[inline] 
  async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
    let path = req.segments::<Segments<'_, Path>>(0..).ok()
      .and_then(|segments| segments.to_path_buf(self.options.allow_dotfiles).ok())
      .map(|path| self.root.join(path));
    match path {
      None => Outcome::forward(data),
      Some(path) if path.is_dir() => Outcome::forward(data),
      Some(path) => Outcome::from_or_forward(req, data, CachedFile::open(path, self.age).await),
    }
  }
}

impl From<CacheFileServer> for Vec<Route> {
  fn from(server: CacheFileServer) -> Self {
    let source = rocket::figment::Source::File(server.root.clone());
    let mut route = Route::ranked(server.rank, Method::Get, "/<path..>", server);
    route.name = Some(format!("CacheFileServer: {}", source).into());
    vec![route]
  }
}
