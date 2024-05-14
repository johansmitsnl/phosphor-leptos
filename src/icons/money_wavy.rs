//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="finance", feature ="commerce"))]
#[component]
pub fn MoneyWavy(
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
                <path d="M244.24,60a8,8,0,0,0-7.75-.4c-42.93,21-73.59,11.16-106,.78C96.4,49.53,61.2,38.28,12.49,62.06A8,8,0,0,0,8,69.24V189.17a8,8,0,0,0,11.51,7.19c42.93-21,73.59-11.16,106.05-.78,19.24,6.15,38.84,12.42,61,12.42,17.09,0,35.73-3.72,56.91-14.06a8,8,0,0,0,4.49-7.18V66.83A8,8,0,0,0,244.24,60ZM48,152a8,8,0,0,1-16,0V88a8,8,0,0,1,16,0Zm80,8a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm96,8a8,8,0,0,1-16,0V104a8,8,0,0,1,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M16,69.21v120c91.64-44.77,132.36,42.35,224-2.42v-120C148.36,111.56,107.64,24.44,16,69.21ZM128,152a24,24,0,1,1,24-24A24,24,0,0,1,128,152Z"
        opacity="0.2"
    ></path>
    <path d="M244.24,60a8,8,0,0,0-7.75-.4c-42.93,21-73.59,11.16-106,.78-34-10.89-69.25-22.14-117.95,1.64A8,8,0,0,0,8,69.24V189.17a8,8,0,0,0,11.51,7.19c42.93-21,73.59-11.16,106.05-.78,19.24,6.15,38.84,12.42,61,12.42,17.09,0,35.73-3.72,56.91-14.06a8,8,0,0,0,4.49-7.18V66.83A8,8,0,0,0,244.24,60ZM232,181.67c-40.6,18.17-70.25,8.69-101.56-1.32-19.24-6.15-38.84-12.42-61-12.42a122,122,0,0,0-45.4,9V74.33c40.6-18.17,70.25-8.69,101.56,1.32S189.14,96,232,79.09ZM128,96a32,32,0,1,0,32,32A32,32,0,0,0,128,96Zm0,48a16,16,0,1,1,16-16A16,16,0,0,1,128,144ZM56,96v48a8,8,0,0,1-16,0V96a8,8,0,1,1,16,0Zm144,64V112a8,8,0,1,1,16,0v48a8,8,0,1,1-16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M242.12,63.39a4,4,0,0,0-3.88-.2c-44.37,21.68-75.77,11.64-109,1s-67.71-21.67-115,1.42A4,4,0,0,0,12,69.21v120a4,4,0,0,0,5.76,3.6c44.37-21.68,75.77-11.64,109-1,18.86,6,38.08,12.19,59.8,12.19,16.61,0,34.69-3.6,55.18-13.61a4,4,0,0,0,2.24-3.6v-120A4,4,0,0,0,242.12,63.39ZM236,184.27c-43.19,20.27-74.1,10.38-106.78-.08-18.86-6-38.08-12.18-59.8-12.18-15,0-31.28,3-49.42,10.94V71.73c43.19-20.27,74.1-10.38,106.78.08C158.7,82,191.67,92.57,236,73.05ZM128,100a28,28,0,1,0,28,28A28,28,0,0,0,128,100Zm0,48a20,20,0,1,1,20-20A20,20,0,0,1,128,148ZM52,96v48a4,4,0,0,1-8,0V96a4,4,0,0,1,8,0Zm152,64V112a4,4,0,0,1,8,0v48a4,4,0,0,1-8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M246.36,56.55a12,12,0,0,0-11.63-.6c-41.48,20.29-71.4,10.71-103.07.56C98.48,45.89,60.88,33.85,10.73,58.37A12,12,0,0,0,4,69.16v120.1a12,12,0,0,0,17.27,10.79c41.48-20.29,71.4-10.71,103.07-.56,18.83,6,39.08,12.51,62.24,12.51,17.66,0,37-3.77,58.69-14.37A12,12,0,0,0,252,186.84V66.74A12,12,0,0,0,246.36,56.55ZM228,179.12c-38,16.16-66.41,7.07-96.34-2.51-18.83-6-39.08-12.52-62.24-12.52A124.86,124.86,0,0,0,28,171.24V76.88c38-16.16,66.41-7.08,96.34,2.51C153.6,88.76,186.29,99.23,228,84.76ZM128,96a32,32,0,1,0,32,32A32.06,32.06,0,0,0,128,96Zm0,40a8,8,0,1,1,8-8A8,8,0,0,1,128,136ZM64,100v40a12,12,0,1,1-24,0V100a12,12,0,1,1,24,0Zm128,56V116a12,12,0,1,1,24,0v40a12,12,0,1,1-24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M243.18,61.72a6,6,0,0,0-5.81-.3c-43.66,21.32-74.69,11.39-107.54.88C96.16,51.53,61.35,40.4,13.37,63.84A6,6,0,0,0,10,69.23v120a6,6,0,0,0,8.63,5.39c43.66-21.32,74.69-11.39,107.54-.88,19,6.09,38.46,12.3,60.42,12.3,16.85,0,35.21-3.66,56-13.84a6,6,0,0,0,3.37-5.39v-120A6,6,0,0,0,243.18,61.72ZM234,183c-41.9,19.21-72.17,9.53-104.17-.71C110.78,176.18,91.37,170,69.41,170c-14.49,0-30.08,2.7-47.41,9.92V73c41.9-19.21,72.17-9.53,104.17.71C157.78,83.84,190.41,94.28,234,76.11ZM128,98a30,30,0,1,0,30,30A30,30,0,0,0,128,98Zm0,48a18,18,0,1,1,18-18A18,18,0,0,1,128,146ZM54,96v48a6,6,0,0,1-12,0V96a6,6,0,1,1,12,0Zm148,64V112a6,6,0,0,1,12,0v48a6,6,0,0,1-12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M244.24,60a8,8,0,0,0-7.75-.4c-42.93,21-73.59,11.16-106,.78-34-10.89-69.25-22.14-117.95,1.64A8,8,0,0,0,8,69.24V189.17a8,8,0,0,0,11.51,7.19c42.93-21,73.59-11.16,106.05-.78,19.24,6.15,38.84,12.42,61,12.42,17.09,0,35.73-3.72,56.91-14.06a8,8,0,0,0,4.49-7.18V66.83A8,8,0,0,0,244.24,60ZM232,181.67c-40.6,18.17-70.25,8.69-101.56-1.32-19.24-6.15-38.84-12.42-61-12.42a122,122,0,0,0-45.4,9V74.33c40.6-18.17,70.25-8.69,101.56,1.32S189.14,96,232,79.09ZM128,96a32,32,0,1,0,32,32A32,32,0,0,0,128,96Zm0,48a16,16,0,1,1,16-16A16,16,0,0,1,128,144ZM56,96v48a8,8,0,0,1-16,0V96a8,8,0,1,1,16,0Zm144,64V112a8,8,0,1,1,16,0v48a8,8,0,1,1-16,0Z"></path>
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