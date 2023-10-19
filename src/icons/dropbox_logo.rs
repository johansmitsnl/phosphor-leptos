/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn DropboxLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M188,120,128,80l55.56-37a8,8,0,0,1,8.88,0L238,73.34a8,8,0,0,1,0,13.32ZM72.44,43a8,8,0,0,0-8.88,0L18,73.34a8,8,0,0,0,0,13.32L68,120l60-40ZM238,153.34,188,120l-60,40,55.56,37a8,8,0,0,0,8.88,0L238,166.66A8,8,0,0,0,238,153.34Zm-220,0a8,8,0,0,0,0,13.32L63.56,197a8,8,0,0,0,8.88,0L128,160,68,120Zm150.61,52.95-38.37-25.58a4,4,0,0,0-4.44,0L87.41,206.29a4,4,0,0,0,0,6.65L123.56,237a8,8,0,0,0,8.88,0l36.15-24.1A4,4,0,0,0,168.59,206.29Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,84,76,120,24,84,76,48Zm104,0L180,48,128,84l52,36ZM24,156l52,36,52-36L76,120Zm104,0,52,36,52-36-52-36Z" opacity="0.2"/><path d="M236.55,149.42,194.05,120l42.5-29.42a8,8,0,0,0,0-13.16l-52-36a8,8,0,0,0-9.1,0L128,74.27,80.55,41.42a8,8,0,0,0-9.1,0l-52,36a8,8,0,0,0,0,13.16L62,120l-42.5,29.42a8,8,0,0,0,0,13.16l52,36a8,8,0,0,0,9.1,0L128,165.73l47.45,32.85a8,8,0,0,0,9.1,0l52-36a8,8,0,0,0,0-13.16ZM180,57.73,218,84,180,110.27,142.05,84ZM38.05,84,76,57.73,114,84,76,110.27Zm38,98.27L38.05,156l38-26.27L114,156Zm14-62.27,38-26.27L166,120,128,146.27Zm90,62.27L142.05,156,180,129.73,218,156Zm-21.53,24.64a8,8,0,0,1-2,11.13l-23.89,16.54a8,8,0,0,1-9.1,0L99.56,218a8,8,0,0,1,9.1-13.16L128,218.27l19.34-13.39A8,8,0,0,1,158.47,206.91Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M234.28,152.71,187,120l47.25-32.71a4,4,0,0,0,0-6.58l-52-36a4,4,0,0,0-4.56,0L128,79.14,78.28,44.71a4,4,0,0,0-4.56,0l-52,36a4,4,0,0,0,0,6.58L69,120,21.72,152.71a4,4,0,0,0,0,6.58l52,36a4,4,0,0,0,4.56,0L128,160.86l49.72,34.43a4,4,0,0,0,4.56,0l52-36a4,4,0,0,0,0-6.58ZM128,151.14,83,120l45-31.14L173,120Zm52-98.27L225,84l-45,31.13L135,84Zm-104,0L121,84,76,115.13,31,84Zm0,134.26L31,156l45-31.13L121,156Zm104,0L135,156l45-31.13L225,156Zm-24.82,22.05a4,4,0,0,1-1,5.57l-23.89,16.54a4,4,0,0,1-4.56,0l-23.89-16.54a4,4,0,0,1,4.56-6.58l21.61,15,21.61-15A4,4,0,0,1,155.18,209.18Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M238.83,146.13,201.08,120l37.75-26.13a12,12,0,0,0,0-19.74l-52-36a12,12,0,0,0-13.66,0L128,69.41,82.83,38.13a12,12,0,0,0-13.66,0l-52,36a12,12,0,0,0,0,19.74L54.92,120,17.17,146.13a12,12,0,0,0,0,19.74l52,36a12,12,0,0,0,13.66,0L128,170.59l45.17,31.28a12,12,0,0,0,13.66,0l52-36a12,12,0,0,0,0-19.74ZM128,141.41,97.08,120,128,98.59,158.92,120ZM180,62.6,210.92,84,180,105.4,149.08,84Zm-104,0L106.92,84,76,105.4,45.08,84Zm0,114.8L45.08,156,76,134.6,106.92,156Zm104,0L149.08,156,180,134.6,210.92,156Zm-25.27,32.09a12,12,0,0,1-3,16.7l-16.86,11.68a12,12,0,0,1-13.66,0l-16.86-11.68A12,12,0,1,1,118,206.46l10,6.94,10-6.94A12,12,0,0,1,154.73,209.49Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M235.42,151.07,190.54,120l44.88-31.07a6,6,0,0,0,0-9.86l-52-36a6,6,0,0,0-6.84,0L128,76.7,79.42,43.07a6,6,0,0,0-6.84,0l-52,36a6,6,0,0,0,0,9.86L65.46,120,20.58,151.07a6,6,0,0,0,0,9.86l52,36a6,6,0,0,0,6.84,0L128,163.3l48.58,33.63a6,6,0,0,0,6.84,0l52-36a6,6,0,0,0,0-9.86ZM128,148.7,86.54,120,128,91.3,169.46,120Zm52-93.4L221.46,84,180,112.7,138.54,84Zm-104,0L117.46,84,76,112.7,34.54,84Zm0,129.4L34.54,156,76,127.3,117.46,156Zm104,0L138.54,156,180,127.3,221.46,156ZM156.82,208a6,6,0,0,1-1.51,8.35l-23.89,16.54a6,6,0,0,1-6.84,0l-23.89-16.54a6,6,0,0,1,6.83-9.86L128,220.7l20.48-14.17A6,6,0,0,1,156.82,208Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M236.55,149.42,194.05,120l42.5-29.42a8,8,0,0,0,0-13.16l-52-36a8,8,0,0,0-9.1,0L128,74.27,80.55,41.42a8,8,0,0,0-9.1,0l-52,36a8,8,0,0,0,0,13.16L62,120l-42.5,29.42a8,8,0,0,0,0,13.16l52,36a8,8,0,0,0,9.1,0L128,165.73l47.45,32.85a8,8,0,0,0,9.1,0l52-36a8,8,0,0,0,0-13.16ZM128,146.27,90.05,120l38-26.27L166,120Zm52-88.54L218,84,180,110.27,142.05,84Zm-104,0L114,84,76,110.27,38.05,84Zm0,124.54L38.05,156l38-26.27L114,156Zm104,0L142.05,156,180,129.73,218,156Zm-21.53,24.64a8,8,0,0,1-2,11.13l-23.89,16.54a8,8,0,0,1-9.1,0L99.56,218a8,8,0,0,1,9.1-13.16L128,218.27l19.34-13.39A8,8,0,0,1,158.47,206.91Z"/> }.into_view()
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