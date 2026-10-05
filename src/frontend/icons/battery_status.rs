use topcoat::{Result, context::Cx, runtime::connected, view::{View, component, emit, live, view}};

use crate::{SHARED_STATE};

#[component]
pub async fn battery_status(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        (live! { 
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                    let battery_color = match SHARED_STATE.battery_percentage {
                        0..=14 => "text-red-500",
                        15..=30 => "text-orange-500",
                        31..=50 => "text-yellow-500",
                        _ => "text-green-500",
                    };

                let token = emit! {
                    <div class=(format!("flex items-center gap-1 {}", battery_color))>
                        <svg class="block h-[1.1rem] w-[1.1rem] shrink-0 fill-none stroke-current" stroke-width="1.8" viewBox="0 0 24 24" aria-hidden="true">
                            <rect x="3" y="7" width="16" height="10" rx="2" />
                            <rect x="19" y="10" width="2" height="4" rx="1" />
                            let width = u8::max(0, u8::min(12, SHARED_STATE.battery_percentage / 8));
                            <rect x="5" y="9" width=(width) height="6" rx="1" fill="currentColor" />
                        </svg>
                        <span class="inline-flex min-w-[2ch] items-center justify-center text-center text-sm leading-none">{(SHARED_STATE.battery_percentage)}"%"</span>
                    </div>
                }?; 
                
                if !connected(cx) {
                    break Ok(token);
                }
                
                timer.tick().await;
            }
        })
    })
}