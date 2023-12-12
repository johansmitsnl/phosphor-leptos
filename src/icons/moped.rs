//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Moped(
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
                <path d="M212,124c-1.22,0-2.43.06-3.62.16L175.24,35.79A12,12,0,0,0,164,28H132a12,12,0,0,0,0,24h23.68l30.23,80.6A44.28,44.28,0,0,0,171,152h-29.2L119.24,91.79A12,12,0,0,0,108,84H28a12,12,0,0,0,0,24h4v2.92A60.14,60.14,0,0,0,0,164a12,12,0,0,0,12,12h4.74a44,44,0,0,0,86.52,0h65.48A44,44,0,1,0,212,124ZM48,130.05a12,12,0,0,0,8-11.32V108H99.68l16.5,44H26.06A36,36,0,0,1,48,130.05ZM60,188a20,20,0,0,1-18.32-12H78.32A20,20,0,0,1,60,188Zm152,0a20,20,0,1,1,20-20A20,20,0,0,1,212,188Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M131,168H8a48,48,0,0,1,32-45.27V96h64Z" opacity="0.2"></path>
    <path d="M216,128a39.3,39.3,0,0,0-6.27.5L175.49,37.19A8,8,0,0,0,168,32H136a8,8,0,0,0,0,16h26.46l32.3,86.13a40.13,40.13,0,0,0-18,25.87H136.54l-25-66.81A8,8,0,0,0,104,88H24a8,8,0,0,0,0,16h8v13.39A56.12,56.12,0,0,0,0,168a8,8,0,0,0,8,8h8.8a40,40,0,0,0,78.4,0h81.6A40,40,0,1,0,216,128ZM56,192a24,24,0,0,1-22.62-16H78.62A24,24,0,0,1,56,192ZM16.81,160a40.07,40.07,0,0,1,25.86-29.73A8,8,0,0,0,48,122.73V104H98.46l21,56ZM216,192a24,24,0,0,1-15.43-42.36l7.94,21.17a8,8,0,0,0,15-5.62L215.55,144H216a24,24,0,0,1,0,48Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,128a39.3,39.3,0,0,0-6.27.5L175.49,37.19A8,8,0,0,0,168,32H136a8,8,0,0,0,0,16h26.46l32.3,86.13a40.13,40.13,0,0,0-18,25.87H136.54l-25-66.81A8,8,0,0,0,104,88H24a8,8,0,0,0,0,16h8v13.39A56.12,56.12,0,0,0,0,168a8,8,0,0,0,8,8h8.8a40,40,0,0,0,78.4,0h81.6A40,40,0,1,0,216,128ZM56,192a24,24,0,0,1-22.62-16H78.62A24,24,0,0,1,56,192Zm160,0a24,24,0,0,1-15.43-42.36l7.94,21.17a8,8,0,0,0,15-5.62L215.55,144H216a24,24,0,0,1,0,48Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,130a37.72,37.72,0,0,0-7.56.76L173.62,37.89A6,6,0,0,0,168,34H136a6,6,0,0,0,0,12h27.84l33.37,89a38.08,38.08,0,0,0-18.73,27H135.16L109.62,93.89A6,6,0,0,0,104,90H24a6,6,0,0,0,0,12H34v16.67A54.12,54.12,0,0,0,2,168a6,6,0,0,0,6,6H18.48a38,38,0,0,0,75,0h85A38,38,0,1,0,216,130ZM42,128.39a6,6,0,0,0,4-5.66V102H99.84l22.5,60H14.43A42.07,42.07,0,0,1,42,128.39ZM56,194a26,26,0,0,1-25.29-20H81.29A26,26,0,0,1,56,194Zm160,0a26,26,0,0,1-14.5-47.57l8.88,23.68a6,6,0,0,0,11.24-4.22l-8.88-23.68A26.91,26.91,0,0,1,216,142a26,26,0,0,1,0,52Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,128a39.3,39.3,0,0,0-6.27.5L175.49,37.19A8,8,0,0,0,168,32H136a8,8,0,0,0,0,16h26.46l32.3,86.13a40.13,40.13,0,0,0-18,25.87H136.54l-25-66.81A8,8,0,0,0,104,88H24a8,8,0,0,0,0,16h8v13.39A56.12,56.12,0,0,0,0,168a8,8,0,0,0,8,8h8.8a40,40,0,0,0,78.4,0h81.6A40,40,0,1,0,216,128ZM42.67,130.27A8,8,0,0,0,48,122.73V104H98.46l21,56H16.81A40.07,40.07,0,0,1,42.67,130.27ZM56,192a24,24,0,0,1-22.62-16H78.62A24,24,0,0,1,56,192Zm160,0a24,24,0,0,1-15.43-42.36l7.94,21.17a8,8,0,0,0,15-5.62L215.55,144H216a24,24,0,0,1,0,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,132a35.7,35.7,0,0,0-8.82,1.1L171.75,38.6A4,4,0,0,0,168,36H136a4,4,0,0,0,0,8h29.23l34.47,91.92A36.06,36.06,0,0,0,180.23,164H133.77l-26-69.4A4,4,0,0,0,104,92H24a4,4,0,0,0,0,8H36v20A52.1,52.1,0,0,0,4,168a4,4,0,0,0,4,4H20.23a36,36,0,0,0,71.54,0h88.46A36,36,0,1,0,216,132ZM41.33,126.5A4,4,0,0,0,44,122.73V100h57.23l24,64h-113A44.1,44.1,0,0,1,41.33,126.5ZM56,196a28,28,0,0,1-27.71-24H83.71A28,28,0,0,1,56,196Zm160,0a28,28,0,0,1-13.47-52.54l9.72,25.94a4,4,0,1,0,7.5-2.8L210,140.65A28,28,0,1,1,216,196Z"></path>
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
