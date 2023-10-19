/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowsInLineVertical(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M90.34,69.66A8,8,0,0,1,96,56h24V16a8,8,0,0,1,16,0V56h24a8,8,0,0,1,5.66,13.66l-32,32a8,8,0,0,1-11.32,0Zm43.32,84.68a8,8,0,0,0-11.32,0l-32,32A8,8,0,0,0,96,200h24v40a8,8,0,0,0,16,0V200h24a8,8,0,0,0,5.66-13.66ZM216,120H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,96,96,64h64ZM96,192h64l-32-32Z" opacity="0.2"/><path d="M224,128a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM90.34,69.66A8,8,0,0,1,96,56h24V16a8,8,0,0,1,16,0V56h24a8,8,0,0,1,5.66,13.66l-32,32a8,8,0,0,1-11.32,0Zm25,2.34L128,84.69,140.69,72Zm50.35,114.34A8,8,0,0,1,160,200H136v40a8,8,0,0,1-16,0V200H96a8,8,0,0,1-5.66-13.66l32-32a8,8,0,0,1,11.32,0Zm-25-2.34L128,171.31,115.31,184Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M220,128a4,4,0,0,1-4,4H40a4,4,0,0,1,0-8H216A4,4,0,0,1,220,128ZM125.17,98.83a4,4,0,0,0,5.66,0l32-32a4,4,0,1,0-5.66-5.66L132,86.34V16a4,4,0,0,0-8,0V86.34L98.83,61.17a4,4,0,0,0-5.66,5.66Zm5.66,58.34a4,4,0,0,0-5.66,0l-32,32a4,4,0,0,0,5.66,5.66L124,169.66V240a4,4,0,0,0,8,0V169.66l25.17,25.17a4,4,0,0,0,5.66-5.66Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M228,128a12,12,0,0,1-12,12H40a12,12,0,0,1,0-24H216A12,12,0,0,1,228,128ZM119.51,96.49a12,12,0,0,0,17,0l32-32a12,12,0,0,0-17-17L140,59V16a12,12,0,0,0-24,0V59L104.49,47.51a12,12,0,0,0-17,17Zm17,63a12,12,0,0,0-17,0l-32,32a12,12,0,0,0,17,17L116,197v43a12,12,0,0,0,24,0V197l11.51,11.52a12,12,0,0,0,17-17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222,128a6,6,0,0,1-6,6H40a6,6,0,0,1,0-12H216A6,6,0,0,1,222,128Zm-98.24-27.76a6,6,0,0,0,8.48,0l32-32a6,6,0,0,0-8.48-8.48L134,81.51V16a6,6,0,0,0-12,0V81.51L100.24,59.76a6,6,0,0,0-8.48,8.48Zm8.48,55.52a6,6,0,0,0-8.48,0l-32,32a6,6,0,0,0,8.48,8.48L122,174.49V240a6,6,0,0,0,12,0V174.49l21.76,21.75a6,6,0,0,0,8.48-8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,128a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM122.34,101.66a8,8,0,0,0,11.32,0l32-32a8,8,0,0,0-11.32-11.32L136,76.69V16a8,8,0,0,0-16,0V76.69L101.66,58.34A8,8,0,0,0,90.34,69.66Zm11.32,52.68a8,8,0,0,0-11.32,0l-32,32a8,8,0,0,0,11.32,11.32L120,179.31V240a8,8,0,0,0,16,0V179.31l18.34,18.35a8,8,0,0,0,11.32-11.32Z"/> }.into_view()
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