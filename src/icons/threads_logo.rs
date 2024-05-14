//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "communication"))]
#[component]
pub fn ThreadsLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M138.62,128a53.54,53.54,0,0,1,13.1,1.63c-.57,8.21-3.34,15-8.11,19.61A23.89,23.89,0,0,1,127,156c-11.87,0-15-7.58-15-12.07C112,133,125.8,128,138.62,128ZM224,128c0,65.12-35.89,104-96,104S32,193.12,32,128,67.89,24,128,24,224,62.88,224,128ZM72,128c0-43.07,18.32-64,56-64,26.34,0,43,10.08,50.81,30.83a8,8,0,0,0,15-5.66C180.9,55.14,150.9,48,128,48c-26.1,0-45.52,8.7-57.72,25.86C60.8,87.19,56,105.4,56,128s4.8,40.81,14.28,54.14C82.48,199.3,101.9,208,128,208c24.45,0,39.82-8.8,48.41-16.18,10.76-9.25,17.19-21.89,17.19-33.82,0-14.3-6.59-26.79-18.56-35.17a54.16,54.16,0,0,0-7.77-4.5c-2.09-14.65-10-25.75-22.34-31.07C130.43,81,112,83.93,101.21,94.19a8,8,0,0,0,11,11.62c5.43-5.14,16.79-8,26.4-3.85a20.05,20.05,0,0,1,10.77,10.92,68.89,68.89,0,0,0-10.76-.85C113.53,112,96,125.15,96,143.93c0,16.27,13,28.07,31,28.07a40,40,0,0,0,27.75-11.29c4.7-4.59,10.11-12.2,12.17-24A25.55,25.55,0,0,1,177.6,158c0,13.71-15.76,34-49.6,34C90.32,192,72,171.07,72,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,128c0,48-16,96-80,96s-80-48-80-96,16-96,80-96S208,80,208,128Z"
        opacity="0.2"
    ></path>
    <path d="M186.42,123.65a63.81,63.81,0,0,0-11.13-6.72c-4-29.89-24-39.31-33.1-42.07-19.78-6-42.51,1.19-52.85,16.7a8,8,0,0,0,13.32,8.88c6.37-9.56,22-14.16,34.89-10.27,9.95,3,16.82,10.3,20.15,21a81.05,81.05,0,0,0-15.29-1.43c-13.92,0-26.95,3.59-36.67,10.1C94.3,127.57,88,139,88,152c0,20.58,15.86,35.52,37.71,35.52a48,48,0,0,0,34.35-14.81c6.44-6.7,14-18.36,15.61-37.1.38.26.74.53,1.1.8C186.88,144.05,192,154.68,192,168c0,19.36-20.34,48-64,48-26.73,0-45.48-8.65-57.34-26.44C60.93,175,56,154.26,56,128s4.93-47,14.66-61.56C82.52,48.65,101.27,40,128,40c32.93,0,54,13.25,64.53,40.52a8,8,0,1,0,14.93-5.75C194.68,41.56,167.2,24,128,24,96,24,72.19,35.29,57.34,57.56,45.83,74.83,40,98.52,40,128s5.83,53.17,17.34,70.44C72.19,220.71,96,232,128,232c30.07,0,48.9-11.48,59.4-21.1C200.3,199.08,208,183,208,168,208,149.66,200.54,134.32,186.42,123.65Zm-37.89,38a31.94,31.94,0,0,1-22.82,9.9c-10.81,0-21.71-6-21.71-19.52,0-12.63,12-26.21,38.41-26.21A63.88,63.88,0,0,1,160,128.24C160,142.32,156,153.86,148.53,161.62Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M184,126.84a59.8,59.8,0,0,0-12.42-7.16c-3-29.38-22-38.4-30.56-41-18.16-5.5-39,1-48.36,15.09a4,4,0,0,0,6.66,4.44c7.4-11.1,24.7-16.32,39.38-11.87,8.12,2.45,20.95,9.6,24.41,30.32a75.83,75.83,0,0,0-20.71-2.88c-13.14,0-25.37,3.34-34.44,9.43-10.45,7-16,17-16,28.78,0,20.7,17,31.52,33.71,31.52a44,44,0,0,0,31.47-13.58c9.56-9.94,14.68-24.19,14.82-41.23a50.18,50.18,0,0,1,7.19,4.51c11,8.32,16.81,20.34,16.81,34.78,0,11.73-6.25,24.46-16.7,34.05C170.36,210.24,154.21,220,128,220c-50.43,0-76-30.95-76-92s25.57-92,76-92c34.29,0,57.26,14.5,68.27,43.08a4,4,0,1,0,7.46-2.87C191.42,44.22,165.94,28,128,28,73.05,28,44,62.58,44,128s29.05,100,84,100c28.79,0,46.72-10.9,56.7-20.05,12.09-11.08,19.3-26,19.3-39.95C204,151,197.09,136.73,184,126.84Zm-32.6,37.55a35.92,35.92,0,0,1-25.7,11.13c-12.38,0-25.71-7.36-25.71-23.52,0-20.76,22-30.21,42.41-30.21A67.08,67.08,0,0,1,164,125.3c0,.88.05,1.78.05,2.7C164,143.25,159.65,155.83,151.41,164.39Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M188.84,120.46a68.14,68.14,0,0,0-10-6.23c-3.72-21.68-16.41-37.41-35.52-43.2C121.94,64.55,97.29,72.42,86,89.34a12,12,0,0,0,20,13.32c5.47-8.2,19.11-12.08,30.41-8.66a24.72,24.72,0,0,1,14.88,12.24,86.73,86.73,0,0,0-8.86-.45C108.56,105.79,84,125.22,84,152c0,22.9,17.54,39.52,41.71,39.52a52,52,0,0,0,37.23-16c6-6.23,12.88-16.46,15.72-32.07,6.2,6.42,9.34,14.67,9.34,24.59,0,17.74-19.07,44-60,44-45.76,0-68-27.48-68-84s22.24-84,68-84c31.08,0,51,12.42,60.8,38a12,12,0,0,0,22.4-8.62C197.77,38.44,169,20,128,20,68.67,20,36,58.35,36,128s32.67,108,92,108c31.36,0,51.08-12.05,62.11-22.15C203.81,201.28,212,184.14,212,168,212,148.36,204,131.92,188.84,120.46Zm-43.2,38.39a27.9,27.9,0,0,1-19.93,8.67c-8.17,0-17.71-4.06-17.71-15.52,0-15.26,17.84-22.21,34.41-22.21a60.23,60.23,0,0,1,13.51,1.52C155.36,142.93,151.84,152.41,145.64,158.85Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M185.22,125.25a62,62,0,0,0-11.78-7c-3.53-29.6-23-38.82-31.83-41.5-19-5.74-40.73,1.09-50.6,15.9a6,6,0,1,0,10,6.66c6.94-10.41,23.25-15.28,37.14-11.07,7.22,2.18,18.39,8.34,22.39,25.61a78.74,78.74,0,0,0-18.11-2.08c-13.53,0-26.16,3.46-35.55,9.77C96,128.85,90,139.66,90,152c0,22,18,33.52,35.71,33.52a46,46,0,0,0,32.91-14.19c6.58-6.85,14.35-19.11,15.29-39.26a44.59,44.59,0,0,1,4.07,2.75c10.48,7.92,16,19.4,16,33.18,0,20.16-21,50-66,50-27.07,0-46.92-9.19-59-27.33C59,175.75,54,154.66,54,128s5-47.75,15-62.67C81.08,47.19,100.93,38,128,38c33.85,0,55.57,13.67,66.4,41.8a6,6,0,1,0,11.2-4.31C193,42.65,166.85,26,128,26,96.67,26,73.46,37,59,58.67,47.72,75.6,42,98.93,42,128s5.72,52.4,17,69.33C73.46,219,96.67,230,128,230c29.43,0,47.81-11.19,58.05-20.58C198.54,198,206,182.49,206,168,206,150.31,198.81,135.52,185.22,125.25ZM150,163a33.94,33.94,0,0,1-24.26,10.51C109.33,173.52,102,162.71,102,152c0-13.59,12.64-28.21,40.41-28.21a65.33,65.33,0,0,1,19.58,3c0,.41,0,.82,0,1.24C162,142.72,157.84,154.82,150,163Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M186.42,123.65a63.81,63.81,0,0,0-11.13-6.72c-4-29.89-24-39.31-33.1-42.07-19.78-6-42.51,1.19-52.85,16.7a8,8,0,0,0,13.32,8.88c6.37-9.56,22-14.16,34.89-10.27,9.95,3,16.82,10.3,20.15,21a81.05,81.05,0,0,0-15.29-1.43c-13.92,0-26.95,3.59-36.67,10.1C94.3,127.57,88,139,88,152c0,20.58,15.86,35.52,37.71,35.52a48,48,0,0,0,34.35-14.81c6.44-6.7,14-18.36,15.61-37.1.38.26.74.53,1.1.8C186.88,144.05,192,154.68,192,168c0,19.36-20.34,48-64,48-26.73,0-45.48-8.65-57.34-26.44C60.93,175,56,154.26,56,128s4.93-47,14.66-61.56C82.52,48.65,101.27,40,128,40c32.93,0,54,13.25,64.53,40.52a8,8,0,1,0,14.93-5.75C194.68,41.56,167.2,24,128,24,96,24,72.19,35.29,57.34,57.56,45.83,74.83,40,98.52,40,128s5.83,53.17,17.34,70.44C72.19,220.71,96,232,128,232c30.07,0,48.9-11.48,59.4-21.1C200.3,199.08,208,183,208,168,208,149.66,200.54,134.32,186.42,123.65Zm-37.89,38a31.94,31.94,0,0,1-22.82,9.9c-10.81,0-21.71-6-21.71-19.52,0-12.63,12-26.21,38.41-26.21A63.88,63.88,0,0,1,160,128.24C160,142.32,156,153.86,148.53,161.62Z"></path>
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
