/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn GitMerge(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,112a32.06,32.06,0,0,0-31,24H152a40.19,40.19,0,0,1-32-16L93.69,84.92A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V104l19.2,25.6A56.26,56.26,0,0,0,152,152h25a32,32,0,1,0,31-40ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,0,1-16-16V144a16,16,0,1,1,16,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M104,56A24,24,0,1,1,80,32,24,24,0,0,1,104,56Z" opacity="0.2"/><path d="M208,112a32.06,32.06,0,0,0-31,24H152a40.19,40.19,0,0,1-32-16L93.69,84.92A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V104l19.2,25.6A56.26,56.26,0,0,0,152,152h25a32,32,0,1,0,31-40ZM64,56A16,16,0,1,1,80,72,16,16,0,0,1,64,56ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,1,1,16-16A16,16,0,0,1,208,160Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,116a28,28,0,0,0-27.71,24H152a44.2,44.2,0,0,1-35.2-17.6L87.28,83A28,28,0,1,0,76,83.71v88.58a28,28,0,1,0,8,0V92l26.4,35.2A52.26,52.26,0,0,0,152,148h28.29A28,28,0,1,0,208,116ZM60,56A20,20,0,1,1,80,76,20,20,0,0,1,60,56Zm40,144a20,20,0,1,1-20-20A20,20,0,0,1,100,200Zm108-36a20,20,0,1,1,20-20A20,20,0,0,1,208,164Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,108a36.07,36.07,0,0,0-33.94,24H152a36.16,36.16,0,0,1-28.8-14.4L99.62,86.16A36,36,0,1,0,68,89.94v76.12a36,36,0,1,0,24,0V116l12,16a60.27,60.27,0,0,0,48,24h22.06A36,36,0,1,0,208,108ZM80,44A12,12,0,1,1,68,56,12,12,0,0,1,80,44Zm0,168a12,12,0,1,1,12-12A12,12,0,0,1,80,212Zm128-56a12,12,0,1,1,12-12A12,12,0,0,1,208,156Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,114a30.05,30.05,0,0,0-29.4,24H152a42.2,42.2,0,0,1-33.6-16.8L90.56,84.08A30,30,0,1,0,74,85.4v85.2a30,30,0,1,0,12,0V98l22.8,30.4A54.26,54.26,0,0,0,152,150h26.6A30,30,0,1,0,208,114ZM62,56A18,18,0,1,1,80,74,18,18,0,0,1,62,56ZM98,200a18,18,0,1,1-18-18A18,18,0,0,1,98,200Zm110-38a18,18,0,1,1,18-18A18,18,0,0,1,208,162Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,112a32.06,32.06,0,0,0-31,24H152a40.19,40.19,0,0,1-32-16L93.69,84.92A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V104l19.2,25.6A56.26,56.26,0,0,0,152,152h25a32,32,0,1,0,31-40ZM64,56A16,16,0,1,1,80,72,16,16,0,0,1,64,56ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,1,1,16-16A16,16,0,0,1,208,160Z"/> }.into_view()
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