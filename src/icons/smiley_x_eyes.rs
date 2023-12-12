//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn SmileyXEyes(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm60.49-92.49a12,12,0,0,1-17,17L164,129l-7.51,7.52a12,12,0,0,1-17-17L147,112l-7.52-7.51a12,12,0,0,1,17-17L164,95l7.51-7.52a12,12,0,0,1,17,17L181,112Zm-72,17a12,12,0,0,1-17,0L92,129l-7.51,7.52a12,12,0,0,1-17-17L75,112l-7.52-7.51a12,12,0,0,1,17-17L92,95l7.51-7.52a12,12,0,0,1,17,17L109,112l7.52,7.51A12,12,0,0,1,116.49,136.49ZM144,180a16,16,0,1,1-16-16A16,16,0,0,1,144,180Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm61.66-93.66a8,8,0,0,1-11.32,11.32L168,123.31l-10.34,10.35a8,8,0,0,1-11.32-11.32L156.69,112l-10.35-10.34a8,8,0,0,1,11.32-11.32L168,100.69l10.34-10.35a8,8,0,0,1,11.32,11.32L179.31,112Zm-80-20.68L99.31,112l10.35,10.34a8,8,0,0,1-11.32,11.32L88,123.31,77.66,133.66a8,8,0,0,1-11.32-11.32L76.69,112,66.34,101.66A8,8,0,0,1,77.66,90.34L88,100.69,98.34,90.34a8,8,0,0,1,11.32,11.32ZM140,180a12,12,0,1,1-12-12A12,12,0,0,1,140,180Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.13,104.13,0,0,0,128,24Zm-18.34,98.34a8,8,0,0,1-11.32,11.32L88,123.31,77.66,133.66a8,8,0,0,1-11.32-11.32L76.69,112,66.34,101.66A8,8,0,0,1,77.66,90.34L88,100.69,98.34,90.34a8,8,0,0,1,11.32,11.32L99.31,112ZM128,192a12,12,0,1,1,12-12A12,12,0,0,1,128,192Zm61.66-69.66a8,8,0,0,1-11.32,11.32L168,123.31l-10.34,10.35a8,8,0,0,1-11.32-11.32L156.69,112l-10.35-10.34a8,8,0,0,1,11.32-11.32L168,100.69l10.34-10.35a8,8,0,0,1,11.32,11.32L179.31,112Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm60.24-117.76L176.49,112l11.75,11.76a6,6,0,1,1-8.48,8.48L168,120.49l-11.76,11.75a6,6,0,0,1-8.48-8.48L159.51,112l-11.75-11.76a6,6,0,0,1,8.48-8.48L168,103.51l11.76-11.75a6,6,0,0,1,8.48,8.48Zm-80,0L96.49,112l11.75,11.76a6,6,0,1,1-8.48,8.48L88,120.49,76.24,132.24a6,6,0,0,1-8.48-8.48L79.51,112,67.76,100.24a6,6,0,0,1,8.48-8.48L88,103.51,99.76,91.76a6,6,0,0,1,8.48,8.48ZM138,180a10,10,0,1,1-10-10A10,10,0,0,1,138,180Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm61.66-93.66a8,8,0,0,1-11.32,11.32L168,123.31l-10.34,10.35a8,8,0,0,1-11.32-11.32L156.69,112l-10.35-10.34a8,8,0,0,1,11.32-11.32L168,100.69l10.34-10.35a8,8,0,0,1,11.32,11.32L179.31,112Zm-80-20.68L99.31,112l10.35,10.34a8,8,0,0,1-11.32,11.32L88,123.31,77.66,133.66a8,8,0,0,1-11.32-11.32L76.69,112,66.34,101.66A8,8,0,0,1,77.66,90.34L88,100.69,98.34,90.34a8,8,0,0,1,11.32,11.32ZM140,180a12,12,0,1,1-12-12A12,12,0,0,1,140,180Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220ZM186.83,98.83,173.66,112l13.17,13.17a4,4,0,0,1-5.66,5.66L168,117.66l-13.17,13.17a4,4,0,0,1-5.66-5.66L162.34,112,149.17,98.83a4,4,0,0,1,5.66-5.66L168,106.34l13.17-13.17a4,4,0,1,1,5.66,5.66Zm-80,0L93.66,112l13.17,13.17a4,4,0,0,1-5.66,5.66L88,117.66,74.83,130.83a4,4,0,0,1-5.66-5.66L82.34,112,69.17,98.83a4,4,0,0,1,5.66-5.66L88,106.34l13.17-13.17a4,4,0,0,1,5.66,5.66ZM136,180a8,8,0,1,1-8-8A8,8,0,0,1,136,180Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
