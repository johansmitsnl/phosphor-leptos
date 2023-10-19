/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Eject(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M224,176v24a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V176a16,16,0,0,1,16-16H208A16,16,0,0,1,224,176ZM48.24,144H207.76a16.18,16.18,0,0,0,14.93-9.76,15.59,15.59,0,0,0-3.1-17.12L145.86,39.61a24.76,24.76,0,0,0-35.72,0L36.41,117.12h0a15.59,15.59,0,0,0-3.1,17.12A16.18,16.18,0,0,0,48.24,144Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,176v24a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V176a8,8,0,0,1,8-8H208A8,8,0,0,1,216,176ZM48.23,136H207.77c7.16,0,10.89-8.27,6-13.37l-73.74-77.5a16.76,16.76,0,0,0-24.14,0l-73.74,77.5C37.34,127.73,41.07,136,48.23,136Z" opacity="0.2"/><path d="M208,160H48a16,16,0,0,0-16,16v24a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V176A16,16,0,0,0,208,160Zm0,40H48V176H208ZM48.24,144H207.76a16.18,16.18,0,0,0,14.93-9.76,15.59,15.59,0,0,0-3.1-17.12L145.86,39.61a24.76,24.76,0,0,0-35.72,0L36.41,117.12h0a15.59,15.59,0,0,0-3.1,17.12A16.18,16.18,0,0,0,48.24,144Zm73.49-93.36a8.77,8.77,0,0,1,12.54,0L207.85,128H48.14Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,164H48a12,12,0,0,0-12,12v24a12,12,0,0,0,12,12H208a12,12,0,0,0,12-12V176A12,12,0,0,0,208,164Zm4,36a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V176a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4ZM48.23,140H207.77A12,12,0,0,0,219,132.67a11.68,11.68,0,0,0-2.33-12.8L143,42.37a20.75,20.75,0,0,0-29.92,0L39.3,119.87A11.68,11.68,0,0,0,37,132.67,12,12,0,0,0,48.23,140ZM45.1,125.39l73.73-77.51a12.78,12.78,0,0,1,18.34,0l73.73,77.51a3.66,3.66,0,0,1,.77,4.12,4.1,4.1,0,0,1-3.9,2.49H48.23a4.1,4.1,0,0,1-3.9-2.49A3.66,3.66,0,0,1,45.1,125.39Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,160H48a20,20,0,0,0-20,20v20a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V180A20,20,0,0,0,208,160Zm-4,36H52V184H204ZM48.27,144H207.73a20.27,20.27,0,0,0,14.61-34.3L148.58,32.78a28.51,28.51,0,0,0-41.16,0L33.66,109.7A20.27,20.27,0,0,0,48.27,144Zm76.48-94.61a4.49,4.49,0,0,1,6.5,0L199,120H57Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,162H48a14,14,0,0,0-14,14v24a14,14,0,0,0,14,14H208a14,14,0,0,0,14-14V176A14,14,0,0,0,208,162Zm2,38a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V176a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2ZM48.24,142H207.76a14.19,14.19,0,0,0,13.1-8.55,13.61,13.61,0,0,0-2.72-14.95L144.41,41a22.76,22.76,0,0,0-32.82,0L37.86,118.5a13.61,13.61,0,0,0-2.72,14.95A14.19,14.19,0,0,0,48.24,142Zm-1.69-15.23,73.73-77.51a10.77,10.77,0,0,1,15.44,0l73.73,77.51a1.67,1.67,0,0,1,.38,2,2.11,2.11,0,0,1-2.07,1.27H48.24a2.11,2.11,0,0,1-2.07-1.27A1.67,1.67,0,0,1,46.55,126.77Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,160H48a16,16,0,0,0-16,16v24a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V176A16,16,0,0,0,208,160Zm0,40H48V176H208ZM48.24,144H207.76a16.18,16.18,0,0,0,14.93-9.76,15.59,15.59,0,0,0-3.1-17.12L145.86,39.61a24.76,24.76,0,0,0-35.72,0L36.41,117.12h0a15.59,15.59,0,0,0-3.1,17.12A16.18,16.18,0,0,0,48.24,144Zm73.49-93.36a8.77,8.77,0,0,1,12.54,0L207.85,128H48.14Z"/> }.into_view()
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