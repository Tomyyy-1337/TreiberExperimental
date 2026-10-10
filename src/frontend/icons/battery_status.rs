use tokio::sync::watch::Receiver;
use topcoat::{Result, context::{Cx, app_context}, runtime::connected, view::{EmitToken, View, class, component, emit, live, view}};

use crate::BatteryState;

#[component]
pub async fn battery_status(cx: &Cx) -> Result<impl View> {
    let mut battery_percentage_watch = app_context::<Receiver<BatteryState>>(cx).clone();
    battery_percentage_watch.mark_changed();

    let mut timer = tokio::time::interval(std::time::Duration::from_secs(15));
    timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    Ok(live! { 
        while let Ok(()) = battery_percentage_watch.changed().await {
            timer.tick().await;
            let battery_percentage = battery_percentage_watch.borrow().battery_percentage;

            let color = match battery_percentage {
                0..=14 => class!("text-red-500"),
                15..=30 => class!("text-orange-500"),
                31..=50 => class!("text-yellow-500"),
                _ => class!("text-green-500"),
            };

            let token = emit! {
                <div class=(class!("flex items-center gap-1 font-bold leading-none", color))>
                    <svg class="block h-[1.1rem] w-[1.1rem] shrink-0 fill-none stroke-current" stroke-width="1.8" viewBox="0 0 24 24" aria-hidden="true">
                        <rect x="3" y="7" width="16" height="10" rx="2" />
                        <rect x="19" y="10" width="2" height="4" rx="1" />
                        let width = u8::max(0, u8::min(12, battery_percentage / 8));
                        <rect x="5" y="9" width=(width) height="6" rx="1" fill="currentColor" />
                    </svg>
                    <span class="inline-flex min-w-[2ch] items-center justify-center text-center text-[0.85rem]">{(battery_percentage)}"%"</span>
                </div>
            }?; 
            
            if !connected(cx) {
                return Ok(token);
            }
        }
        Ok(EmitToken)
    })
}