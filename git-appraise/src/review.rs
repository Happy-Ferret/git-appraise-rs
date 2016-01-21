use refs;

use std::str::FromStr;
use git2::{ Repository, Note };
use super::{ Error, Result, Oid, Request, CIStatuses };
use request::{ ByTimestamp };

pub struct Review<'r> {
  git: &'r Repository,
  id: Oid,
  request: Request,
}

impl<'r> Review<'r> {
  pub fn for_commit(git: &'r Repository, id: Oid) -> Result<Review<'r>> {
    git.find_note(refs::REVIEWS, id)
      .map_err(From::from)
      .and_then(|note| Review::from_note(git, id, note))
  }

  pub fn from_note(git: &'r Repository, id: Oid, note: Note<'r>) -> Result<Review<'r>> {
    note.message()
      .ok_or(Error::NotFound)
      .and_then(|message|
        message.lines()
          .filter_map(|line| Request::from_str(line).ok())
          .map(|req| ByTimestamp(req))
          .max()
          .map(|wrapper| wrapper.0)
          .ok_or(Error::NotFound)
          .map(|req| Review::from_request(git, id, req)))
  }

  pub fn id(&self) -> Oid {
    self.id
  }

  pub fn request(&self) -> &Request {
    &self.request
  }

  pub fn ci_statuses(&self) -> CIStatuses {
    CIStatuses::for_commit(&self.git, self.id)
  }

  fn from_request(git: &'r Repository, id: Oid, req: Request) -> Review<'r> {
    Review {
      git: git,
      id: id,
      request: req,
    }
  }
}
