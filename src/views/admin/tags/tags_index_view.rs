use leptos::*;

use crate::{
    components::admin::header_content::{ButtonProps, HeaderContent},
    services::admin::api::tags::get_tags,
    utils::add_class::add_class,
};

#[component]
pub fn AdminTagsView() -> impl IntoView {
    add_class("body", "tags");

    let tags = create_resource(|| (), |_| async { get_tags().await });

    view! {
        <HeaderContent
            title="Tags"
            button=ButtonProps {
                text: "Add new tags",
                url: "/rs-admin/tags/new",
            }
        />

        <Suspense fallback=move || view! { <p>"Loading tags..."</p> }>
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
                        <th scope="col">Name</th>
                        <th scope="col">Slug</th>
                        <th scope="col">Description</th>
                    </tr>
                </thead>
                <tbody>

                    {move || {
                        match tags.get() {
                            Some(Ok(tags)) => {
                                view! {
                                    <>
                                        {tags
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
                                                                "/rs-admin/tags/{}/edit",
                                                                post.id,
                                                            )>{&post.name}</a>
                                                        </td>
                                                        <td>{&post.slug}</td>
                                                        <td>
                                                            {<std::string::String as Clone>::clone(&post.description)}
                                                        </td>
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
                                        <td colspan="4">{"Failed to load tags: "} {err}</td>
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
