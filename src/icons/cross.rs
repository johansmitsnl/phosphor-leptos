//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design", feature ="communication"))]
#[component]
pub fn Cross(
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
                <path d="M216,92v24a16,16,0,0,1-16,16H156v92a16,16,0,0,1-16,16H116a16,16,0,0,1-16-16V132H56a16,16,0,0,1-16-16V92A16,16,0,0,1,56,76h44V32a16,16,0,0,1,16-16h24a16,16,0,0,1,16,16V76h44A16,16,0,0,1,216,92Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,88v32a8,8,0,0,1-8,8H152v96a8,8,0,0,1-8,8H112a8,8,0,0,1-8-8V128H56a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8h48V32a8,8,0,0,1,8-8h32a8,8,0,0,1,8,8V80h48A8,8,0,0,1,208,88Z"
        opacity="0.2"
    ></path>
    <path d="M200,72H160V32a16,16,0,0,0-16-16H112A16,16,0,0,0,96,32V72H56A16,16,0,0,0,40,88v32a16,16,0,0,0,16,16H96v88a16,16,0,0,0,16,16h32a16,16,0,0,0,16-16V136h40a16,16,0,0,0,16-16V88A16,16,0,0,0,200,72Zm0,48H152a8,8,0,0,0-8,8v96H112V128a8,8,0,0,0-8-8H56V88h48a8,8,0,0,0,8-8V32h32V80a8,8,0,0,0,8,8h48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,76H156V32a12,12,0,0,0-12-12H112a12,12,0,0,0-12,12V76H56A12,12,0,0,0,44,88v32a12,12,0,0,0,12,12h44v92a12,12,0,0,0,12,12h32a12,12,0,0,0,12-12V132h44a12,12,0,0,0,12-12V88A12,12,0,0,0,200,76Zm4,44a4,4,0,0,1-4,4H152a4,4,0,0,0-4,4v96a4,4,0,0,1-4,4H112a4,4,0,0,1-4-4V128a4,4,0,0,0-4-4H56a4,4,0,0,1-4-4V88a4,4,0,0,1,4-4h48a4,4,0,0,0,4-4V32a4,4,0,0,1,4-4h32a4,4,0,0,1,4,4V80a4,4,0,0,0,4,4h48a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200,68H164V32a20,20,0,0,0-20-20H112A20,20,0,0,0,92,32V68H56A20,20,0,0,0,36,88v32a20,20,0,0,0,20,20H92v84a20,20,0,0,0,20,20h32a20,20,0,0,0,20-20V140h36a20,20,0,0,0,20-20V88A20,20,0,0,0,200,68Zm-4,48H152a12,12,0,0,0-12,12v92H116V128a12,12,0,0,0-12-12H60V92h44a12,12,0,0,0,12-12V36h24V80a12,12,0,0,0,12,12h44Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200,74H158V32a14,14,0,0,0-14-14H112A14,14,0,0,0,98,32V74H56A14,14,0,0,0,42,88v32a14,14,0,0,0,14,14H98v90a14,14,0,0,0,14,14h32a14,14,0,0,0,14-14V134h42a14,14,0,0,0,14-14V88A14,14,0,0,0,200,74Zm2,46a2,2,0,0,1-2,2H152a6,6,0,0,0-6,6v96a2,2,0,0,1-2,2H112a2,2,0,0,1-2-2V128a6,6,0,0,0-6-6H56a2,2,0,0,1-2-2V88a2,2,0,0,1,2-2h48a6,6,0,0,0,6-6V32a2,2,0,0,1,2-2h32a2,2,0,0,1,2,2V80a6,6,0,0,0,6,6h48a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,72H160V32a16,16,0,0,0-16-16H112A16,16,0,0,0,96,32V72H56A16,16,0,0,0,40,88v32a16,16,0,0,0,16,16H96v88a16,16,0,0,0,16,16h32a16,16,0,0,0,16-16V136h40a16,16,0,0,0,16-16V88A16,16,0,0,0,200,72Zm0,48H152a8,8,0,0,0-8,8v96H112V128a8,8,0,0,0-8-8H56V88h48a8,8,0,0,0,8-8V32h32V80a8,8,0,0,0,8,8h48Z"></path>
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