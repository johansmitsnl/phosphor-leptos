//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="people"))]
#[component]
pub fn UserCircleGear(
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
                <path d="M228.25,63.07l-4.66-2.69a23.6,23.6,0,0,0,0-8.76l4.66-2.69a8,8,0,0,0-8-13.86l-4.67,2.7A23.92,23.92,0,0,0,208,33.38V28a8,8,0,0,0-16,0v5.38a23.92,23.92,0,0,0-7.58,4.39l-4.67-2.7a8,8,0,1,0-8,13.86l4.66,2.69a23.6,23.6,0,0,0,0,8.76l-4.66,2.69a8,8,0,0,0,4,14.93,7.92,7.92,0,0,0,4-1.07l4.67-2.7A23.92,23.92,0,0,0,192,78.62V84a8,8,0,0,0,16,0V78.62a23.92,23.92,0,0,0,7.58-4.39l4.67,2.7a7.92,7.92,0,0,0,4,1.07,8,8,0,0,0,4-14.93ZM200,64a8,8,0,1,1,8-8A8,8,0,0,1,200,64ZM128,76a44,44,0,1,1-44,44A44,44,0,0,1,128,76Zm102.56,34.68a103.92,103.92,0,1,1-85.24-85.24,8,8,0,0,1-2.64,15.78A88.07,88.07,0,0,0,40,128a87.62,87.62,0,0,0,22.24,58.41A79.71,79.71,0,0,1,84,165.1a4,4,0,0,1,4.83.32,59.81,59.81,0,0,0,78.27,0,4,4,0,0,1,4.84-.32,79.86,79.86,0,0,1,21.79,21.31A87.62,87.62,0,0,0,216,128a88.85,88.85,0,0,0-1.22-14.68,8,8,0,1,1,15.78-2.64Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,128a95.76,95.76,0,0,1-31.8,71.37A72,72,0,0,0,128,160a40,40,0,1,0-40-40,40,40,0,0,0,40,40,72,72,0,0,0-64.2,39.37h0A96,96,0,0,1,184.92,50.69a16,16,0,0,0,20.39,20.39A95.61,95.61,0,0,1,224,128Z"
        opacity="0.2"
    ></path>
    <path d="M228.25,63.07l-4.66-2.69a23.6,23.6,0,0,0,0-8.76l4.66-2.69a8,8,0,0,0-8-13.86l-4.67,2.7A23.92,23.92,0,0,0,208,33.38V28a8,8,0,0,0-16,0v5.38a23.92,23.92,0,0,0-7.58,4.39l-4.67-2.7a8,8,0,1,0-8,13.86l4.66,2.69a23.6,23.6,0,0,0,0,8.76l-4.66,2.69a8,8,0,0,0,4,14.93,7.92,7.92,0,0,0,4-1.07l4.67-2.7A23.92,23.92,0,0,0,192,78.62V84a8,8,0,0,0,16,0V78.62a23.92,23.92,0,0,0,7.58-4.39l4.67,2.7a7.92,7.92,0,0,0,4,1.07,8,8,0,0,0,4-14.93ZM192,56a8,8,0,1,1,8,8A8,8,0,0,1,192,56Zm29.35,48.11a8,8,0,0,0-6.57,9.21A88.85,88.85,0,0,1,216,128a87.62,87.62,0,0,1-22.24,58.41,79.66,79.66,0,0,0-36.06-28.75,48,48,0,1,0-59.4,0,79.66,79.66,0,0,0-36.06,28.75A88,88,0,0,1,128,40a88.76,88.76,0,0,1,14.68,1.22,8,8,0,0,0,2.64-15.78,103.92,103.92,0,1,0,85.24,85.24A8,8,0,0,0,221.35,104.11ZM96,120a32,32,0,1,1,32,32A32,32,0,0,1,96,120ZM74.08,197.5a64,64,0,0,1,107.84,0,87.83,87.83,0,0,1-107.84,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M226.25,66.54,219,62.33a19.78,19.78,0,0,0,0-12.66l7.29-4.21a4,4,0,0,0-4-6.92l-7.31,4.21A20,20,0,0,0,204,36.4V28a4,4,0,0,0-8,0v8.4a20,20,0,0,0-10.94,6.35l-7.31-4.21a4,4,0,1,0-4,6.92L181,49.67a19.78,19.78,0,0,0,0,12.66l-7.29,4.21a4,4,0,0,0,2,7.46,3.92,3.92,0,0,0,2-.54l7.31-4.21A20,20,0,0,0,196,75.6V84a4,4,0,0,0,8,0V75.6a20,20,0,0,0,10.94-6.35l7.31,4.21a3.92,3.92,0,0,0,2,.54,4,4,0,0,0,2-7.46ZM200,68a12,12,0,1,1,12-12A12,12,0,0,1,200,68Zm22,40.06a4,4,0,0,0-3.28,4.6A93.58,93.58,0,0,1,220,128a91.69,91.69,0,0,1-26.83,64.87,75.61,75.61,0,0,0-44.51-34,44,44,0,1,0-41.32,0,75.61,75.61,0,0,0-44.51,34A92,92,0,0,1,128,36a93.4,93.4,0,0,1,15.34,1.27,4,4,0,0,0,1.32-7.89,99.89,99.89,0,1,0,82,82A4,4,0,0,0,222,108.06ZM92,120a36,36,0,1,1,36,36A36,36,0,0,1,92,120ZM68.87,198.42a68,68,0,0,1,118.27,0,91.81,91.81,0,0,1-118.27,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M169.57,46.11a12,12,0,0,1,15.12-7.7L188,39.48V36a12,12,0,0,1,24,0v3.48l3.31-1.07a12,12,0,1,1,7.42,22.82l-3.31,1.08,2,2.82a12,12,0,1,1-19.41,14.1L200,76.42,198,79.23a12,12,0,1,1-19.41-14.1l2-2.82-3.31-1.08A12,12,0,0,1,169.57,46.11ZM236,128A108,108,0,1,1,128,20a109.19,109.19,0,0,1,18,1.49,12,12,0,0,1-4,23.67A85,85,0,0,0,128,44,83.94,83.94,0,0,0,62.05,179.94a83.48,83.48,0,0,1,29-23.42,52,52,0,1,1,74,0,83.48,83.48,0,0,1,29,23.42A83.57,83.57,0,0,0,212,128a85.2,85.2,0,0,0-1.16-14,12,12,0,0,1,23.67-4A109,109,0,0,1,236,128ZM128,148a28,28,0,1,0-28-28A28,28,0,0,0,128,148Zm0,64a83.53,83.53,0,0,0,48.43-15.43,60,60,0,0,0-96.86,0A83.53,83.53,0,0,0,128,212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M227.25,64.8l-5.92-3.41a22,22,0,0,0,0-10.78l5.92-3.41a6,6,0,0,0-6-10.4l-5.93,3.43A22,22,0,0,0,206,34.84V28a6,6,0,0,0-12,0v6.84a22,22,0,0,0-9.32,5.39l-5.93-3.43a6,6,0,0,0-6,10.4l5.92,3.41a22,22,0,0,0,0,10.78l-5.92,3.41a6,6,0,0,0,6,10.4l5.93-3.43A22,22,0,0,0,194,77.16V84a6,6,0,0,0,12,0V77.16a22,22,0,0,0,9.32-5.39l5.93,3.43a6,6,0,0,0,6-10.4ZM200,66a10,10,0,1,1,10-10A10,10,0,0,1,200,66Zm21.68,40.08a6,6,0,0,0-4.92,6.91A91.76,91.76,0,0,1,218,128a89.65,89.65,0,0,1-24.49,61.64,77.53,77.53,0,0,0-40-31.38,46,46,0,1,0-51,0,77.53,77.53,0,0,0-40,31.38A89.95,89.95,0,0,1,128,38a91.66,91.66,0,0,1,15,1.24,6,6,0,1,0,2-11.83,102,102,0,1,0,50,177.44c.65,1.36,1.68,0,2.34-2.11A102,102,0,0,0,228.59,111,6,6,0,0,0,221.68,106.08ZM94,120a34,34,0,1,1,34,34A34,34,0,0,1,94,120ZM71.44,198a66,66,0,0,1,113.12,0,89.8,89.8,0,0,1-113.12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M228.25,63.07l-4.66-2.69a23.6,23.6,0,0,0,0-8.76l4.66-2.69a8,8,0,0,0-8-13.86l-4.67,2.7A23.92,23.92,0,0,0,208,33.38V28a8,8,0,0,0-16,0v5.38a23.92,23.92,0,0,0-7.58,4.39l-4.67-2.7a8,8,0,1,0-8,13.86l4.66,2.69a23.6,23.6,0,0,0,0,8.76l-4.66,2.69a8,8,0,0,0,4,14.93,7.92,7.92,0,0,0,4-1.07l4.67-2.7A23.92,23.92,0,0,0,192,78.62V84a8,8,0,0,0,16,0V78.62a23.92,23.92,0,0,0,7.58-4.39l4.67,2.7a7.92,7.92,0,0,0,4,1.07,8,8,0,0,0,4-14.93ZM192,56a8,8,0,1,1,8,8A8,8,0,0,1,192,56Zm29.35,48.11a8,8,0,0,0-6.57,9.21A88.85,88.85,0,0,1,216,128a87.62,87.62,0,0,1-22.24,58.41,79.66,79.66,0,0,0-36.06-28.75,48,48,0,1,0-59.4,0,79.66,79.66,0,0,0-36.06,28.75A88,88,0,0,1,128,40a88.76,88.76,0,0,1,14.68,1.22,8,8,0,0,0,2.64-15.78,103.92,103.92,0,1,0,85.24,85.24A8,8,0,0,0,221.35,104.11ZM96,120a32,32,0,1,1,32,32A32,32,0,0,1,96,120ZM74.08,197.5a64,64,0,0,1,107.84,0,87.83,87.83,0,0,1-107.84,0Z"></path>
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