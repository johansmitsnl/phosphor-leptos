/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ShieldSlash(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M213.92,210.61a8,8,0,1,1-11.84,10.77l-14.51-16A147.19,147.19,0,0,1,133,239.18a15.44,15.44,0,0,1-10,0c-15.2-5-91-34.76-91-124.38V56a16,16,0,0,1,8.26-14,8,8,0,0,1,13.66-7.39ZM208,40H98.52A8,8,0,0,0,92.6,53.38L199.69,171.17a7.94,7.94,0,0,0,5.91,2.62,7.64,7.64,0,0,0,1.26-.1,8,8,0,0,0,6-4.61c7.37-16.36,11.1-34.62,11.1-54.29V56A16,16,0,0,0,208,40Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,56v58.77c0,84.18-71.31,112.07-85.54,116.8a7.54,7.54,0,0,1-4.92,0C111.31,226.86,40,199,40,114.79V56a8,8,0,0,1,8-8H208A8,8,0,0,1,216,56Z" opacity="0.2"/><path d="M53.92,34.62A8,8,0,0,0,40.26,42,16,16,0,0,0,32,56v58.77c0,89.62,75.82,119.34,91,124.38a15.44,15.44,0,0,0,10,0,147.19,147.19,0,0,0,54.59-33.76l14.51,16a8,8,0,1,0,11.84-10.77ZM128,224c-13.53-4.5-80-30.68-80-109.18V56h3.73L176.8,193.57A130.13,130.13,0,0,1,128,224ZM224,56v58.77c0,19.67-3.73,37.93-11.1,54.29a8,8,0,1,1-14.59-6.57c6.43-14.28,9.69-30.33,9.69-47.72V56L98.52,56a8,8,0,1,1,0-16H208A16,16,0,0,1,224,56Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M51,37.31A4,4,0,0,0,45,42.69l1.31,1.44A12,12,0,0,0,36,56v58.77c0,86.88,73.54,115.7,88.28,120.59a11.47,11.47,0,0,0,7.44,0c7.82-2.59,34.16-12.64,55.95-35.8L205,218.69a4,4,0,1,0,5.92-5.38Zm78.23,190.48a3.51,3.51,0,0,1-2.39,0C113,223.2,44,196.17,44,114.79V56a4,4,0,0,1,4-4h5.5L182.27,193.65C161.69,215.77,136.61,225.33,129.19,227.79ZM220,56v58.77c0,19.1-3.62,36.81-10.75,52.65a4,4,0,0,1-7.29-3.29c6.66-14.79,10-31.4,10-49.36V56a4,4,0,0,0-4-4H98.52a4,4,0,1,1,0-8H208A12,12,0,0,1,220,56Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M228,56V114.8a138.72,138.72,0,0,1-7.77,46.8,12,12,0,1,1-22.6-8.08A114.62,114.62,0,0,0,204,114.8V60H109.33a12,12,0,1,1,0-24H208A20,20,0,0,1,228,56ZM216.88,207.93a12,12,0,1,1-17.76,16.15l-11.74-12.92A151.78,151.78,0,0,1,134.24,243a19.63,19.63,0,0,1-12.49,0C106.1,237.78,28,207.16,28,114.8V56a20,20,0,0,1,8-16,12,12,0,0,1,20.87-8.1Zm-45.64-14.52L52,62.24V114.8c0,73.56,60.53,99.53,76,105A126.88,126.88,0,0,0,171.24,193.41Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M52.44,36a6,6,0,0,0-9.63,7A14,14,0,0,0,34,56v58.77c0,88.25,74.68,117.52,89.65,122.49a13.5,13.5,0,0,0,8.7,0c7.76-2.58,33.48-12.37,55.29-34.76L203.56,220a6,6,0,0,0,4.44,2,6,6,0,0,0,4.44-10Zm76.12,189.93a1.57,1.57,0,0,1-1.13,0C113.84,221.38,46,194.8,46,114.79V56a2,2,0,0,1,2-2h4.62L179.55,193.62C159.54,214.44,135.72,223.52,128.56,225.89ZM222,56v58.77c0,19.38-3.67,37.37-10.92,53.47a6,6,0,0,1-11-4.93c6.55-14.54,9.87-30.87,9.87-48.54V56a2,2,0,0,0-2-2H98.52a6,6,0,1,1,0-12H208A14,14,0,0,1,222,56Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M53.92,34.62A8,8,0,0,0,40.26,42,16,16,0,0,0,32,56v58.77c0,89.62,75.82,119.34,91,124.38a15.44,15.44,0,0,0,10,0,147.19,147.19,0,0,0,54.59-33.76l14.51,16a8,8,0,1,0,11.84-10.77ZM128,224c-13.53-4.5-80-30.68-80-109.18V56h3.73L176.8,193.57A130.13,130.13,0,0,1,128,224ZM224,56v58.77c0,19.67-3.73,37.93-11.1,54.29a8,8,0,1,1-14.59-6.57c6.43-14.28,9.69-30.33,9.69-47.72V56L98.52,56a8,8,0,1,1,0-16H208A16,16,0,0,1,224,56Z"/> }.into_view()
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