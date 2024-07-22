use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: u32,
    title: String,
    content: String,
    author_id: u32,
}

#[component]
pub fn Posts() -> impl IntoView {
    create_effect(move |_| {
        let body = document().body().expect("document should have a body");
        body.class_list()
            .add_1("posts")
            .expect("could not add class");
        on_cleanup(move || {
            body.class_list()
                .remove_1("posts")
                .expect("could not remove class");
        });
    });

    let posts = create_resource(|| (), |_| async { fetch_posts().await });

    view! {
        <Title text="Posts"/>
        <h1>"Posts"</h1>
        <a class="btn btn-outline-primary" href="/rs-admin/posts/new" role="button">Add new post</a>

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
                    {
                        move || {
                            match posts.get() {
                                Some(Ok(posts)) => view! {
                                    <>
                                    {posts.iter().map(|post| view! {
                                        <tr>
                                            <td>{post.id}</td>
                                            <td>{&post.title}</td>
                                            <td>{&post.content}</td>
                                            <td>{post.author_id}</td>
                                        </tr>
                                    }).collect::<Vec<_>>()}
                                    </>
                                }.into_view(),
                                Some(Err(err)) => view! {
                                    <tr><td colspan="4">{"Failed to load posts: "}{err}</td></tr>
                                }.into_view(),
                                None => view! {
                                    <tr><td colspan="4">"Loading..."</td></tr>
                                }.into_view(),
                            }
                        }
                    }
                </tbody>
            </table>
        </Suspense>
    }
}

async fn fetch_posts() -> Result<Vec<Post>, String> {
    use reqwest::Client;
    let client = Client::new();

    let resp = client
        .get("http://127.0.0.1:8080/api/v1/posts")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let posts = resp.json::<Vec<Post>>().await.map_err(|e| e.to_string())?;
    Ok(posts)
}
