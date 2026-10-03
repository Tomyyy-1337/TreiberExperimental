use topcoat::{Result, context::Cx, runtime::connected, view::{View, attributes, component, emit, live, view}};

use crate::{SHARED_STATE};

#[component]
pub async fn battery_status(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        (live! { 
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                let attributes = attributes! {
                    style = (match SHARED_STATE.battery_percentage {
                        0..=14 => "color: red;",
                        15..=30 => "color: orange;",
                        31..=50 => "color: yellow;",
                        _ => "color: green;",
                    })
                };

                let token = emit! {
                    <div (attributes) class="battery_status-container">
                        <svg viewBox="0 0 24 24" aria-hidden="true">
                            <rect x="3" y="7" width="16" height="10" rx="2" />
                            <rect x="19" y="10" width="2" height="4" rx="1" />
                            let width = u8::max(0, u8::min(12, SHARED_STATE.battery_percentage / 8));
                            <rect x="5" y="9" width=(width) height="6" rx="1" fill="currentColor" />
                        </svg>
                        <span class="battery_status-span">{(SHARED_STATE.battery_percentage)}"%"</span>
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