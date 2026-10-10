use tokio::sync::watch::Receiver;
use topcoat::{Result, context::{Cx, app_context}, runtime::connected, view::{EmitToken, View, class, component, emit, live}};

use crate::{GpsSatelites};

#[component]
pub async fn satelite_status(cx: &Cx) -> Result<impl View> {
    let mut gps_receiver = app_context::<Receiver<GpsSatelites>>(cx).clone();
    gps_receiver.mark_changed();

    let mut timer = tokio::time::interval(std::time::Duration::from_secs(5));
    timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    Ok(live! {
        while let Ok(()) = gps_receiver.changed().await{
            timer.tick().await;
            let satelice_count = gps_receiver.borrow().count;

            let color = match satelice_count {
                0..4 => class!("text-red-500"),
                4..=7 => class!("text-yellow-500"),
                _ => class!("text-green-500"),
            };

            let token = emit! {
                <div class=(class!("flex items-center gap-1 font-bold leading-none", color)) aria-label=(format!("Satelliten: {}", satelice_count))>
                    <div class="relative h-[1.1rem] w-[1.1rem] shrink-0" aria-hidden="true">
                        <div class="absolute left-1/2 top-1/2 h-[0.38rem] w-[0.38rem] -translate-x-1/2 -translate-y-1/2 rotate-45 rounded-[0.1rem] bg-current"></div>
                        <div class="absolute left-[0.04rem] top-1/2 h-[0.76rem] w-[0.28rem] -translate-y-1/2 rounded-[0.08rem] bg-current opacity-90"></div>
                        <div class="absolute right-[0.04rem] top-1/2 h-[0.76rem] w-[0.28rem] -translate-y-1/2 rounded-[0.08rem] bg-current opacity-90"></div>
                    </div>
                    <span class="inline-flex min-w-[2ch] items-center justify-center text-center text-[0.85rem]">{(satelice_count)}</span>
                </div>
            }?;

            if !connected(cx) {
                return Ok(token);
            }
        }
        Ok(EmitToken)
    })
}