//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="map"))]
#[component]
pub fn Warehouse(
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
                <path d="M240,184h-8V57.9l9.67-2.08a8,8,0,1,0-3.35-15.64l-224,48A8,8,0,0,0,16,104a8.16,8.16,0,0,0,1.69-.18L24,102.47V184H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Zm-56,0H72V168H184Zm0-32H72V136H184Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M184,128v64H72V128Z" opacity="0.2"></path>
    <path d="M240,184h-8V57.9l9.67-2.08a8,8,0,1,0-3.35-15.64l-224,48A8,8,0,0,0,16,104a8.16,8.16,0,0,0,1.69-.18L24,102.47V184H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM40,99,216,61.33V184H192V128a8,8,0,0,0-8-8H72a8,8,0,0,0-8,8v56H40Zm136,53H80V136h96ZM80,168h96v16H80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,188H228V54.66l12.84-2.75a4,4,0,1,0-1.68-7.82l-224,48A4,4,0,0,0,16,100a4.07,4.07,0,0,0,.84-.09L28,97.52V188H16a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8ZM36,95.81,220,56.38V188H188V128a4,4,0,0,0-4-4H72a4,4,0,0,0-4,4v60H36ZM180,156H76V132H180ZM76,164H180v24H76Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,180h-4V61.13l6.51-1.39a12,12,0,1,0-5-23.47l-224,48A12,12,0,0,0,16,108a12.21,12.21,0,0,0,2.53-.26l1.48-.32V180H16a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24ZM44,102.27l168-36V180H192V120a12,12,0,0,0-12-12H76a12,12,0,0,0-12,12v60H44ZM168,144H88V132h80ZM88,168h80v12H88Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,186H230V56.28l11.26-2.41a6,6,0,1,0-2.52-11.74l-224,48a6,6,0,0,0,2.52,11.74L26,100v86H16a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12ZM38,97.42,218,58.85V186H190V128a6,6,0,0,0-6-6H72a6,6,0,0,0-6,6v58H38ZM178,154H78V134H178ZM78,166H178v20H78Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,184h-8V57.9l9.67-2.08a8,8,0,1,0-3.35-15.64l-224,48A8,8,0,0,0,16,104a8.16,8.16,0,0,0,1.69-.18L24,102.47V184H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM40,99,216,61.33V184H192V128a8,8,0,0,0-8-8H72a8,8,0,0,0-8,8v56H40Zm136,53H80V136h96ZM80,168h96v16H80Z"></path>
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