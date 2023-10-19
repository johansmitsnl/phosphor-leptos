/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Backspace(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,40H68.53a16.12,16.12,0,0,0-13.72,7.77L9.14,123.88a8,8,0,0,0,0,8.24l45.67,76.11h0A16.11,16.11,0,0,0,68.53,216H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM165.66,146.34a8,8,0,0,1-11.32,11.32L136,139.31l-18.35,18.35a8,8,0,0,1-11.31-11.32L124.69,128l-18.35-18.34a8,8,0,1,1,11.31-11.32L136,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L147.31,128Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,56V200a8,8,0,0,1-8,8H68.53a8,8,0,0,1-6.86-3.88L16,128,61.67,51.88A8,8,0,0,1,68.53,48H216A8,8,0,0,1,224,56Z" opacity="0.2"/><path d="M216,40H68.53a16.08,16.08,0,0,0-13.72,7.77L9.14,123.88a8,8,0,0,0,0,8.24l45.67,76.11A16.08,16.08,0,0,0,68.53,216H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM61.67,204.12,68.53,200h0ZM216,200H68.53l-43.2-72,43.2-72H216ZM106.34,146.34,124.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L136,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L147.31,128l18.35,18.34a8,8,0,0,1-11.32,11.32L136,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,44H68.53a12.06,12.06,0,0,0-10.29,5.83L12.57,125.94a4,4,0,0,0,0,4.12l45.67,76.11A12.06,12.06,0,0,0,68.53,212H216a12,12,0,0,0,12-12V56A12,12,0,0,0,216,44Zm4,156a4,4,0,0,1-4,4H68.53a4,4,0,0,1-3.43-1.94L20.67,128,65.1,53.94A4,4,0,0,1,68.53,52H216a4,4,0,0,1,4,4Zm-57.17-93.17L141.66,128l21.17,21.17a4,4,0,0,1-5.66,5.66L136,133.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L130.34,128l-21.17-21.17a4,4,0,0,1,5.66-5.66L136,122.34l21.17-21.17a4,4,0,1,1,5.66,5.66Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216,36H68.53a20.09,20.09,0,0,0-17.15,9.71L5.71,121.83a12,12,0,0,0,0,12.34l45.67,76.12A20.09,20.09,0,0,0,68.53,220H216a20,20,0,0,0,20-20V56A20,20,0,0,0,216,36Zm-4,160H70.8L30,128,70.8,60H212ZM103.51,143.51,119,128l-15.52-15.51a12,12,0,0,1,17-17L136,111l15.51-15.52a12,12,0,0,1,17,17L153,128l15.52,15.51a12,12,0,0,1-17,17L136,145l-15.51,15.52a12,12,0,0,1-17-17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,42H68.53a14,14,0,0,0-12,6.8L10.86,124.91a6,6,0,0,0,0,6.18L56.53,207.2a14,14,0,0,0,12,6.8H216a14,14,0,0,0,14-14V56A14,14,0,0,0,216,42Zm2,158a2,2,0,0,1-2,2H68.53a2,2,0,0,1-1.71-1h0L23,128,66.82,55a2,2,0,0,1,1.71-1H216a2,2,0,0,1,2,2Zm-53.76-91.76L144.48,128l19.76,19.76a6,6,0,1,1-8.48,8.48L136,136.48l-19.76,19.76a6,6,0,0,1-8.48-8.48L127.52,128l-19.76-19.76a6,6,0,0,1,8.48-8.48L136,119.52l19.76-19.76a6,6,0,0,1,8.48,8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,40H68.53a16.08,16.08,0,0,0-13.72,7.77L9.14,123.88a8,8,0,0,0,0,8.24l45.67,76.11A16.08,16.08,0,0,0,68.53,216H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM61.67,204.12,68.53,200h0ZM216,200H68.53l-43.2-72,43.2-72H216ZM106.34,146.34,124.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L136,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L147.31,128l18.35,18.34a8,8,0,0,1-11.32,11.32L136,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32Z"/> }.into_view()
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