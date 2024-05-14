//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design", feature ="editor"))]
#[component]
pub fn Perspective(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M248,136a8,8,0,0,1-8,8H224v64a16,16,0,0,1-16,16,16.47,16.47,0,0,1-2.87-.26l-160-29.09A16,16,0,0,1,32,178.91V144H16a8,8,0,0,1,0-16H240A8,8,0,0,1,248,136ZM36,112H220a4,4,0,0,0,4-4V48.42a16.48,16.48,0,0,0-4.07-11.08,16,16,0,0,0-14.79-5.08l-160,29.09A16,16,0,0,0,32,77.09V108A4,4,0,0,0,36,112Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,48V208a8,8,0,0,1-9.43,7.87l-160-29.09A8,8,0,0,1,40,178.91V77.09a8,8,0,0,1,6.57-7.87l160-29.09A8,8,0,0,1,216,48Z"
        opacity="0.2"
    ></path>
    <path d="M240,120H224V48a16,16,0,0,0-18.86-15.74l-160,29.09A16,16,0,0,0,32,77.09V120H16a8,8,0,0,0,0,16H32v42.91a16,16,0,0,0,13.14,15.74l160,29.09A16.47,16.47,0,0,0,208,224a16,16,0,0,0,16-16V136h16a8,8,0,0,0,0-16ZM48,77.09,208,48v72H48ZM208,208,48,178.91V136H208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,124H220V48a12,12,0,0,0-14.15-11.81l-160,29.1A12,12,0,0,0,36,77.09V124H16a4,4,0,0,0,0,8H36v46.91a12,12,0,0,0,9.85,11.8l160,29.09a11.28,11.28,0,0,0,2.16.2,12,12,0,0,0,12-12V132h20a4,4,0,0,0,0-8ZM44,77.09a4,4,0,0,1,3.28-3.93l160-29.09A4,4,0,0,1,212,48v76H44ZM212,208a4,4,0,0,1-4.72,3.93l-160-29.09A4,4,0,0,1,44,178.91V132H212Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,116H228V48a20,20,0,0,0-23.58-19.67l-160,29.09A20,20,0,0,0,28,77.09V116H16a12,12,0,0,0,0,24H28v38.91a20,20,0,0,0,16.42,19.67l160,29.09A20,20,0,0,0,228,208V140h12a12,12,0,0,0,0-24ZM52,80.43,204,52.8V116H52ZM204,203.2,52,175.57V140H204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,122H222V48a14,14,0,0,0-16.5-13.77L45.5,63.32A14,14,0,0,0,34,77.09V122H16a6,6,0,0,0,0,12H34v44.91a14,14,0,0,0,11.5,13.77l160,29.09A14.2,14.2,0,0,0,208,222a14,14,0,0,0,14-14V134h18a6,6,0,0,0,0-12ZM46,77.09a2,2,0,0,1,1.64-2l160-29.1.37,0a2,2,0,0,1,2,2v74H46ZM210,208a2,2,0,0,1-.72,1.53,2,2,0,0,1-1.64.44l-160-29.1a2,2,0,0,1-1.64-2V134H210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,120H224V48a16,16,0,0,0-18.86-15.74l-160,29.09A16,16,0,0,0,32,77.09V120H16a8,8,0,0,0,0,16H32v42.91a16,16,0,0,0,13.14,15.74l160,29.09A16.47,16.47,0,0,0,208,224a16,16,0,0,0,16-16V136h16a8,8,0,0,0,0-16ZM48,77.09,208,48v72H48ZM208,208,48,178.91V136H208Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}