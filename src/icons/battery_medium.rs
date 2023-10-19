/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn BatteryMedium(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M112,96v64a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8h56A8,8,0,0,1,112,96ZM224,80v96a24,24,0,0,1-24,24H32A24,24,0,0,1,8,176V80A24,24,0,0,1,32,56H200A24,24,0,0,1,224,80Zm-16,0a8,8,0,0,0-8-8H32a8,8,0,0,0-8,8v96a8,8,0,0,0,8,8H200a8,8,0,0,0,8-8Zm40,8a8,8,0,0,0-8,8v64a8,8,0,0,0,16,0V96A8,8,0,0,0,248,88Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,80v96a16,16,0,0,1-16,16H32a16,16,0,0,1-16-16V80A16,16,0,0,1,32,64H200A16,16,0,0,1,216,80Z" opacity="0.2"/><path d="M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM104,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M200,60H32A20,20,0,0,0,12,80v96a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V80A20,20,0,0,0,200,60Zm12,116a12,12,0,0,1-12,12H32a12,12,0,0,1-12-12V80A12,12,0,0,1,32,68H200a12,12,0,0,1,12,12ZM100,96v64a4,4,0,0,1-8,0V96a4,4,0,0,1,8,0ZM60,96v64a4,4,0,0,1-8,0V96a4,4,0,0,1,8,0Zm192,0v64a4,4,0,0,1-8,0V96a4,4,0,0,1,8,0Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M196,52H28A28,28,0,0,0,0,80v96a28,28,0,0,0,28,28H196a28,28,0,0,0,28-28V80A28,28,0,0,0,196,52Zm4,124a4,4,0,0,1-4,4H28a4,4,0,0,1-4-4V80a4,4,0,0,1,4-4H196a4,4,0,0,1,4,4Zm-96-72v48a12,12,0,0,1-24,0V104a12,12,0,0,1,24,0Zm-40,0v48a12,12,0,0,1-24,0V104a12,12,0,0,1,24,0Zm192,0v48a12,12,0,0,1-24,0V104a12,12,0,0,1,24,0Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M200,58H32A22,22,0,0,0,10,80v96a22,22,0,0,0,22,22H200a22,22,0,0,0,22-22V80A22,22,0,0,0,200,58Zm10,118a10,10,0,0,1-10,10H32a10,10,0,0,1-10-10V80A10,10,0,0,1,32,70H200a10,10,0,0,1,10,10ZM102,96v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0ZM62,96v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Zm192,0v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM104,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z"/> }.into_view()
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