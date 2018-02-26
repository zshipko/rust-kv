use lmdb;

pub struct RoCursor<'a>(pub lmdb::RoCursor<'a>);

impl <'a> RoCursor<'a> {

}

pub struct RwCursor<'a>(pub lmdb::RwCursor<'a>);

impl <'a> RwCursor<'a> {

}
