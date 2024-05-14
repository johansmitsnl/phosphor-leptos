//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design", feature ="editor"))]
#[component]
pub fn Unite(
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
                <path d="M240,164a76,76,0,0,1-151.9,3.9,76,76,0,1,1,79.8-79.8A76.1,76.1,0,0,1,240,164Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,160a72,72,0,0,1-143.6,7.6,72,72,0,1,1,79.2-79.2A72,72,0,0,1,232,160Z"
        opacity="0.2"
    ></path>
    <path d="M174.63,81.37a80,80,0,1,0-93.26,93.26,80,80,0,1,0,93.26-93.26ZM32,96a64,64,0,0,1,126-16A80.08,80.08,0,0,0,80.05,158,64.11,64.11,0,0,1,32,96Zm128,0a64.07,64.07,0,0,1-64,64A64.07,64.07,0,0,1,160,96Zm0,128A64.11,64.11,0,0,1,98,176,80.08,80.08,0,0,0,176,98,64,64,0,0,1,160,224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M171.17,84.83a76,76,0,1,0-86.34,86.34,76,76,0,1,0,86.34-86.34ZM228,160a68.63,68.63,0,0,1-1.27,13.07l-57.34-57.34A76,76,0,0,0,172,96c0-1,0-2-.07-2.93A68.1,68.1,0,0,1,228,160ZM45.2,50.86l58.34,58.33a76.35,76.35,0,0,0-14.09,22.6L31.67,74A68.14,68.14,0,0,1,45.2,50.86ZM74,31.67l57.78,57.78a76.35,76.35,0,0,0-22.6,14.09L50.86,45.2A68.14,68.14,0,0,1,74,31.67ZM164,96a67.59,67.59,0,0,1-7.45,30.89L129.11,99.45A67.59,67.59,0,0,1,160,92c1.3,0,2.6,0,3.88.12C164,93.4,164,94.7,164,96Zm-30,56.36L103.64,122A68.74,68.74,0,0,1,122,103.64L152.36,134A68.74,68.74,0,0,1,134,152.36ZM92,160a67.59,67.59,0,0,1,7.45-30.89l27.44,27.44A67.59,67.59,0,0,1,96,164c-1.3,0-2.6-.05-3.88-.12C92.05,162.6,92,161.3,92,160Zm54.81-7.54,58.33,58.34A68.14,68.14,0,0,1,182,224.33l-57.78-57.78A76.35,76.35,0,0,0,146.81,152.46Zm5.65-5.65a76.35,76.35,0,0,0,14.09-22.6L224.33,182a68.14,68.14,0,0,1-13.53,23.15Zm10.47-62.74c-1,0-2-.07-2.93-.07a76,76,0,0,0-19.73,2.61L82.93,29.27a68,68,0,0,1,80,54.8ZM28,96a68.63,68.63,0,0,1,1.27-13.07l57.34,57.34A76,76,0,0,0,84,160c0,1,0,2,.07,2.93A68.1,68.1,0,0,1,28,96Zm65.07,75.93c1,0,2,.07,2.93.07a76,76,0,0,0,19.73-2.61l57.34,57.34a68,68,0,0,1-80-54.8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M178,78A84,84,0,1,0,78,178,84,84,0,1,0,178,78Zm42,82a60.75,60.75,0,0,1-.38,6.65l-44-44a83.31,83.31,0,0,0,4-19.34A60.09,60.09,0,0,1,220,160Zm-74-30.94L126.94,110a59.57,59.57,0,0,1,28.9-9.81A59.57,59.57,0,0,1,146,129.06ZM110,126.94,129.06,146a59.57,59.57,0,0,1-28.9,9.81A59.57,59.57,0,0,1,110,126.94Zm42.7-50.6a83.31,83.31,0,0,0-19.34,4l-44-44A60.75,60.75,0,0,1,96,36,60.09,60.09,0,0,1,152.67,76.34ZM64.19,45.16l46.73,46.73a85,85,0,0,0-19,19L45.16,64.19A60.45,60.45,0,0,1,64.19,45.16ZM36,96a60.75,60.75,0,0,1,.38-6.65l44,44a83.31,83.31,0,0,0-4,19.34A60.09,60.09,0,0,1,36,96Zm67.33,83.66a83.31,83.31,0,0,0,19.34-4l44,44A60.75,60.75,0,0,1,160,220,60.09,60.09,0,0,1,103.33,179.66Zm88.48,31.18-46.73-46.73a85,85,0,0,0,19-19l46.73,46.73A60.45,60.45,0,0,1,191.81,210.84Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M172.91,83.09a78,78,0,1,0-89.82,89.82,78,78,0,1,0,89.82-89.82ZM226,160a65.31,65.31,0,0,1-.62,8.9l-53.76-53.77A77.84,77.84,0,0,0,174,96c0-.17,0-.33,0-.49A66.1,66.1,0,0,1,226,160ZM45.31,53.79l55.5,55.5a77.86,77.86,0,0,0-12,19L34,73.48A66,66,0,0,1,45.31,53.79Zm88.92,96-28-28a66.47,66.47,0,0,1,15.52-15.52l28,28A66.47,66.47,0,0,1,134.23,149.75ZM162,96a65.62,65.62,0,0,1-6,27.49L132.51,100A65.62,65.62,0,0,1,160,94c.65,0,1.3,0,1.95,0C162,94.7,162,95.35,162,96Zm-52.71,4.81-55.5-55.5A66,66,0,0,1,73.48,34l54.8,54.81A77.86,77.86,0,0,0,109.29,100.81ZM94,160a65.62,65.62,0,0,1,6-27.49L123.49,156A65.62,65.62,0,0,1,96,162c-.65,0-1.3,0-2-.05C94,161.3,94,160.65,94,160Zm52.71-4.81,55.5,55.5A66,66,0,0,1,182.52,222l-54.8-54.81A77.86,77.86,0,0,0,146.71,155.19Zm8.48-8.48a77.86,77.86,0,0,0,12-19L222,182.52a66,66,0,0,1-11.35,19.69Zm5.3-64.7H160a77.84,77.84,0,0,0-19.13,2.38L87.1,30.62A65.31,65.31,0,0,1,96,30,66.1,66.1,0,0,1,160.49,82ZM30,96a65.31,65.31,0,0,1,.62-8.9l53.76,53.77A77.84,77.84,0,0,0,82,160c0,.17,0,.33,0,.49A66.1,66.1,0,0,1,30,96Zm65.51,78H96a77.84,77.84,0,0,0,19.13-2.38l53.77,53.76a65.31,65.31,0,0,1-8.9.62A66.1,66.1,0,0,1,95.51,174Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M174.63,81.37a80,80,0,1,0-93.26,93.26,80,80,0,1,0,93.26-93.26ZM224,160c0,1.52-.07,3-.18,4.51l-50-50A80,80,0,0,0,176,98,64.11,64.11,0,0,1,224,160ZM45.47,56.79,98.09,109.4a80.5,80.5,0,0,0-9.93,15.44L36.3,73A64,64,0,0,1,45.47,56.79ZM73,36.3l51.86,51.86a80.5,80.5,0,0,0-15.44,9.93L56.79,45.47A64,64,0,0,1,73,36.3Zm61.46,110.83-25.57-25.57a64.65,64.65,0,0,1,12.69-12.69l25.57,25.57A64.65,64.65,0,0,1,134.44,147.13ZM155.31,120,136,100.69A63.48,63.48,0,0,1,160,96,63.48,63.48,0,0,1,155.31,120Zm-54.62,16L120,155.31A63.48,63.48,0,0,1,96,160,63.48,63.48,0,0,1,100.69,136Zm45.91,21.91,52.61,52.62A64,64,0,0,1,183,219.7l-51.86-51.86A80.5,80.5,0,0,0,146.6,157.91Zm11.31-11.31a80.5,80.5,0,0,0,9.93-15.44L219.7,183a64,64,0,0,1-9.17,16.19ZM158,80.05a80,80,0,0,0-16.49,2.13l-50-50C93,32.07,94.48,32,96,32A64.11,64.11,0,0,1,158,80.05ZM32,96c0-1.52.07-3,.18-4.51l50,50A80,80,0,0,0,80.05,158,64.11,64.11,0,0,1,32,96ZM98,176a80,80,0,0,0,16.49-2.13l50,50c-1.49.11-3,.18-4.51.18A64.11,64.11,0,0,1,98,176Z"></path>
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