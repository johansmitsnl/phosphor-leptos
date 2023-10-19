/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn MediumLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M136,128A64,64,0,1,1,72,64,64.07,64.07,0,0,1,136,128Zm48-64c-5.68,0-16.4,2.76-24.32,21.25C154.73,96.8,152,112,152,128s2.73,31.2,7.68,42.75C167.6,189.24,178.32,192,184,192s16.4-2.76,24.32-21.25C213.27,159.2,216,144,216,128s-2.73-31.2-7.68-42.75C200.4,66.76,189.68,64,184,64Zm56,0a8,8,0,0,0-8,8V184a8,8,0,0,0,16,0V72A8,8,0,0,0,240,64Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,128A56,56,0,1,1,72,72,56,56,0,0,1,128,128Zm56-56c-13.25,0-24,25.07-24,56s10.75,56,24,56,24-25.07,24-56S197.25,72,184,72Z" opacity="0.2"/><path d="M72,64a64,64,0,1,0,64,64A64.07,64.07,0,0,0,72,64Zm0,112a48,48,0,1,1,48-48A48.05,48.05,0,0,1,72,176ZM184,64c-5.68,0-16.4,2.76-24.32,21.25C154.73,96.8,152,112,152,128s2.73,31.2,7.68,42.75C167.6,189.24,178.32,192,184,192s16.4-2.76,24.32-21.25C213.27,159.2,216,144,216,128s-2.73-31.2-7.68-42.75C200.4,66.76,189.68,64,184,64Zm0,112c-5.64,0-16-18.22-16-48s10.36-48,16-48,16,18.22,16,48S189.64,176,184,176ZM248,72V184a8,8,0,0,1-16,0V72a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M72,68a60,60,0,1,0,60,60A60.07,60.07,0,0,0,72,68Zm0,112a52,52,0,1,1,52-52A52.06,52.06,0,0,1,72,180ZM184,68c-16,0-28,25.79-28,60s12,60,28,60,28-25.79,28-60S200,68,184,68Zm0,112c-9.46,0-20-21.36-20-52s10.54-52,20-52,20,21.36,20,52S193.46,180,184,180ZM244,72V184a4,4,0,0,1-8,0V72a4,4,0,0,1,8,0Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M68,60a68,68,0,1,0,68,68A68.07,68.07,0,0,0,68,60Zm0,112a44,44,0,1,1,44-44A44.05,44.05,0,0,1,68,172ZM184,60c-23.63,0-36,34.21-36,68s12.37,68,36,68,36-34.21,36-68S207.63,60,184,60Zm0,111.87c-3.74-2.16-12-17.09-12-43.87s8.26-41.71,12-43.87c3.74,2.16,12,17.09,12,43.87S187.74,169.71,184,171.87ZM256,72V184a12,12,0,0,1-24,0V72a12,12,0,0,1,24,0Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M72,66a62,62,0,1,0,62,62A62.07,62.07,0,0,0,72,66Zm0,112a50,50,0,1,1,50-50A50.06,50.06,0,0,1,72,178ZM184,66c-17.1,0-30,26.65-30,62s12.9,62,30,62,30-26.65,30-62S201.1,66,184,66Zm0,112c-7.34,0-18-19.48-18-50s10.66-50,18-50,18,19.48,18,50S191.34,178,184,178ZM246,72V184a6,6,0,0,1-12,0V72a6,6,0,0,1,12,0Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M72,64a64,64,0,1,0,64,64A64.07,64.07,0,0,0,72,64Zm0,112a48,48,0,1,1,48-48A48.05,48.05,0,0,1,72,176ZM184,64c-5.68,0-16.4,2.76-24.32,21.25C154.73,96.8,152,112,152,128s2.73,31.2,7.68,42.75C167.6,189.24,178.32,192,184,192s16.4-2.76,24.32-21.25C213.27,159.2,216,144,216,128s-2.73-31.2-7.68-42.75C200.4,66.76,189.68,64,184,64Zm0,112c-5.64,0-16-18.22-16-48s10.36-48,16-48,16,18.22,16,48S189.64,176,184,176ZM248,72V184a8,8,0,0,1-16,0V72a8,8,0,0,1,16,0Z"/> }.into_view()
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