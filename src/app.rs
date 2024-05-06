mod term;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use term::Term;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/cmrnw.css"/>
        <Title text="cmrnw.com"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Term/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    view! { <h1>"Not Found"</h1> }
}
