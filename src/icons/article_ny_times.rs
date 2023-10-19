/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArticleNyTimes(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M248,96v16a8,8,0,0,1-8,8H160a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8h80A8,8,0,0,1,248,96Zm-8,40H160a8,8,0,0,0-8,8v16a8,8,0,0,0,8,8h80a8,8,0,0,0,8-8V144A8,8,0,0,0,240,136Zm0,48H72a8,8,0,0,0-8,8v16a8,8,0,0,0,8,8H240a8,8,0,0,0,8-8V192A8,8,0,0,0,240,184ZM80,168a56.06,56.06,0,0,1-56-56,55.49,55.49,0,0,1,4.11-21A28,28,0,0,1,44,40a8,8,0,0,1,3.89,1l69.93,38.85A12,12,0,0,0,116,56a8,8,0,0,1,0-16,28,28,0,0,1,0,56,8,8,0,0,1-3.89-1l-40-22.21A40,40,0,0,0,72,151.2V112a8,8,0,0,1,16,0v39.19a40.09,40.09,0,0,0,29.73-25.86,8,8,0,0,1,15.08,5.34A56.09,56.09,0,0,1,80,168ZM36.23,77.14a56.33,56.33,0,0,1,17.5-14.58L42.18,56.14a12,12,0,0,0-6,21Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,104v96H160V104Z" opacity="0.2"/><path d="M152,104a8,8,0,0,1,8-8h80a8,8,0,0,1,0,16H160A8,8,0,0,1,152,104Zm88,24H160a8,8,0,0,0,0,16h80a8,8,0,0,0,0-16Zm0,32H160a8,8,0,0,0,0,16h80a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM80,176a56.06,56.06,0,0,1-56-56,55.49,55.49,0,0,1,4.11-21A28,28,0,0,1,44,48a8,8,0,0,1,3.89,1l69.93,38.85A12,12,0,0,0,116,64a8,8,0,0,1,0-16,28,28,0,0,1,0,56,8,8,0,0,1-3.89-1l-40-22.21A40,40,0,0,0,72,159.2V120a8,8,0,0,1,16,0v39.19a40.09,40.09,0,0,0,29.73-25.86,8,8,0,0,1,15.08,5.34A56.09,56.09,0,0,1,80,176ZM36.23,85.14a56.33,56.33,0,0,1,17.5-14.58L42.18,64.14a12,12,0,0,0-6,21Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M156,104a4,4,0,0,1,4-4h80a4,4,0,0,1,0,8H160A4,4,0,0,1,156,104Zm84,28H160a4,4,0,0,0,0,8h80a4,4,0,0,0,0-8Zm0,32H160a4,4,0,0,0,0,8h80a4,4,0,0,0,0-8Zm0,32H72a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8ZM80,172A51.94,51.94,0,0,1,33.2,97.41,24,24,0,0,1,44,52a4,4,0,0,1,1.94.5L117,92a16,16,0,0,0-1-32,4,4,0,0,1,0-8,24,24,0,0,1,0,48,4,4,0,0,1-1.94-.5L72.83,76.59A44,44,0,0,0,76,163.81V120a4,4,0,0,1,8,0v43.81a44.1,44.1,0,0,0,37.5-29.14,4,4,0,0,1,7.54,2.66A52.09,52.09,0,0,1,80,172ZM37.24,90.5A52.1,52.1,0,0,1,62.7,71L43,60A16,16,0,0,0,37.24,90.5Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M152,124a12,12,0,0,1,12-12h76a12,12,0,0,1,0,24H164A12,12,0,0,1,152,124Zm88,28H164a12,12,0,0,0,0,24h76a12,12,0,0,0,0-24Zm0,40H72a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24ZM80,180a60.07,60.07,0,0,1-60-60,59.42,59.42,0,0,1,3.32-19.59A32,32,0,0,1,44,44a12,12,0,0,1,5.83,1.51l68.6,38.11A8,8,0,0,0,116,68a12,12,0,0,1,0-24,32,32,0,0,1,0,64,12,12,0,0,1-5.83-1.51L71.51,85A36,36,0,0,0,68,153.94V124a12,12,0,0,1,24,0v29.91A36,36,0,0,0,114,132a12,12,0,0,1,22.63,8A60.1,60.1,0,0,1,80,180ZM36.49,78.76a60,60,0,0,1,9.33-8l-4.25-2.36A8,8,0,0,0,36,76,8.22,8.22,0,0,0,36.49,78.76Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M154,104a6,6,0,0,1,6-6h80a6,6,0,0,1,0,12H160A6,6,0,0,1,154,104Zm86,26H160a6,6,0,0,0,0,12h80a6,6,0,0,0,0-12Zm0,32H160a6,6,0,0,0,0,12h80a6,6,0,0,0,0-12Zm0,32H72a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12ZM80,174A53.94,53.94,0,0,1,30.6,98.26,26,26,0,0,1,44,50a6,6,0,0,1,2.91.75l70.52,39.18A14,14,0,0,0,116,62a6,6,0,0,1,0-12,26,26,0,0,1,0,52,6,6,0,0,1-2.91-.75L72.46,78.67A42,42,0,0,0,74,161.56V120a6,6,0,0,1,12,0v41.56A42.06,42.06,0,0,0,119.61,134a6,6,0,0,1,11.32,4A54.11,54.11,0,0,1,80,174ZM36.64,87.9A54.29,54.29,0,0,1,58.05,70.67l-15.48-8.6A14,14,0,0,0,36.64,87.9Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M152,104a8,8,0,0,1,8-8h80a8,8,0,0,1,0,16H160A8,8,0,0,1,152,104Zm88,24H160a8,8,0,0,0,0,16h80a8,8,0,0,0,0-16Zm0,32H160a8,8,0,0,0,0,16h80a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM80,176a56.06,56.06,0,0,1-56-56,55.49,55.49,0,0,1,4.11-21A28,28,0,0,1,44,48a8,8,0,0,1,3.89,1l69.93,38.85A12,12,0,0,0,116,64a8,8,0,0,1,0-16,28,28,0,0,1,0,56,8,8,0,0,1-3.89-1l-40-22.21A40,40,0,0,0,72,159.2V120a8,8,0,0,1,16,0v39.19a40.09,40.09,0,0,0,29.73-25.86,8,8,0,0,1,15.08,5.34A56.09,56.09,0,0,1,80,176ZM36.23,85.14a56.33,56.33,0,0,1,17.5-14.58L42.18,64.14a12,12,0,0,0-6,21Z"/> }.into_view()
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