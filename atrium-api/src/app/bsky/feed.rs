// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed` namespace.
pub mod defs;
pub mod describe_feed_generator;
pub mod generator;
pub mod get_actor_feeds;
pub mod get_actor_likes;
pub mod get_author_feed;
pub mod get_feed;
pub mod get_feed_generator;
pub mod get_feed_generators;
pub mod get_feed_skeleton;
pub mod get_likes;
pub mod get_list_feed;
pub mod get_post_thread;
pub mod get_posts;
pub mod get_reposted_by;
pub mod get_suggested_feeds;
pub mod get_timeline;
pub mod like;
pub mod post;
pub mod repost;
pub mod search_posts;
pub mod threadgate;
#[derive(Debug)]
pub struct Generator;
impl crate::types::Collection for Generator {
    const NSID: &'static str = "app.bsky.feed.generator";
    type Record = generator::Record;
}
#[derive(Debug)]
pub struct Like;
impl crate::types::Collection for Like {
    const NSID: &'static str = "app.bsky.feed.like";
    type Record = like::Record;
}
#[derive(Debug)]
pub struct Post;
impl crate::types::Collection for Post {
    const NSID: &'static str = "app.bsky.feed.post";
    type Record = post::Record;
}
#[derive(Debug)]
pub struct Repost;
impl crate::types::Collection for Repost {
    const NSID: &'static str = "app.bsky.feed.repost";
    type Record = repost::Record;
}
#[derive(Debug)]
pub struct Threadgate;
impl crate::types::Collection for Threadgate {
    const NSID: &'static str = "app.bsky.feed.threadgate";
    type Record = threadgate::Record;
}
