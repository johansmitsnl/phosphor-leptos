/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ShoppingCartSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M96,216a16,16,0,1,1-16-16A16,16,0,0,1,96,216Zm88-16a16,16,0,1,0,16,16A16,16,0,0,0,184,200ZM230.44,67.25A8,8,0,0,0,224,64H48.32L40.21,35.6A16.08,16.08,0,0,0,24.82,24H8A8,8,0,0,0,8,40H24.82L61,166.59A24.11,24.11,0,0,0,84.07,184h96.11a23.89,23.89,0,0,0,22.94-16.94l28.53-92.71A8,8,0,0,0,230.44,67.25Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,72l-28.52,92.71A16,16,0,0,1,180.18,176H84.07a16,16,0,0,1-15.39-11.6L42.29,72Z" opacity="0.2"/><path d="M96,216a16,16,0,1,1-16-16A16,16,0,0,1,96,216Zm88-16a16,16,0,1,0,16,16A16,16,0,0,0,184,200ZM231.65,74.35l-28.53,92.71A23.89,23.89,0,0,1,180.18,184H84.07A24.11,24.11,0,0,1,61,166.59L24.82,40H8A8,8,0,0,1,8,24H24.82A16.08,16.08,0,0,1,40.21,35.6L48.32,64H224a8,8,0,0,1,7.65,10.35ZM213.17,80H52.89l23.49,82.2a8,8,0,0,0,7.69,5.8h96.11a8,8,0,0,0,7.65-5.65Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M92,216a12,12,0,1,1-12-12A12,12,0,0,1,92,216Zm92-12a12,12,0,1,0,12,12A12,12,0,0,0,184,204ZM227.82,73.18l-28.52,92.7A19.9,19.9,0,0,1,180.18,180H84.07a20.08,20.08,0,0,1-19.23-14.51L28.67,38.9A4,4,0,0,0,24.82,36H8a4,4,0,0,1,0-8H24.82a12.05,12.05,0,0,1,11.54,8.7L45.3,68H224a4,4,0,0,1,3.82,5.18ZM218.58,76h-171l24.94,87.3A12.05,12.05,0,0,0,84.07,172h96.11a11.94,11.94,0,0,0,11.47-8.47Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M100,216a20,20,0,1,1-20-20A20,20,0,0,1,100,216Zm84-20a20,20,0,1,0,20,20A20,20,0,0,0,184,196ZM235.47,75.53l-27.29,88.7A27.87,27.87,0,0,1,181.41,184H82.93A28.13,28.13,0,0,1,56,163.69L21.81,44H12a12,12,0,0,1,0-24H24.82A20.08,20.08,0,0,1,44.05,34.51L51.34,60H224a12,12,0,0,1,11.47,15.53ZM207.75,84H58.19l20.89,73.1a4,4,0,0,0,3.85,2.9h98.48a4,4,0,0,0,3.83-2.82Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M94,216a14,14,0,1,1-14-14A14,14,0,0,1,94,216Zm90-14a14,14,0,1,0,14,14A14,14,0,0,0,184,202ZM229.73,73.76l-28.52,92.71a21.89,21.89,0,0,1-21,15.53H84.07a22.1,22.1,0,0,1-21.16-16L26.75,39.45A2,2,0,0,0,24.82,38H8A6,6,0,0,1,8,26H24.82A14.05,14.05,0,0,1,38.28,36.15L46.81,66H224a6,6,0,0,1,5.73,7.76ZM215.88,78H50.24l24.21,84.75A10,10,0,0,0,84.07,170h96.11a10,10,0,0,0,9.56-7.06Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M96,216a16,16,0,1,1-16-16A16,16,0,0,1,96,216Zm88-16a16,16,0,1,0,16,16A16,16,0,0,0,184,200ZM231.65,74.35l-28.53,92.71A23.89,23.89,0,0,1,180.18,184H84.07A24.11,24.11,0,0,1,61,166.59L24.82,40H8A8,8,0,0,1,8,24H24.82A16.08,16.08,0,0,1,40.21,35.6L48.32,64H224a8,8,0,0,1,7.65,10.35ZM213.17,80H52.89l23.49,82.2a8,8,0,0,0,7.69,5.8h96.11a8,8,0,0,0,7.65-5.65Z"/> }.into_view()
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