//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="system"))]
#[component]
pub fn BluetoothX(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M184,176a8,8,0,0,1-3.2,6.4l-64,48A8,8,0,0,1,112,232a7.9,7.9,0,0,1-4.11-1.14,8.3,8.3,0,0,1-3.9-7.18V144L52.76,182.4a8,8,0,0,1-11.16-1.55,8.26,8.26,0,0,1,1.8-11.43L98.66,128,43.38,86.57a8.19,8.19,0,0,1-2.13-10.93,8,8,0,0,1,11.51-2L104,112V32.24a8.21,8.21,0,0,1,2.83-6.34,8,8,0,0,1,10-.3l33.62,25.2A4,4,0,0,1,152,54v52a4,4,0,0,1-1.6,3.2L125.34,128l55.5,41.6A8,8,0,0,1,184,176Zm53.47-77.87L219.37,80l18.11-18.11a8.21,8.21,0,0,0,.41-11.37,8,8,0,0,0-11.49-.18L208.05,68.69,189.93,50.58a8.23,8.23,0,0,0-10.83-.88,8,8,0,0,0-.73,12L196.73,80,178.58,98.13a8.2,8.2,0,0,0-.6,11.1,8,8,0,0,0,11.71.43l18.36-18.35,18.35,18.35a8,8,0,0,0,11.72-.43A8.21,8.21,0,0,0,237.51,98.13Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M176,176l-64,48V128ZM167.47,73.6,112,32v96l55.47-41.6A8,8,0,0,0,167.47,73.6Z"
        opacity="0.2"
    ></path>
    <path d="M180.8,169.6,125.33,128l23.47-17.6a8,8,0,0,0-9.6-12.8L120,112V48l19.2,14.4a8,8,0,1,0,9.6-12.8l-32-24A8,8,0,0,0,104,32v80L52.8,73.6a8,8,0,0,0-9.6,12.8L98.67,128,43.2,169.6a8,8,0,1,0,9.6,12.8L104,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM120,208V144l42.67,32ZM237.66,98.34a8,8,0,0,1-11.32,11.32L208,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L196.69,80,178.34,61.66a8,8,0,0,1,11.32-11.32L208,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L219.31,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M178.4,172.8,118.67,128l27.73-20.8a4,4,0,1,0-4.8-6.4L116,120V40l25.6,19.2a4,4,0,0,0,4.8-6.4l-32-24A4,4,0,0,0,108,32v88L50.4,76.8a4,4,0,0,0-4.8,6.4L105.33,128,45.6,172.8a4,4,0,0,0,4.8,6.4L108,136v88a4,4,0,0,0,2.21,3.58A4.05,4.05,0,0,0,112,228a4,4,0,0,0,2.4-.8l64-48a4,4,0,0,0,0-6.4ZM116,216V136l53.33,40ZM234.83,101.17a4,4,0,0,1-5.66,5.66L208,85.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L202.34,80,181.17,58.83a4,4,0,0,1,5.66-5.66L208,74.34l21.17-21.17a4,4,0,1,1,5.66,5.66L213.66,80Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M183.2,166.4,132,128l11.61-8.71a12,12,0,1,0-14.4-19.2L124,104V56l5.21,3.91a12,12,0,1,0,14.4-19.2L119.2,22.4A12,12,0,0,0,100,32v72L55.2,70.4A12,12,0,0,0,40.8,89.6L92,128,40.8,166.4a12,12,0,1,0,14.4,19.2L100,152v72a12,12,0,0,0,19.2,9.6l64-48a12,12,0,0,0,0-19.2ZM124,200V152l32,24ZM240.49,95.51a12,12,0,0,1-17,17L208,97l-15.51,15.52a12,12,0,1,1-17-17L191,80,175.52,64.49a12,12,0,1,1,17-17L208,63l15.51-15.52a12,12,0,0,1,17,17L225,80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M179.6,171.2,122,128l25.6-19.2a6,6,0,1,0-7.2-9.6L118,116V44l22.4,16.8a6,6,0,1,0,7.2-9.6l-32-24A6,6,0,0,0,106,32v84L51.6,75.2a6,6,0,0,0-7.2,9.6L102,128,44.4,171.2a6,6,0,0,0,7.2,9.6L106,140v84a6,6,0,0,0,9.6,4.8l64-48a6,6,0,0,0,0-9.6ZM118,212V140l48,36ZM236.24,99.76a6,6,0,1,1-8.48,8.48L208,88.49l-19.76,19.75a6,6,0,0,1-8.48-8.48L199.51,80,179.76,60.24a6,6,0,0,1,8.48-8.48L208,71.51l19.76-19.75a6,6,0,0,1,8.48,8.48L216.49,80Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M180.8,169.6,125.33,128l23.47-17.6a8,8,0,0,0-9.6-12.8L120,112V48l19.2,14.4a8,8,0,1,0,9.6-12.8l-32-24A8,8,0,0,0,104,32v80L52.8,73.6a8,8,0,0,0-9.6,12.8L98.67,128,43.2,169.6a8,8,0,1,0,9.6,12.8L104,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM120,208V144l42.67,32ZM237.66,98.34a8,8,0,0,1-11.32,11.32L208,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L196.69,80,178.34,61.66a8,8,0,0,1,11.32-11.32L208,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L219.31,80Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}