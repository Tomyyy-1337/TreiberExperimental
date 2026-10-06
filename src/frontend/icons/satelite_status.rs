use topcoat::{Result, context::Cx, runtime::connected, view::{View, component, emit, live, view}};

use crate::{GPS_STATE};

#[component]
pub async fn satelite_status(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        (live! {
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(3));

            loop {
                timer.tick().await;

                let satelice_count = GPS_STATE.satellite_count;
                let color = match satelice_count {
                    0..4 => "text-red-500",
                    4..=7 => "text-yellow-500",
                    _ => "text-green-500",
                };

                let token = emit! {
                    <div class=(format!("flex items-center gap-1 font-bold leading-none {}", color)) aria-label=(format!("Satelliten: {}", satelice_count))>
                        <div class="relative h-[1.1rem] w-[1.1rem] shrink-0" aria-hidden="true">
                            <div class="absolute left-1/2 top-1/2 h-[0.38rem] w-[0.38rem] -translate-x-1/2 -translate-y-1/2 rotate-45 rounded-[0.1rem] bg-current"></div>
                            <div class="absolute left-[0.04rem] top-1/2 h-[0.76rem] w-[0.28rem] -translate-y-1/2 rounded-[0.08rem] bg-current opacity-90"></div>
                            <div class="absolute right-[0.04rem] top-1/2 h-[0.76rem] w-[0.28rem] -translate-y-1/2 rounded-[0.08rem] bg-current opacity-90"></div>
                        </div>
                        <span class="inline-flex min-w-[2ch] items-center justify-center text-center text-[0.85rem]">{(satelice_count)}</span>
                    </div>
                }?;

                if !connected(cx) {
                    break Ok(token);
                }
            }
        })
    })
}