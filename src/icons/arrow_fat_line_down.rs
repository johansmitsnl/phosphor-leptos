/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowFatLineDown(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M72,40a8,8,0,0,1,8-8h96a8,8,0,0,1,0,16H80A8,8,0,0,1,72,40Zm159.39,92.94A8,8,0,0,0,224,128H184V72a8,8,0,0,0-8-8H80a8,8,0,0,0-8,8v56H32a8,8,0,0,0-5.66,13.66l96,96a8,8,0,0,0,11.32,0l96-96A8,8,0,0,0,231.39,132.94Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,136l-96,96L32,136H80V72h96v64Z" opacity="0.2"/><path d="M231.39,132.94A8,8,0,0,0,224,128H184V72a8,8,0,0,0-8-8H80a8,8,0,0,0-8,8v56H32a8,8,0,0,0-5.66,13.66l96,96a8,8,0,0,0,11.32,0l96-96A8,8,0,0,0,231.39,132.94ZM128,220.69,51.31,144H80a8,8,0,0,0,8-8V80h80v56a8,8,0,0,0,8,8h28.69ZM72,40a8,8,0,0,1,8-8h96a8,8,0,0,1,0,16H80A8,8,0,0,1,72,40Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M227.7,134.47A4,4,0,0,0,224,132H180V72a4,4,0,0,0-4-4H80a4,4,0,0,0-4,4v60H32a4,4,0,0,0-2.83,6.83l96,96a4,4,0,0,0,5.66,0l96-96A4,4,0,0,0,227.7,134.47ZM128,226.34,41.66,140H80a4,4,0,0,0,4-4V76h88v60a4,4,0,0,0,4,4h38.34ZM76,40a4,4,0,0,1,4-4h96a4,4,0,0,1,0,8H80A4,4,0,0,1,76,40Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M235.09,131.41A12,12,0,0,0,224,124H188V80a12,12,0,0,0-12-12H80A12,12,0,0,0,68,80v44H32a12,12,0,0,0-8.48,20.49l96,96a12,12,0,0,0,17,0l96-96A12,12,0,0,0,235.09,131.41ZM128,215,61,148H80a12,12,0,0,0,12-12V92h72v44a12,12,0,0,0,12,12h19ZM68,40A12,12,0,0,1,80,28h96a12,12,0,0,1,0,24H80A12,12,0,0,1,68,40Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M229.54,133.7A6,6,0,0,0,224,130H182V72a6,6,0,0,0-6-6H80a6,6,0,0,0-6,6v58H32a6,6,0,0,0-4.24,10.24l96,96a6,6,0,0,0,8.48,0l96-96A6,6,0,0,0,229.54,133.7ZM128,223.51,46.49,142H80a6,6,0,0,0,6-6V78h84v58a6,6,0,0,0,6,6h33.51ZM74,40a6,6,0,0,1,6-6h96a6,6,0,0,1,0,12H80A6,6,0,0,1,74,40Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M231.39,132.94A8,8,0,0,0,224,128H184V72a8,8,0,0,0-8-8H80a8,8,0,0,0-8,8v56H32a8,8,0,0,0-5.66,13.66l96,96a8,8,0,0,0,11.32,0l96-96A8,8,0,0,0,231.39,132.94ZM128,220.69,51.31,144H80a8,8,0,0,0,8-8V80h80v56a8,8,0,0,0,8,8h28.69ZM72,40a8,8,0,0,1,8-8h96a8,8,0,0,1,0,16H80A8,8,0,0,1,72,40Z"/> }.into_view()
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