//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "objects"))]
#[component]
pub fn TrolleySuitcase(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M88,224a16,16,0,1,1-16-16A16,16,0,0,1,88,224Zm128-16a16,16,0,1,0,16,16A16,16,0,0,0,216,208Zm24-32H56V75.31A15.86,15.86,0,0,0,51.31,64L29.66,42.34A8,8,0,0,0,18.34,53.66L40,75.31V176H32a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM72,144V72A16,16,0,0,1,88,56h32V40a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V56h32a16,16,0,0,1,16,16v72a16,16,0,0,1-16,16H88A16,16,0,0,1,72,144Zm64-88h32V40H136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,72v72a8,8,0,0,1-8,8H88a8,8,0,0,1-8-8V72a8,8,0,0,1,8-8H216A8,8,0,0,1,224,72Z"
        opacity="0.2"
    ></path>
    <path d="M88,224a16,16,0,1,1-16-16A16,16,0,0,1,88,224Zm128-16a16,16,0,1,0,16,16A16,16,0,0,0,216,208Zm24-32H56V75.31A15.86,15.86,0,0,0,51.31,64L29.66,42.34A8,8,0,0,0,18.34,53.66L40,75.31V176H32a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM72,144V72A16,16,0,0,1,88,56h32V40a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V56h32a16,16,0,0,1,16,16v72a16,16,0,0,1-16,16H88A16,16,0,0,1,72,144Zm64-88h32V40H136ZM88,144H216V72H88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M88,156H216a12,12,0,0,0,12-12V72a12,12,0,0,0-12-12H180V40a12,12,0,0,0-12-12H136a12,12,0,0,0-12,12V60H88A12,12,0,0,0,76,72v72A12,12,0,0,0,88,156ZM132,40a4,4,0,0,1,4-4h32a4,4,0,0,1,4,4V60H132ZM84,72a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4v72a4,4,0,0,1-4,4H88a4,4,0,0,1-4-4Zm0,152a12,12,0,1,1-12-12A12,12,0,0,1,84,224Zm144,0a12,12,0,1,1-12-12A12,12,0,0,1,228,224Zm16-40a4,4,0,0,1-4,4H32a4,4,0,0,1,0-8H44V75.31a4,4,0,0,0-1.17-2.83L21.17,50.83a4,4,0,0,1,5.66-5.66L48.49,66.83A12,12,0,0,1,52,75.31V180H240A4,4,0,0,1,244,184Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M96,156H216a20,20,0,0,0,20-20V76a20,20,0,0,0-20-20H192V40a20,20,0,0,0-20-20H140a20,20,0,0,0-20,20V56H96A20,20,0,0,0,76,76v60A20,20,0,0,0,96,156ZM144,44h24V56H144ZM100,80H212v52H100ZM92,224a20,20,0,1,1-20-20A20,20,0,0,1,92,224Zm144,0a20,20,0,1,1-20-20A20,20,0,0,1,236,224Zm16-44a12,12,0,0,1-12,12H32a12,12,0,0,1,0-24h4V77L15.51,56.49a12,12,0,0,1,17-17L54.14,61.17A19.86,19.86,0,0,1,60,75.31V168H240A12,12,0,0,1,252,180Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M88,158H216a14,14,0,0,0,14-14V72a14,14,0,0,0-14-14H182V40a14,14,0,0,0-14-14H136a14,14,0,0,0-14,14V58H88A14,14,0,0,0,74,72v72A14,14,0,0,0,88,158ZM134,40a2,2,0,0,1,2-2h32a2,2,0,0,1,2,2V58H134ZM86,72a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2v72a2,2,0,0,1-2,2H88a2,2,0,0,1-2-2Zm0,152a14,14,0,1,1-14-14A14,14,0,0,1,86,224Zm144,0a14,14,0,1,1-14-14A14,14,0,0,1,230,224Zm16-40a6,6,0,0,1-6,6H32a6,6,0,0,1,0-12H42V75.31a2,2,0,0,0-.59-1.41L19.76,52.24a6,6,0,1,1,8.48-8.48L49.9,65.41a13.94,13.94,0,0,1,4.1,9.9V178H240A6,6,0,0,1,246,184Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,224a16,16,0,1,1-16-16A16,16,0,0,1,88,224Zm128-16a16,16,0,1,0,16,16A16,16,0,0,0,216,208Zm24-32H56V75.31A15.86,15.86,0,0,0,51.31,64L29.66,42.34A8,8,0,0,0,18.34,53.66L40,75.31V176H32a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM72,144V72A16,16,0,0,1,88,56h32V40a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V56h32a16,16,0,0,1,16,16v72a16,16,0,0,1-16,16H88A16,16,0,0,1,72,144Zm64-88h32V40H136ZM88,144H216V72H88Z"></path>
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
