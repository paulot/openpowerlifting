//! Suite of tests for the search function on the compiled database.

extern crate server;

use server::pages::api_search::*;
use server::pages::selection::*;

mod common;

/// Checks that basic rankings search functionality works.
#[test]
fn basic_rankings_search() {
    let db = common::db();
    let cache = db.get_static_cache();
    let selection = Selection::new_default();

    // Perform the search.
    let res = search_rankings(&db, &selection, 0, "Sean Stangl");
    let row = res.next_index.unwrap();

    // Check that the result is for the specified lifter.
    let list = cache.get_full_sorted_uniqued(&selection, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}