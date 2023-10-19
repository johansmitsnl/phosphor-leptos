/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn DribbbleLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M93.27,36.86a4,4,0,0,1,.82-7.19,103.94,103.94,0,0,1,88.66,9.95,4,4,0,0,1,1,5.87,153.32,153.32,0,0,1-41.89,37A169.43,169.43,0,0,0,93.27,36.86ZM127.58,90a153,153,0,0,0-56-46.91,3.94,3.94,0,0,0-4,.33,104.41,104.41,0,0,0-38.34,52,4,4,0,0,0,3,5.16A152.34,152.34,0,0,0,64,104,151,151,0,0,0,127.58,90Zm103.8,26.69A103.81,103.81,0,0,0,202.19,55.2a4,4,0,0,0-6,.34,169.15,169.15,0,0,1-45.69,40.4,167.73,167.73,0,0,1,13.55,29.9A167.64,167.64,0,0,1,208,120,169.35,169.35,0,0,1,227,121.07,4,4,0,0,0,231.38,116.72Zm-62.91,24.5a167.7,167.7,0,0,1,4.45,38.47,168,168,0,0,1-4.11,36.85A4,4,0,0,0,174.5,221a104.25,104.25,0,0,0,56.57-79.25,4,4,0,0,0-3.49-4.49,152.44,152.44,0,0,0-59.11,4Zm-19.64-10.45a151.76,151.76,0,0,0-12.39-27.21A167,167,0,0,1,64,120a168.4,168.4,0,0,1-34.88-3.65,4,4,0,0,0-4.81,3.56q-.31,4-.32,8.09a103.72,103.72,0,0,0,33,75.91,4,4,0,0,0,6.15-.92A169,169,0,0,1,148.83,130.77ZM75.69,213.25a4,4,0,0,0,1.52,5.48,103.88,103.88,0,0,0,68.85,11.69,3.93,3.93,0,0,0,3.06-2.65,152.6,152.6,0,0,0,7.8-48.08,151.3,151.3,0,0,0-3.74-33.46A152.94,152.94,0,0,0,75.69,213.25Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"/><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm87.65,96.18Q211.83,120,208,120a168.58,168.58,0,0,0-43.94,5.84A166.52,166.52,0,0,0,150.61,96a168.32,168.32,0,0,0,38.2-31.55A87.78,87.78,0,0,1,215.65,120.18ZM176.28,54.46A151.75,151.75,0,0,1,142,82.52a169.22,169.22,0,0,0-38.63-39,88,88,0,0,1,73,10.94ZM85.65,50.88a153.13,153.13,0,0,1,42,39.18A151.82,151.82,0,0,1,64,104a154.19,154.19,0,0,1-20.28-1.35A88.39,88.39,0,0,1,85.65,50.88ZM40,128a87.73,87.73,0,0,1,.53-9.64A168.85,168.85,0,0,0,64,120a167.84,167.84,0,0,0,72.52-16.4,150.82,150.82,0,0,1,12.31,27.13,167.11,167.11,0,0,0-24.59,11.6,169.22,169.22,0,0,0-55.07,51.06A87.8,87.8,0,0,1,40,128Zm42,75a152.91,152.91,0,0,1,50.24-46.79,148.81,148.81,0,0,1,20.95-10,152.48,152.48,0,0,1,3.73,33.47,152.93,152.93,0,0,1-3.49,32.56A87.92,87.92,0,0,1,82,203Zm89.06,1.73a170,170,0,0,0,1.86-25,168.69,168.69,0,0,0-4.45-38.47A152.31,152.31,0,0,1,208,136q3.8,0,7.61.19A88.13,88.13,0,0,1,171.06,204.72Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm91.92,96.43C216,124.15,212,124,208,124a164.4,164.4,0,0,0-46.55,6.74A163.48,163.48,0,0,0,146.61,97c-.49-.82-1-1.64-1.5-2.46A164,164,0,0,0,188.5,58.75,91.8,91.8,0,0,1,219.92,124.43ZM182.25,53.75a156.23,156.23,0,0,1-41.46,34.08,165,165,0,0,0-46.3-45.51,91.86,91.86,0,0,1,87.76,11.43ZM85.78,46.28a157,157,0,0,1,47.9,45.34A155.67,155.67,0,0,1,64,108a157.47,157.47,0,0,1-25.33-2A92.35,92.35,0,0,1,85.78,46.28ZM36,128a92.34,92.34,0,0,1,1.1-14.2A165.6,165.6,0,0,0,64,116a163.6,163.6,0,0,0,74.05-17.62c.55.9,1.11,1.79,1.64,2.7a155.15,155.15,0,0,1,14.14,32.11,162.7,162.7,0,0,0-27.59,12.61A164.94,164.94,0,0,0,70,199.36,91.84,91.84,0,0,1,36,128Zm40.44,76.16a157.08,157.08,0,0,1,53.8-51.43A153.87,153.87,0,0,1,156,140.91a156.32,156.32,0,0,1,4.9,38.78,157,157,0,0,1-4.11,35.69,91.94,91.94,0,0,1-80.37-11.22Zm89.3,7.74a165.24,165.24,0,0,0,3.18-32.21,164.26,164.26,0,0,0-5.28-41.26A156.44,156.44,0,0,1,208,132c4,0,7.94.15,11.88.45A92.17,92.17,0,0,1,165.74,211.9Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm83.13,96c-1,0-2.08,0-3.12,0a172.63,172.63,0,0,0-41.39,5.06A171.26,171.26,0,0,0,156,97.39,172.34,172.34,0,0,0,188.9,70.24,83.72,83.72,0,0,1,211.13,116ZM170,55.3a148.53,148.53,0,0,1-27,21.88,173.29,173.29,0,0,0-30.58-31.71A83.52,83.52,0,0,1,170,55.3Zm-84.46.27a149.23,149.23,0,0,1,35.9,32.87A147.73,147.73,0,0,1,64,100c-5,0-10-.26-14.94-.75A84.49,84.49,0,0,1,85.53,55.57ZM44,128c0-1.73.07-3.44.17-5.14A175.15,175.15,0,0,0,64,124a171.8,171.8,0,0,0,70.84-15.22,145.82,145.82,0,0,1,8.92,19.65,170.71,170.71,0,0,0-21.52,10.44,173,173,0,0,0-53.68,48.44A83.77,83.77,0,0,1,44,128Zm43.77,73.72a149,149,0,0,1,46.46-42.06,147.2,147.2,0,0,1,16-7.94,148.52,148.52,0,0,1,2.67,28A148.66,148.66,0,0,1,150,209.06a83.81,83.81,0,0,1-62.22-7.34Zm88.29-4.89c.56-5.68.86-11.4.86-17.14a172.57,172.57,0,0,0-3.72-35.54A148.85,148.85,0,0,1,208,140c1,0,2.07,0,3.11,0A84.07,84.07,0,0,1,176.06,196.83Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm89.81,96.3c-3.26-.19-6.54-.3-9.8-.3a166.44,166.44,0,0,0-45.25,6.29A164.28,164.28,0,0,0,148.33,96c-.14-.25-.3-.49-.44-.74a166.34,166.34,0,0,0,40.79-33.71A89.79,89.79,0,0,1,217.81,122.3ZM179.29,54.09a154.52,154.52,0,0,1-37.9,31.11A167,167,0,0,0,98.88,42.84a89.87,89.87,0,0,1,80.41,11.25ZM85.71,48.58a155,155,0,0,1,45,42.27A153.71,153.71,0,0,1,64,106a156.8,156.8,0,0,1-22.84-1.69A90.37,90.37,0,0,1,85.71,48.58ZM38,128a90.17,90.17,0,0,1,.79-11.92A167.23,167.23,0,0,0,64,118a165.69,165.69,0,0,0,73.29-17c.22.37.46.73.67,1.1A152.2,152.2,0,0,1,151.34,132a164.57,164.57,0,0,0-26.09,12.11A167,167,0,0,0,69.57,196.4,89.84,89.84,0,0,1,38,128Zm41.19,75.58a155.24,155.24,0,0,1,52.05-49.12,152.9,152.9,0,0,1,23.38-10.93,154.31,154.31,0,0,1,4.3,36.16,154.78,154.78,0,0,1-3.81,34.13,89.88,89.88,0,0,1-75.92-10.24Zm89.24,4.81a166.76,166.76,0,0,0,2.49-28.7,166.67,166.67,0,0,0-4.86-39.87A154.6,154.6,0,0,1,208,134c3.25,0,6.52.11,9.77.32A90.16,90.16,0,0,1,168.43,208.39Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm87.65,96.18Q211.83,120,208,120a168.58,168.58,0,0,0-43.94,5.84A166.52,166.52,0,0,0,150.61,96a168.32,168.32,0,0,0,38.2-31.55A87.78,87.78,0,0,1,215.65,120.18ZM176.28,54.46A151.75,151.75,0,0,1,142,82.52a169.22,169.22,0,0,0-38.63-39,88,88,0,0,1,73,10.94ZM85.65,50.88a153.13,153.13,0,0,1,42,39.18A151.82,151.82,0,0,1,64,104a154.19,154.19,0,0,1-20.28-1.35A88.39,88.39,0,0,1,85.65,50.88ZM40,128a87.73,87.73,0,0,1,.53-9.64A168.85,168.85,0,0,0,64,120a167.84,167.84,0,0,0,72.52-16.4,150.82,150.82,0,0,1,12.31,27.13,167.11,167.11,0,0,0-24.59,11.6,169.22,169.22,0,0,0-55.07,51.06A87.8,87.8,0,0,1,40,128Zm42,75a152.91,152.91,0,0,1,50.24-46.79,148.81,148.81,0,0,1,20.95-10,152.48,152.48,0,0,1,3.73,33.47,152.93,152.93,0,0,1-3.49,32.56A87.92,87.92,0,0,1,82,203Zm89.06,1.73a170,170,0,0,0,1.86-25,168.69,168.69,0,0,0-4.45-38.47A152.31,152.31,0,0,1,208,136q3.8,0,7.61.19A88.13,88.13,0,0,1,171.06,204.72Z"/> }.into_view()
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