// This is our last exercise. Let's go down a more unstructured path!
// 这是我们的最后一个练习。让我们走一条更无结构化的道路！
// Try writing an **asynchronous REST API** to expose the functionality
// 尝试编写一个**异步 REST API**，以公开我们在这门课程中构建的
// of the ticket management system we built throughout the course.
// 票据管理系统的功能。
// It should expose endpoints to:
// 它应该暴露以下端点：
//  - Create a ticket
//  - 创建票据
//  - Retrieve ticket details
//  - 检索票据详情
//  - Patch a ticket
//  - 补丁更新票据
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// 使用 Rust 的包注册表 crates.io 来查找你所需的依赖
// (if any) to build this system.
// （如果有的话）来构建这个系统。
mod handlers;
pub mod model;
pub mod state;

use axum::Router;
use axum::routing::get;
use handlers::{create_ticket, delete_ticket, get_ticket, list_tickets, patch_ticket};
use state::AppState;
use tower_http::cors::CorsLayer;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/tickets", get(list_tickets).post(create_ticket))
        .route(
            "/tickets/{id}",
            get(get_ticket).patch(patch_ticket).delete(delete_ticket),
        )
        .layer(CorsLayer::permissive())
        .with_state(state)
}
