/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Octagon(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M227.31,80.23,175.77,28.69A16.13,16.13,0,0,0,164.45,24H91.55a16.13,16.13,0,0,0-11.32,4.69L28.69,80.23A16.13,16.13,0,0,0,24,91.55v72.9a16.13,16.13,0,0,0,4.69,11.32l51.54,51.54A16.13,16.13,0,0,0,91.55,232h72.9a16.13,16.13,0,0,0,11.32-4.69l51.54-51.54A16.13,16.13,0,0,0,232,164.45V91.55A16.13,16.13,0,0,0,227.31,80.23Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,91.55v72.9a8,8,0,0,1-2.34,5.66l-51.55,51.55a8,8,0,0,1-5.66,2.34H91.55a8,8,0,0,1-5.66-2.34L34.34,170.11A8,8,0,0,1,32,164.45V91.55a8,8,0,0,1,2.34-5.66L85.89,34.34A8,8,0,0,1,91.55,32h72.9a8,8,0,0,1,5.66,2.34l51.55,51.55A8,8,0,0,1,224,91.55Z" opacity="0.2"/><path d="M227.31,80.24,175.76,28.69A15.86,15.86,0,0,0,164.45,24H91.55a15.86,15.86,0,0,0-11.31,4.69L28.69,80.24A15.86,15.86,0,0,0,24,91.55v72.9a15.86,15.86,0,0,0,4.69,11.31l51.55,51.55A15.86,15.86,0,0,0,91.55,232h72.9a15.86,15.86,0,0,0,11.31-4.69l51.55-51.55A15.86,15.86,0,0,0,232,164.45V91.55A15.86,15.86,0,0,0,227.31,80.24ZM216,164.45,164.45,216H91.55L40,164.45V91.55L91.55,40h72.9L216,91.55Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M224.49,83.06,172.94,31.51A12,12,0,0,0,164.45,28H91.55a12,12,0,0,0-8.49,3.51L31.51,83.06A12,12,0,0,0,28,91.55v72.9a12,12,0,0,0,3.51,8.49l51.55,51.55A12,12,0,0,0,91.55,228h72.9a12,12,0,0,0,8.49-3.51l51.55-51.55a12,12,0,0,0,3.51-8.49V91.55A12,12,0,0,0,224.49,83.06ZM220,164.45a4,4,0,0,1-1.17,2.83l-51.55,51.55a4,4,0,0,1-2.83,1.17H91.55a4,4,0,0,1-2.83-1.17L37.17,167.28A4,4,0,0,1,36,164.45V91.55a4,4,0,0,1,1.17-2.83L88.72,37.17A4,4,0,0,1,91.55,36h72.9a4,4,0,0,1,2.83,1.17l51.55,51.55A4,4,0,0,1,220,91.55Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M230.14,77.41,178.59,25.86A19.85,19.85,0,0,0,164.45,20H91.55a19.85,19.85,0,0,0-14.14,5.86L25.86,77.41A19.85,19.85,0,0,0,20,91.55v72.9a19.85,19.85,0,0,0,5.86,14.14l51.55,51.55A19.85,19.85,0,0,0,91.55,236h72.9a19.85,19.85,0,0,0,14.14-5.86l51.55-51.55A19.85,19.85,0,0,0,236,164.45V91.55A19.85,19.85,0,0,0,230.14,77.41ZM212,162.79,162.79,212H93.21L44,162.79V93.21L93.21,44h69.58L212,93.21Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M225.9,81.65,174.35,30.1a13.92,13.92,0,0,0-9.9-4.1H91.55a13.92,13.92,0,0,0-9.9,4.1L30.1,81.65a13.92,13.92,0,0,0-4.1,9.9v72.9a13.92,13.92,0,0,0,4.1,9.9L81.65,225.9a13.92,13.92,0,0,0,9.9,4.1h72.9a13.92,13.92,0,0,0,9.9-4.1l51.55-51.55a13.92,13.92,0,0,0,4.1-9.9V91.55A13.92,13.92,0,0,0,225.9,81.65Zm-7.9,82.8a2,2,0,0,1-.59,1.42l-51.55,51.54a2,2,0,0,1-1.41.59H91.55a2,2,0,0,1-1.42-.59L38.59,165.87a2,2,0,0,1-.59-1.42V91.55a2,2,0,0,1,.59-1.42L90.14,38.59A2,2,0,0,1,91.55,38h72.9a2,2,0,0,1,1.42.59l51.54,51.55a2,2,0,0,1,.59,1.41Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M227.31,80.24,175.76,28.69A15.86,15.86,0,0,0,164.45,24H91.55a15.86,15.86,0,0,0-11.31,4.69L28.69,80.24A15.86,15.86,0,0,0,24,91.55v72.9a15.86,15.86,0,0,0,4.69,11.31l51.55,51.55A15.86,15.86,0,0,0,91.55,232h72.9a15.86,15.86,0,0,0,11.31-4.69l51.55-51.55A15.86,15.86,0,0,0,232,164.45V91.55A15.86,15.86,0,0,0,227.31,80.24ZM216,164.45,164.45,216H91.55L40,164.45V91.55L91.55,40h72.9L216,91.55Z"/> }.into_view()
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