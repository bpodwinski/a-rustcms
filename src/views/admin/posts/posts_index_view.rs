use leptos::*;
use leptos_meta::*;

use crate::{services::admin::api::posts::fetch_posts, utils::add_class::add_class};

#[component]
pub fn Posts() -> impl IntoView {
    add_class("body", "posts");

    let posts = create_resource(|| (), |_| async { fetch_posts().await });

    view! {
        <Title text="Posts"/>
        <h1>"Posts"</h1>
        <a class="btn btn-outline-primary" href="/rs-admin/posts/new" role="button">
            Add new post
        </a>

        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <table class="table">
                <thead>
                    <tr>
                        <th scope="col">#</th>
                        <th scope="col">Title</th>
                        <th scope="col">Content</th>
                        <th scope="col">Author</th>
                    </tr>
                </thead>
                <tbody>

                    {move || {
                        match posts.get() {
                            Some(Ok(posts)) => {
                                view! {
                                    <>
                                        {posts
                                            .iter()
                                            .map(|post| {
                                                view! {
                                                    <tr>
                                                        <td>{post.id}</td>
                                                        <td>{&post.title}</td>
                                                        <td>{&post.content}</td>
                                                        <td>{post.author_id}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </>
                                }
                                    .into_view()
                            }
                            Some(Err(err)) => {
                                view! {
                                    <tr>
                                        <td colspan="4">{"Failed to load posts: "} {err}</td>
                                    </tr>
                                }
                                    .into_view()
                            }
                            None => {
                                view! {
                                    <tr>
                                        <td colspan="4">"Loading..."</td>
                                    </tr>
                                }
                                    .into_view()
                            }
                        }
                    }}

                </tbody>
            </table>
        </Suspense>
    }
}
