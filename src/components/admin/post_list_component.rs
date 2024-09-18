use crate::models::admin::posts_model::PostStruct;
use leptos::*;
use std::collections::HashSet;

#[component]
pub fn PostList(
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let toggle_post_selection = move |post_id: u32| {
        selected_posts.update(|selected| {
            if selected.contains(&post_id) {
                selected.remove(&post_id);
            } else {
                selected.insert(post_id);
            }
        });
    };

    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">
                        <input
                            class="form-check-input"
                            type="checkbox"
                            aria-label="Select all posts"
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                if checked {
                                    let all_ids = posts
                                        .get()
                                        .iter()
                                        .map(|post| post.id)
                                        .collect::<HashSet<u32>>();
                                    selected_posts.set(all_ids);
                                } else {
                                    selected_posts.set(HashSet::new());
                                }
                            }
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
                    let post_list = posts.get();
                    if !post_list.is_empty() {
                        view! {
                            <>
                                {post_list
                                    .into_iter()
                                    .map(|post| {
                                        let is_checked = selected_posts.get().contains(&post.id);
                                        view! {
                                            <tr>
                                                <td>
                                                    <input
                                                        class="form-check-input"
                                                        type="checkbox"
                                                        on:change=move |_| toggle_post_selection(post.id)
                                                        prop:checked=is_checked
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
                    } else {
                        view! {
                            <>
                                <tr>
                                    <td colspan="5">"No posts available"</td>
                                </tr>
                            </>
                        }
                    }
                }}

            </tbody>
        </table>
    }
}
