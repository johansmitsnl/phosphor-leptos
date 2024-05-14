//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="nature", feature ="commerce"))]
#[component]
pub fn FishSimple(
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
                <path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm42,79.08c-15.08,20.84-37.53,34.88-66.7,41.74-20.08,4.72-43.54,6-70.12,3.93q2.4,17.82,6.72,37.54a8,8,0,0,1-6.1,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.81-6.29q-4.89-22.36-7.41-42.62-20.22-2.51-42.58-7.41a8,8,0,0,1,3.43-15.63q19.7,4.32,37.5,6.73c-2.09-26.56-.78-50,3.93-70.06C66,83.55,80.05,61.1,100.88,46,115,35.76,140.14,23.64,179.27,24c21.19.21,40.83,4.33,43.81,6.08a8,8,0,0,1,2.83,2.83c1.75,3,5.87,22.59,6.08,43.78C232.21,98.31,228.57,129.44,210,155.08Zm2.43-111.52a175.75,175.75,0,0,0-39.22-3.51c-24.34.64-44.71,6.49-60.76,17.39a96,96,0,0,0,86.09,86.1c10.91-16,16.76-36.42,17.4-60.76A175.82,175.82,0,0,0,212.44,43.56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M202.43,151.85c-21.26,28.17-62.09,48.24-138.35,40C55.89,115.64,76,74.82,104.15,53.57A104,104,0,0,0,202.43,151.85Z"
        opacity="0.2"
    ></path>
    <path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm42,79.08c-15.08,20.84-37.53,34.88-66.7,41.74-20.08,4.72-43.54,6-70.12,3.93q2.4,17.82,6.72,37.54a8,8,0,0,1-6.1,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.81-6.29q-4.89-22.36-7.41-42.62-20.22-2.51-42.58-7.41a8,8,0,0,1,3.43-15.63q19.7,4.32,37.5,6.73c-2.09-26.56-.78-50,3.93-70.06C66,83.55,80.05,61.1,100.88,46,115,35.76,140.12,23.64,179.27,24c21.19.21,40.83,4.33,43.81,6.08a8,8,0,0,1,2.83,2.83c1.75,3,5.87,22.59,6.08,43.78C232.21,98.31,228.57,129.44,210,155.08Zm-23.76,2.8A112.07,112.07,0,0,1,98.12,69.74C75.64,94,66.7,132.47,71.36,184.6,123.51,189.28,162,180.35,186.25,157.88ZM212.44,43.56a175.75,175.75,0,0,0-39.22-3.51c-24.34.64-44.71,6.49-60.76,17.39a96,96,0,0,0,86.09,86.1c10.91-16,16.76-36.42,17.4-60.76A175.82,175.82,0,0,0,212.44,43.56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,76a8,8,0,1,1-8-8A8,8,0,0,1,164,76Zm42.77,76.73c-14.5,20-36.17,33.57-64.38,40.2-20.91,4.91-45.61,6-73.8,3.42q2.46,20.18,7.41,42.79a4,4,0,0,1-3,4.77,4.19,4.19,0,0,1-.86.09,4,4,0,0,1-3.9-3.14,457,457,0,0,1-7.72-45.36q-21.4-2.49-45.33-7.72A4,4,0,1,1,16.86,180q22.59,4.95,42.76,7.41c-2.62-28.16-1.48-52.84,3.43-73.73,6.63-28.21,20.14-49.88,40.18-64.39C127.32,31.8,156.61,28,177.53,28h1.71c22,.21,40.12,4.54,41.81,5.53A4,4,0,0,1,222.46,35c1,1.68,5.31,19.8,5.53,41.79C228.2,97.75,224.69,128,206.77,152.73Zm-12.23,2.43a108,108,0,0,1-93.7-93.7c-27.92,25.08-39,67.62-33.14,126.81C126.9,194.14,169.45,183.07,194.54,155.16Zm21.2-114.9C204.9,37.73,148.5,26.69,108.3,55.46a99.92,99.92,0,0,0,92.23,92.24C229.29,107.52,218.27,51.11,215.74,40.26Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M172,76a16,16,0,1,1-16-16A16,16,0,0,1,172,76Zm64,.66c.32,31.85-7.55,59.77-22.74,80.76-15.66,21.65-38.89,36.21-69,43.29-19.2,4.52-41.39,6-66.36,4.38q2.27,15.49,6,32.34a12,12,0,1,1-23.44,5.14q-4.58-20.85-7.08-39.91-19-2.5-39.87-7.07a12,12,0,0,1,5.14-23.44q16.83,3.67,32.31,6c-1.58-25-.13-47.12,4.38-66.3,7.08-30.14,21.64-53.36,43.27-69,21-15.21,48.92-23.1,80.78-22.77,21.79.21,42,4.42,45.8,6.64a12,12,0,0,1,4.24,4.23C231.57,34.64,235.78,54.87,236,76.66Zm-58.46,83.28A116.08,116.08,0,0,1,96.06,78.46c-9.21,12.16-15.56,27.57-18.93,46-3,16.39-3.66,35.25-2,56.39,21.16,1.65,40,1,56.42-2C150,175.49,165.37,169.14,177.53,159.94ZM209,47c-15.9-3-59.3-8.45-92.26,12.44a92,92,0,0,0,79.81,79.82C217.43,106.32,212,62.92,209,47Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M166,76a10,10,0,1,1-10-10A10,10,0,0,1,166,76Zm42.38,77.9c-14.79,20.44-36.84,34.22-65.53,41-20.5,4.81-44.58,6-72,3.68q2.43,19,7.07,40.15a6,6,0,0,1-4.58,7.15,6.29,6.29,0,0,1-1.29.14,6,6,0,0,1-5.85-4.72q-5.07-23.13-7.57-44-20.8-2.51-43.94-7.57A6,6,0,1,1,17.29,178q21.14,4.63,40.12,7.07c-2.35-27.36-1.13-51.41,3.69-71.9C67.84,84.49,81.61,62.44,102,47.64c47.55-34.44,116.31-18,120-15.81a5.93,5.93,0,0,1,2.11,2.11C226.34,37.63,242.81,106.33,208.37,153.91Zm-17.94,2.69a110.06,110.06,0,0,1-91-91c-25.13,24.7-35.12,65.24-29.87,120.89C125.19,191.71,165.72,181.73,190.43,156.6ZM214.06,41.93c-12.78-2.84-65.42-12.17-103.69,14.52a97.92,97.92,0,0,0,89.16,89.17C226.21,107.37,216.89,54.72,214.06,41.93Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm42,79.08c-15.08,20.84-37.53,34.88-66.7,41.74-20.08,4.72-43.54,6-70.12,3.93q2.4,17.82,6.72,37.54a8,8,0,0,1-6.1,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.81-6.29q-4.89-22.36-7.41-42.62-20.22-2.51-42.58-7.41a8,8,0,0,1,3.43-15.63q19.7,4.32,37.5,6.73c-2.09-26.56-.78-50,3.93-70.06C66,83.55,80.05,61.1,100.88,46,115,35.76,140.15,23.64,179.27,24c21.19.21,40.83,4.33,43.81,6.08a8,8,0,0,1,2.83,2.83c1.75,3,5.87,22.59,6.08,43.78C232.21,98.31,228.57,129.44,210,155.08Zm-23.76,2.8A112.07,112.07,0,0,1,98.12,69.74C75.64,94,66.7,132.47,71.36,184.6,123.51,189.28,162,180.35,186.25,157.88ZM212.44,43.56a175.75,175.75,0,0,0-39.22-3.51c-24.34.64-44.71,6.49-60.76,17.39a96,96,0,0,0,86.09,86.1c10.91-16,16.76-36.42,17.4-60.76A175.82,175.82,0,0,0,212.44,43.56Z"></path>
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