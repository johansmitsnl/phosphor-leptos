/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Baby(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M134.16,24.1a4,4,0,0,0-3.56,1.81C120.3,41.48,120,55.79,120,56a8,8,0,0,0,9.68,7.79A8.24,8.24,0,0,0,136,55.68,8,8,0,0,1,144.8,48a8.14,8.14,0,0,1,7.2,8.23,24,24,0,0,1-48-.27c0-.63.09-10.78,5.44-24a4,4,0,0,0-4.59-5.39A104.16,104.16,0,0,0,24.07,131.66C26,186.72,71.23,231,126.32,231.9a104,104,0,0,0,7.84-207.8ZM80,127.91a12,12,0,1,1,12,12A12,12,0,0,1,80,127.91Zm80.27,54.77a61,61,0,0,1-64.54,0,8,8,0,0,1,8.54-13.54,45,45,0,0,0,47.46,0,8,8,0,0,1,8.54,13.54ZM164,139.91a12,12,0,1,1,12-12A12,12,0,0,1,164,139.91Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"/><path d="M92,140a12,12,0,1,1,12-12A12,12,0,0,1,92,140Zm72-24a12,12,0,1,0,12,12A12,12,0,0,0,164,116Zm-12.27,45.23a45,45,0,0,1-47.46,0,8,8,0,0,0-8.54,13.54,61,61,0,0,0,64.54,0,8,8,0,0,0-8.54-13.54ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88.11,88.11,0,0,0-84.09-87.91C120.32,56.38,120,71.88,120,72a8,8,0,0,0,16,0,8,8,0,0,1,16,0,24,24,0,0,1-48,0c0-.73.13-14.3,8.46-30.63A88,88,0,1,0,216,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M92,136a8,8,0,1,1,8-8A8,8,0,0,1,92,136Zm72-16a8,8,0,1,0,8,8A8,8,0,0,0,164,120Zm-10.13,44.62a49,49,0,0,1-51.74,0,4,4,0,0,0-4.26,6.76,57,57,0,0,0,60.26,0,4,4,0,1,0-4.26-6.76ZM228,128A100,100,0,1,1,128,28,100.11,100.11,0,0,1,228,128Zm-8,0a92.11,92.11,0,0,0-90.06-92C116.26,54.07,116,71.83,116,72a12,12,0,0,0,24,0,4,4,0,0,1,8,0,20,20,0,0,1-40,0c0-.78.16-17.31,12-35.64A92,92,0,1,0,220,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M92,144a16,16,0,1,1,16-16A16,16,0,0,1,92,144Zm72-32a16,16,0,1,0,16,16A16,16,0,0,0,164,112Zm-14.4,49.85a41,41,0,0,1-43.2,0,12,12,0,1,0-12.8,20.3,65,65,0,0,0,68.8,0,12,12,0,1,0-12.8-20.3ZM236,128A108,108,0,1,1,128,20,108.12,108.12,0,0,1,236,128Zm-24,0a84.1,84.1,0,0,0-78.08-83.77c-9.31,14.09-9.89,27-9.92,27.83a4,4,0,0,0,8-.06,12,12,0,0,1,24,0,28,28,0,0,1-56,0c0-.65.1-11.19,5.52-24.92A84,84,0,1,0,212,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M92,138a10,10,0,1,1,10-10A10,10,0,0,1,92,138Zm72-20a10,10,0,1,0,10,10A10,10,0,0,0,164,118Zm-11.2,44.92a47,47,0,0,1-49.6,0,6,6,0,0,0-6.4,10.16,59,59,0,0,0,62.4,0,6,6,0,1,0-6.4-10.16ZM230,128A102,102,0,1,1,128,26,102.12,102.12,0,0,1,230,128Zm-12,0a90.11,90.11,0,0,0-87.07-89.95C118.3,55.23,118,71.85,118,72a10,10,0,0,0,20,0,6,6,0,0,1,12,0,22,22,0,0,1-44,0c0-.75.15-15.82,10.14-33.22A90,90,0,1,0,218,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M92,140a12,12,0,1,1,12-12A12,12,0,0,1,92,140Zm72-24a12,12,0,1,0,12,12A12,12,0,0,0,164,116Zm-12.27,45.23a45,45,0,0,1-47.46,0,8,8,0,0,0-8.54,13.54,61,61,0,0,0,64.54,0,8,8,0,0,0-8.54-13.54ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88.11,88.11,0,0,0-84.09-87.91C120.32,56.38,120,71.88,120,72a8,8,0,0,0,16,0,8,8,0,0,1,16,0,24,24,0,0,1-48,0c0-.73.13-14.3,8.46-30.63A88,88,0,1,0,216,128Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}