//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn FolderSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M216,68H132L105.33,48a20.12,20.12,0,0,0-12-4H40A20,20,0,0,0,20,64V200a20,20,0,0,0,20,20H216.89A19.13,19.13,0,0,0,236,200.89V88A20,20,0,0,0,216,68Zm-4,128H44V68H92l26.67,20a20.12,20.12,0,0,0,12,4H212Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,88V200.89a7.11,7.11,0,0,1-7.11,7.11H40a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H93.33a8,8,0,0,1,4.8,1.6l27.74,20.8a8,8,0,0,0,4.8,1.6H216A8,8,0,0,1,224,88Z"
        opacity="0.2"
    ></path>
    <path d="M216,72H130.67L102.93,51.2a16.12,16.12,0,0,0-9.6-3.2H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216.89A15.13,15.13,0,0,0,232,200.89V88A16,16,0,0,0,216,72Zm0,128H40V64H93.33l27.74,20.8a16.12,16.12,0,0,0,9.6,3.2H216Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,72H130.67L102.92,51.2A16,16,0,0,0,93.34,48H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216.89A15.13,15.13,0,0,0,232,200.89V88A16,16,0,0,0,216,72Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,74H130.67a2,2,0,0,1-1.2-.4L101.73,52.8a14,14,0,0,0-8.4-2.8H40A14,14,0,0,0,26,64V200a14,14,0,0,0,14,14H216.89A13.12,13.12,0,0,0,230,200.89V88A14,14,0,0,0,216,74Zm2,126.89a1.11,1.11,0,0,1-1.11,1.11H40a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H93.33a2,2,0,0,1,1.2.4l27.74,20.8a14,14,0,0,0,8.4,2.8H216a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,72H130.67L102.93,51.2a16.12,16.12,0,0,0-9.6-3.2H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216.89A15.13,15.13,0,0,0,232,200.89V88A16,16,0,0,0,216,72Zm0,128H40V64H93.33l27.74,20.8a16.12,16.12,0,0,0,9.6,3.2H216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,76H130.67a4,4,0,0,1-2.4-.8L100.53,54.4a12.05,12.05,0,0,0-7.2-2.4H40A12,12,0,0,0,28,64V200a12,12,0,0,0,12,12H216.89A11.12,11.12,0,0,0,228,200.89V88A12,12,0,0,0,216,76Zm4,124.89a3.12,3.12,0,0,1-3.11,3.11H40a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H93.33a4,4,0,0,1,2.4.8l27.74,20.8a12.05,12.05,0,0,0,7.2,2.4H216a4,4,0,0,1,4,4Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
