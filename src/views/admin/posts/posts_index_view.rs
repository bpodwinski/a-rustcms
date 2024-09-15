use leptos::*;

use crate::components::admin::header_content_component::{
    ButtonProps, HeaderContent,
};
use crate::components::front::loading_component::LoadingComponent;
use crate::{
    services::admin::api::posts_api::get_posts,
    utils::add_class_util::add_class,
};

#[component]
pub fn AdminPostsView() -> impl IntoView {
    add_class("body", "posts");

    let posts = create_resource(|| (), |_| async { get_posts().await });

    view! {
        <HeaderContent
            title="Posts"
            button=ButtonProps {
                text: "Add new post",
                url: "/rs-admin/posts/new",
            }
        />

        <Suspense fallback=move || view! { <LoadingComponent/> }>
            <table class="table">
                <thead>
                    <tr>
                        <th scope="col">
                            <input
                                class="form-check-input"
                                type="checkbox"
                                value=""
                                id="flexCheckDefault"
                            />
                        </th>
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
                                                        <td>
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                value=""
                                                                id="flexCheckDefault"
                                                            />
                                                        </td>
                                                        <td>{post.id}</td>
                                                        <td>
                                                            <a href=format!(
                                                                "/rs-admin/posts/{}/edit",
                                                                post.id,
                                                            )>{&post.title}</a>
                                                        </td>
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
