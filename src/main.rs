use caps2esc_hjkl_arrow::*;
use interception_rs::event::EventManager;
use interception_rs::prelude::*;

fn main() {
    let mut evm = EventManager::default();
    loop {
        for ev in evm.fetch_events_batch() {
            if !ev.is_key() {
                evm.write_event(ev);
                continue;
            }
            if ev.is_caps() && (ev.is_down() || ev.is_repeat()) {
                if evm.get_last_event().is_repeat() {
                    evm.write_sync_event(20_000);
                    evm.write_event(evm.get_last_event().get_up_ev());
                }
                'caps_down: loop {
                    for cde in evm.fetch_events_batch() {
                        if !cde.is_key() {
                            evm.write_event(cde);
                            continue;
                        };
                        if cde.is_caps() && cde.is_up() {
                            evm.write_event(KeyCode::KEY_ESC.down());
                            evm.write_sync_event(20_000);
                            evm.write_event(KeyCode::KEY_ESC.up());
                            break 'caps_down;
                        } else {
                            evm.write_event(cde.get_hjkl_equivalent());
                            evm.write_sync_event(20_000);
                            let mut last_event = cde.get_hjkl_equivalent();
                            loop {
                                for chjkl in evm.fetch_events_batch() {
                                    if !chjkl.is_key() {
                                        evm.write_event(chjkl);
                                        continue;
                                    };
                                    if chjkl.is_caps() && chjkl.is_up() {
                                        if last_event.is_repeat() {
                                            evm.write_sync_event(20_000);
                                            evm.write_event(last_event.get_up_ev())
                                        }
                                        break 'caps_down;
                                    }
                                    let replace_event = chjkl.get_hjkl_equivalent();
                                    evm.write_event(replace_event);
                                    last_event = replace_event;
                                }
                            }
                        }
                    }
                }
            } else {
                evm.write_event(ev);
            }
        }
    }
}
