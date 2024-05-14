//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="map", feature ="objects"))]
#[component]
pub fn Airplane(
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
                <path d="M240,136v32a8,8,0,0,1-8,8,7.61,7.61,0,0,1-1.57-.16L156,161v23.73l17.66,17.65A8,8,0,0,1,176,208v24a8,8,0,0,1-11,7.43l-37-14.81L91,239.43A8,8,0,0,1,80,232V208a8,8,0,0,1,2.34-5.66L100,184.69V161L25.57,175.84A7.61,7.61,0,0,1,24,176a8,8,0,0,1-8-8V136a8,8,0,0,1,4.42-7.16L100,89.06V44a28,28,0,0,1,56,0V89.06l79.58,39.78A8,8,0,0,1,240,136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M152,152v32l16,16v32l-40-16L88,232V200l16-16V152L24,168V136l80-40V48a24,24,0,0,1,48,0V96l80,40v32Z"
        opacity="0.2"
    ></path>
    <path d="M235.58,128.84,160,91.06V48a32,32,0,0,0-64,0V91.06L20.42,128.84A8,8,0,0,0,16,136v32a8,8,0,0,0,9.57,7.84L96,161.76v18.93L82.34,194.34A8,8,0,0,0,80,200v32a8,8,0,0,0,11,7.43l37-14.81,37,14.81A8,8,0,0,0,176,232V200a8,8,0,0,0-2.34-5.66L160,180.69V161.76l70.43,14.08A8,8,0,0,0,240,168V136A8,8,0,0,0,235.58,128.84ZM224,158.24l-70.43-14.08A8,8,0,0,0,144,152v32a8,8,0,0,0,2.34,5.66L160,203.31v16.87l-29-11.61a8,8,0,0,0-5.94,0L96,220.18V203.31l13.66-13.65A8,8,0,0,0,112,184V152a8,8,0,0,0-9.57-7.84L32,158.24v-17.3l75.58-37.78A8,8,0,0,0,112,96V48a16,16,0,0,1,32,0V96a8,8,0,0,0,4.42,7.16L224,140.94Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M233.79,132.42,156,93.53V48a28,28,0,0,0-56,0V93.53L22.21,132.42A4,4,0,0,0,20,136v32a4,4,0,0,0,4.78,3.92l75.22-15v25.46L85.17,197.17A4,4,0,0,0,84,200v32a4,4,0,0,0,5.49,3.71L128,220.31l38.51,15.4A3.87,3.87,0,0,0,168,236a3.94,3.94,0,0,0,2.24-.69A4,4,0,0,0,172,232V200a4,4,0,0,0-1.17-2.83L156,182.34V156.88l75.22,15A4,4,0,0,0,236,168V136A4,4,0,0,0,233.79,132.42ZM228,163.12l-75.22-15A4,4,0,0,0,148,152v32a4,4,0,0,0,1.17,2.83L164,201.66v24.43l-34.51-13.8a4,4,0,0,0-3,0L92,226.09V201.66l14.83-14.83A4,4,0,0,0,108,184V152a4,4,0,0,0-4.78-3.92L28,163.12V138.47l77.79-38.89A4,4,0,0,0,108,96V48a20,20,0,0,1,40,0V96a4,4,0,0,0,2.21,3.58L228,138.47Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M237.37,125.27,164,88.58V48a36,36,0,0,0-72,0V88.58L18.63,125.27A12,12,0,0,0,12,136v32a12,12,0,0,0,14.35,11.77L92,166.64V179L79.51,191.51A12,12,0,0,0,76,200v32a12,12,0,0,0,16.46,11.14L128,228.92l35.54,14.22A11.91,11.91,0,0,0,168,244a12,12,0,0,0,12-12V200a12,12,0,0,0-3.51-8.49L164,179V166.64l65.65,13.13A12,12,0,0,0,244,168V136A12,12,0,0,0,237.37,125.27ZM220,153.36l-65.65-13.13A12,12,0,0,0,140,152v32a12,12,0,0,0,3.51,8.49L156,205v9.31l-23.54-9.42a12,12,0,0,0-8.92,0L100,214.28V205l12.49-12.48A12,12,0,0,0,116,184V152a12,12,0,0,0-14.35-11.77L36,153.36v-9.94l73.37-36.69A12,12,0,0,0,116,96V48a12,12,0,0,1,24,0V96a12,12,0,0,0,6.63,10.73L220,143.42Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M234.68,130.63,158,92.29V48a30,30,0,0,0-60,0V92.29L21.32,130.63A6,6,0,0,0,18,136v32a6,6,0,0,0,7.18,5.88L98,159.32v22.19L83.76,195.76A6,6,0,0,0,82,200v32a6,6,0,0,0,8.23,5.57L128,222.46l37.77,15.11A6,6,0,0,0,174,232V200a6,6,0,0,0-1.76-4.24L158,181.51V159.32l72.82,14.56A6,6,0,0,0,238,168V136A6,6,0,0,0,234.68,130.63ZM226,160.68l-72.82-14.56A6,6,0,0,0,146,152v32a6,6,0,0,0,1.76,4.24L162,202.49v20.65l-31.77-12.71a6,6,0,0,0-4.46,0L94,223.14V202.49l14.24-14.25A6,6,0,0,0,110,184V152a6,6,0,0,0-7.18-5.88L30,160.68v-21l76.68-38.34A6,6,0,0,0,110,96V48a18,18,0,0,1,36,0V96a6,6,0,0,0,3.32,5.37L226,139.71Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M235.58,128.84,160,91.06V48a32,32,0,0,0-64,0V91.06L20.42,128.84A8,8,0,0,0,16,136v32a8,8,0,0,0,9.57,7.84L96,161.76v18.93L82.34,194.34A8,8,0,0,0,80,200v32a8,8,0,0,0,11,7.43l37-14.81,37,14.81A8,8,0,0,0,176,232V200a8,8,0,0,0-2.34-5.66L160,180.69V161.76l70.43,14.08A8,8,0,0,0,240,168V136A8,8,0,0,0,235.58,128.84ZM224,158.24l-70.43-14.08A8,8,0,0,0,144,152v32a8,8,0,0,0,2.34,5.66L160,203.31v16.87l-29-11.61a8,8,0,0,0-5.94,0L96,220.18V203.31l13.66-13.65A8,8,0,0,0,112,184V152a8,8,0,0,0-9.57-7.84L32,158.24v-17.3l75.58-37.78A8,8,0,0,0,112,96V48a16,16,0,0,1,32,0V96a8,8,0,0,0,4.42,7.16L224,140.94Z"></path>
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