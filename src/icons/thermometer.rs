/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Thermometer(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M212,56a28,28,0,1,0,28,28A28,28,0,0,0,212,56Zm0,40a12,12,0,1,1,12-12A12,12,0,0,1,212,96Zm-60,50.08V40a32,32,0,0,0-64,0V146.08a56,56,0,1,0,64,0ZM136,80H104V40a16,16,0,0,1,32,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M152,138V48a32,32,0,0,0-64,0v90a56,56,0,1,0,64,0Zm-32,70a24,24,0,1,1,24-24A24,24,0,0,1,120,208Z" opacity="0.2"/><path d="M212,56a28,28,0,1,0,28,28A28,28,0,0,0,212,56Zm0,40a12,12,0,1,1,12-12A12,12,0,0,1,212,96Zm-84,57V88a8,8,0,0,0-16,0v65a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,120,200Zm40-66V48a40,40,0,0,0-80,0v86a64,64,0,1,0,80,0Zm-40,98a48,48,0,0,1-27.42-87.4A8,8,0,0,0,96,138V48a24,24,0,0,1,48,0v90a8,8,0,0,0,3.42,6.56A48,48,0,0,1,120,232Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M212,60a24,24,0,1,0,24,24A24,24,0,0,0,212,60Zm0,40a16,16,0,1,1,16-16A16,16,0,0,1,212,100Zm-88,56.29V88a4,4,0,0,0-8,0v68.29a28,28,0,1,0,8,0ZM120,204a20,20,0,1,1,20-20A20,20,0,0,1,120,204Zm36-68V48a36,36,0,0,0-72,0v88a60,60,0,1,0,72,0ZM120,236a52,52,0,0,1-29.71-94.68A4,4,0,0,0,92,138V48a28,28,0,0,1,56,0v90a4,4,0,0,0,1.71,3.28A52,52,0,0,1,120,236Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,52a32,32,0,1,0,32,32A32,32,0,0,0,212,52Zm0,40a8,8,0,1,1,8-8A8,8,0,0,1,212,92ZM160,56A52,52,0,0,0,56,56v94.69a64,64,0,1,0,104,0ZM108,228a40,40,0,0,1-30.91-65.39A12,12,0,0,0,80,154.78V56a28,28,0,0,1,56,0v98.77a12,12,0,0,0,2.77,7.68A40,40,0,0,1,108,228Zm24-40a24,24,0,1,1-36-20.78V92a12,12,0,0,1,24,0v75.22A24,24,0,0,1,132,188Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M212,58a26,26,0,1,0,26,26A26,26,0,0,0,212,58Zm0,40a14,14,0,1,1,14-14A14,14,0,0,1,212,98Zm-86,56.6V88a6,6,0,0,0-12,0v66.6a30,30,0,1,0,12,0ZM120,202a18,18,0,1,1,18-18A18,18,0,0,1,120,202Zm38-67V48a38,38,0,0,0-76,0v87a62,62,0,1,0,76,0Zm-38,99a50,50,0,0,1-28.57-91A6,6,0,0,0,94,138V48a26,26,0,0,1,52,0v90a6,6,0,0,0,2.57,4.92A50,50,0,0,1,120,234Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M212,56a28,28,0,1,0,28,28A28,28,0,0,0,212,56Zm0,40a12,12,0,1,1,12-12A12,12,0,0,1,212,96Zm-84,57V88a8,8,0,0,0-16,0v65a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,120,200Zm40-66V48a40,40,0,0,0-80,0v86a64,64,0,1,0,80,0Zm-40,98a48,48,0,0,1-27.42-87.4A8,8,0,0,0,96,138V48a24,24,0,0,1,48,0v90a8,8,0,0,0,3.42,6.56A48,48,0,0,1,120,232Z"/> }.into_view()
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