use chrono::{offset::Local, DateTime};
use futures::{future, Future};
use http;
use hyper::{header, Body, Method, StatusCode};

use std::{fs, io, net::SocketAddr, path::Path};

use crate::base::ctx::ctxs;
use crate::base::{response, Request, Response};
use crate::config::Route;
use crate::consts::CONTENT_TYPE_HTML;
use crate::service::GlobalState;
use crate::views::{EntryMetadata, EntryOrder};

pub fn index_handler(
    route: &Route,
    reqpath: &str,
    path: &Path,
    meta: &fs::Metadata,
    req: Request,
    addr: &SocketAddr,
    state: GlobalState,
) -> impl Future<Output = Result<Response, http::Error>> {
    match index_handler2(route, reqpath, path, meta, req, addr, state) {
        Ok(resp) => future::ready(resp),
        Err(e) => {
            error!("index_handler2 faield: {:?}", e);
            future::ready(response().status(500).body(Body::empty()))
        }
    }
}

pub fn index_handler2(
    route: &Route,
    reqpath: &str,
    path: &Path,
    meta: &fs::Metadata,
    req: Request,
    addr: &SocketAddr,
    state: ctxs::State,
) -> io::Result<Result<Response, http::Error>> {
    let mut resp = response();
    let cache_secs = state.config().cache_secs;

    let entry_order = EntryOrder::new(req.uri().query());

    if cache_secs > 0 {
        let last_modified = meta.modified()?;
        let last_modified: DateTime<Local> = last_modified.into();
        let http_last_modified = last_modified.to_rfc2822();

        // W/"80-5d564a70.3797f8b1@Empty"
        let etag = format!(
            "W/\"{:x}-{:x}.{:x}@{}\"",
            meta.len(),
            last_modified.timestamp_millis(),
            last_modified.timestamp_subsec_nanos(),
            entry_order
        );

        let http_etag = req
            .headers()
            .get(header::IF_NONE_MATCH)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();

        let http_if_modified_since = req.headers().get(header::IF_MODIFIED_SINCE);
        let if_modified_since = http_if_modified_since
            .and_then(|v| v.to_str().ok())
            .and_then(|v| DateTime::parse_from_rfc2822(v).ok())
            .map(|v| v.with_timezone(&Local));

        if etag.as_str() == http_etag
            && if_modified_since
                .map(|v| v.timestamp() == last_modified.timestamp())
                .unwrap_or(true)
        {
            // 304
            return Ok(resp.status(StatusCode::NOT_MODIFIED).body(Body::empty()));
        }
        resp = resp.header(header::CACHE_CONTROL, format!("public, max-age={}", cache_secs).as_str());
        resp = resp.header(header::LAST_MODIFIED, http_last_modified);
        resp = resp.header(header::ETAG, etag);
    }

    let html = render_html(addr, reqpath, path, &req, &entry_order, route)?;
    resp = resp.header(header::CONTENT_TYPE, CONTENT_TYPE_HTML);
    resp = resp.header(header::CONTENT_LENGTH, html.len());

    match *req.method() {
        Method::GET => Ok(resp.body(html.into())),
        // 204： curl -Lv -X HEAD "0.0.0.0:8000/src/main.rs"
        Method::HEAD => Ok(resp.status(StatusCode::NO_CONTENT).body(Body::empty())),
        _ => unreachable!(),
    }
}

use crate::tools::url_for_parent;
use crate::views::IndexTemplate;
use askama::Template;

pub fn render_html(
    remote_addr: &SocketAddr,
    title: &str,
    index: &Path,
    req: &Request,
    order: &EntryOrder,
    config: &Route,
) -> io::Result<String> {
    let metadatas = EntryMetadata::read_dir(index, config.follow_links, config.show_hider, order)?;
    let next_order = order.next();
    let parent = url_for_parent(req.uri().path());

    let template = IndexTemplate::new(
        title,
        title,
        &parent,
        &remote_addr,
        next_order,
        &metadatas,
        config.upload,
        config.mkdir,
    );
    let html = template.render().unwrap();

    Ok(html)
}
