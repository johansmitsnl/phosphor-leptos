//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="map", feature ="objects"))]
#[component]
pub fn ShoppingCartSimple(
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
                <path d="M239.71,74.14l-25.64,92.28A24.06,24.06,0,0,1,191,184H92.16A24.06,24.06,0,0,1,69,166.42L33.92,40H16a8,8,0,0,1,0-16H40a8,8,0,0,1,7.71,5.86L57.19,64H232a8,8,0,0,1,7.71,10.14ZM88,200a16,16,0,1,0,16,16A16,16,0,0,0,88,200Zm104,0a16,16,0,1,0,16,16A16,16,0,0,0,192,200Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,72l-25.63,92.28A16,16,0,0,1,191,176H92.16a16,16,0,0,1-15.41-11.72L51.11,72Z"
        opacity="0.2"
    ></path>
    <path d="M104,216a16,16,0,1,1-16-16A16,16,0,0,1,104,216Zm88-16a16,16,0,1,0,16,16A16,16,0,0,0,192,200ZM239.71,74.14l-25.64,92.28A24.06,24.06,0,0,1,191,184H92.16A24.06,24.06,0,0,1,69,166.42L33.92,40H16a8,8,0,0,1,0-16H40a8,8,0,0,1,7.71,5.86L57.19,64H232a8,8,0,0,1,7.71,10.14ZM221.47,80H61.64l22.81,82.14A8,8,0,0,0,92.16,168H191a8,8,0,0,0,7.71-5.86Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.18,69.58A4,4,0,0,0,232,68H54.15L43.85,30.93A4,4,0,0,0,40,28H16a4,4,0,0,0,0,8H37L72.89,165.35A20.06,20.06,0,0,0,92.16,180H191a20.06,20.06,0,0,0,19.27-14.65l25.63-92.28A4,4,0,0,0,235.18,69.58Zm-32.67,93.63A12,12,0,0,1,191,172H92.16a12,12,0,0,1-11.56-8.79L56.37,76H226.74ZM100,216a12,12,0,1,1-12-12A12,12,0,0,1,100,216Zm104,0a12,12,0,1,1-12-12A12,12,0,0,1,204,216Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M241.55,64.74A12,12,0,0,0,232,60H60.23L51.56,28.79A12,12,0,0,0,40,20H20a12,12,0,0,0,0,24H30.88l34.3,123.49a28.09,28.09,0,0,0,27,20.51H191a28.09,28.09,0,0,0,27-20.51l25.63-92.28A12,12,0,0,0,241.55,64.74ZM194.8,161.07A4,4,0,0,1,191,164H92.16a4,4,0,0,1-3.85-2.93L66.9,84H216.21ZM108,220a20,20,0,1,1-20-20A20,20,0,0,1,108,220Zm104,0a20,20,0,1,1-20-20A20,20,0,0,1,212,220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M236.78,68.37A6,6,0,0,0,232,66H55.67L45.78,30.39A6,6,0,0,0,40,26H16a6,6,0,0,0,0,12H35.44L71,165.89A22.08,22.08,0,0,0,92.16,182H191a22.08,22.08,0,0,0,21.2-16.11l25.63-92.28A6,6,0,0,0,236.78,68.37Zm-36.2,94.31A10,10,0,0,1,191,170H92.16a10,10,0,0,1-9.63-7.32L59,78H224.11ZM102,216a14,14,0,1,1-14-14A14,14,0,0,1,102,216Zm104,0a14,14,0,1,1-14-14A14,14,0,0,1,206,216Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M104,216a16,16,0,1,1-16-16A16,16,0,0,1,104,216Zm88-16a16,16,0,1,0,16,16A16,16,0,0,0,192,200ZM239.71,74.14l-25.64,92.28A24.06,24.06,0,0,1,191,184H92.16A24.06,24.06,0,0,1,69,166.42L33.92,40H16a8,8,0,0,1,0-16H40a8,8,0,0,1,7.71,5.86L57.19,64H232a8,8,0,0,1,7.71,10.14ZM221.47,80H61.64l22.81,82.14A8,8,0,0,0,92.16,168H191a8,8,0,0,0,7.71-5.86Z"></path>
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