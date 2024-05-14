//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="games", feature ="health", feature ="objects"))]
#[component]
pub fn Volleyball(
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
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm87.63,96H181.37a104.18,104.18,0,0,0-35.78-78.23A88.18,88.18,0,0,1,215.63,120ZM44.53,155.87A87.95,87.95,0,0,1,77.27,56.13L94.39,85.78a104.14,104.14,0,0,0-49.86,70.09ZM58.9,182.43a88,88,0,0,1,43.49-82.79L118.76,128,77.27,199.87A88.62,88.62,0,0,1,58.9,182.43Zm150.84-21.85a88,88,0,0,1-93.49,3.78L132.62,136h83A87.16,87.16,0,0,1,209.74,160.58Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216.25,165.8A96,96,0,0,1,80,211.15h0a95.75,95.75,0,0,1-28.86-25.58h0A96,96,0,0,1,105.47,89L80,44.86a95.55,95.55,0,0,1,36.58-12.2h0A96,96,0,0,1,173.06,128H128l-22.53,39a96,96,0,0,0,110.78-1.22Z"
        opacity="0.2"
    ></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm81.74,136.58a88,88,0,0,1-93.49,3.78L132.62,136h83A87.16,87.16,0,0,1,209.74,160.58ZM91.12,48.11a87.57,87.57,0,0,1,24.22-7.2,88,88,0,0,1,50,79.09H132.62ZM215.63,120H181.37a104.18,104.18,0,0,0-35.78-78.23A88.18,88.18,0,0,1,215.63,120ZM77.27,56.13,94.39,85.78a104.14,104.14,0,0,0-49.86,70.09A87.95,87.95,0,0,1,77.27,56.13ZM58.9,182.43a88,88,0,0,1,43.49-82.79L118.76,128,77.27,199.87A88.62,88.62,0,0,1,58.9,182.43ZM128,216a87.5,87.5,0,0,1-36.88-8.11l17.13-29.67a104.23,104.23,0,0,0,85.53,8.17A87.81,87.81,0,0,1,128,216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm85,135.19a92,92,0,0,1-102.18,2.57L130.31,132h89.6A91.61,91.61,0,0,1,213,163.19ZM85.52,46.42A91.11,91.11,0,0,1,116,36.79,92,92,0,0,1,169.29,124h-39ZM219.91,124H177.29a100.06,100.06,0,0,0-46-87.93A92.11,92.11,0,0,1,219.91,124ZM78.59,50.42l21.3,36.89a100.09,100.09,0,0,0-53.16,83.77A91.92,91.92,0,0,1,78.59,50.42ZM55,183.94a92,92,0,0,1,48.87-89.7L123.38,128,78.59,205.58A92.75,92.75,0,0,1,55,183.94ZM128,220a91.37,91.37,0,0,1-42.48-10.42l21.3-36.89a100.07,100.07,0,0,0,99.1,4.16A92,92,0,0,1,128,220Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20ZM96.83,50a83.49,83.49,0,0,1,17.92-5A84,84,0,0,1,161.26,116H134.93Zm38.1,90h76.2a83.37,83.37,0,0,1-4.69,18,84.07,84.07,0,0,1-84.68,4.79Zm76.2-24H185.29a107.43,107.43,0,0,0-14.4-49.71A108.71,108.71,0,0,0,159.5,50.15,84.21,84.21,0,0,1,211.13,116ZM76.07,62.05,89,84.39a107.44,107.44,0,0,0-35.85,37.32,108.9,108.9,0,0,0-8.28,18A83.65,83.65,0,0,1,76.07,62.05ZM62.79,180.87A84,84,0,0,1,101,105.2L114.14,128,76.07,194A84.68,84.68,0,0,1,62.79,180.87Zm34,25.11,12.92-22.37A107.41,107.41,0,0,0,160,196a109,109,0,0,0,19.59-1.78A83.72,83.72,0,0,1,96.83,206Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm83.37,135.89a90,90,0,0,1-97.85,3.18L131.46,134H217.8A89.49,89.49,0,0,1,211.37,161.89ZM88.3,47.24a89.54,89.54,0,0,1,27.35-8.39A90,90,0,0,1,167.34,122H131.46ZM217.8,122H179.34A102.12,102.12,0,0,0,138.5,38.62,90.15,90.15,0,0,1,217.8,122ZM77.92,53.26,97.13,86.53a102.16,102.16,0,0,0-51.79,77.06A89.93,89.93,0,0,1,77.92,53.26ZM57,183.19a90,90,0,0,1,46.17-86.26L121.07,128,77.92,202.74A90.59,90.59,0,0,1,57,183.19ZM128,218a89.5,89.5,0,0,1-39.7-9.24l19.22-33.29a102.13,102.13,0,0,0,92.58,6.34A89.91,89.91,0,0,1,128,218Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm81.74,136.58a88,88,0,0,1-93.49,3.78L132.62,136h83A87.16,87.16,0,0,1,209.74,160.58ZM91.12,48.11a87.57,87.57,0,0,1,24.22-7.2,88,88,0,0,1,50,79.09H132.62ZM215.63,120H181.37a104.18,104.18,0,0,0-35.78-78.23A88.18,88.18,0,0,1,215.63,120ZM77.27,56.13,94.39,85.78a104.14,104.14,0,0,0-49.86,70.09A87.95,87.95,0,0,1,77.27,56.13ZM58.9,182.43a88,88,0,0,1,43.49-82.79L118.76,128,77.27,199.87A88.62,88.62,0,0,1,58.9,182.43ZM128,216a87.5,87.5,0,0,1-36.88-8.11l17.13-29.67a104.23,104.23,0,0,0,85.53,8.17A87.81,87.81,0,0,1,128,216Z"></path>
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