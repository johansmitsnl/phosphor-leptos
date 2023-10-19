/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowBendDoubleUpRight(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M229.66,109.66l-48,48a8,8,0,0,1-11.32-11.32L212.69,104,170.34,61.66a8,8,0,0,1,11.32-11.32l48,48A8,8,0,0,1,229.66,109.66Zm-48-11.32-48-48A8,8,0,0,0,120,56V96.3A104.15,104.15,0,0,0,24,200a8,8,0,0,0,16,0,88.11,88.11,0,0,1,80-87.63V152a8,8,0,0,0,13.66,5.66l48-48A8,8,0,0,0,181.66,98.34Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M176,104l-48,48V56Z" opacity="0.2"/><path d="M229.66,109.66l-48,48a8,8,0,0,1-11.32-11.32L212.69,104,170.34,61.66a8,8,0,0,1,11.32-11.32l48,48A8,8,0,0,1,229.66,109.66Zm-48,0-48,48A8,8,0,0,1,120,152V112.37A88.11,88.11,0,0,0,40,200a8,8,0,0,1-16,0A104.15,104.15,0,0,1,120,96.3V56a8,8,0,0,1,13.66-5.66l48,48A8,8,0,0,1,181.66,109.66Zm-17-5.66L136,75.31v57.38Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M226.83,106.83l-48,48a4,4,0,0,1-5.66-5.66L218.34,104,173.17,58.83a4,4,0,0,1,5.66-5.66l48,48A4,4,0,0,1,226.83,106.83Zm-48-5.66-48-48a4,4,0,1,0-5.66,5.66L166.34,100H128A100.11,100.11,0,0,0,28,200a4,4,0,0,0,8,0,92.1,92.1,0,0,1,92-92h38.34l-41.17,41.17a4,4,0,0,0,5.66,5.66l48-48A4,4,0,0,0,178.83,101.17Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M232.49,112.49l-48,48a12,12,0,0,1-17-17L207,104,167.51,64.48a12,12,0,0,1,17-17l48,48A12,12,0,0,1,232.49,112.49Zm-56-17-48-48a12,12,0,1,0-17,17L139,92H128A108.12,108.12,0,0,0,20,200a12,12,0,0,0,24,0,84.09,84.09,0,0,1,84-84h11l-27.52,27.51a12,12,0,0,0,17,17l48-48A12,12,0,0,0,176.49,95.51Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M228.24,108.24l-48,48a6,6,0,0,1-8.48-8.48L215.51,104,171.76,60.24a6,6,0,0,1,8.48-8.48l48,48A6,6,0,0,1,228.24,108.24Zm-48-8.48-48-48a6,6,0,1,0-8.48,8.48L161.51,98H128A102.12,102.12,0,0,0,26,200a6,6,0,0,0,12,0,90.1,90.1,0,0,1,90-90h33.51l-37.75,37.76a6,6,0,1,0,8.48,8.48l48-48A6,6,0,0,0,180.24,99.76Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M229.66,109.66l-48,48a8,8,0,0,1-11.32-11.32L212.69,104,170.34,61.66a8,8,0,0,1,11.32-11.32l48,48A8,8,0,0,1,229.66,109.66Zm-48-11.32-48-48a8,8,0,0,0-11.32,11.32L156.69,96H128A104.11,104.11,0,0,0,24,200a8,8,0,0,0,16,0,88.1,88.1,0,0,1,88-88h28.69l-34.35,34.34a8,8,0,0,0,11.32,11.32l48-48A8,8,0,0,0,181.66,98.34Z"/> }.into_view()
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