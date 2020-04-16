// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use x11rb::connection::Connection;
use x11rb::generated::xproto::*;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::x11_utils::Event;

// Lifted from XCB
#[allow(dead_code)]
enum IcccmWmState {
    Withdrawn = 0,
    Normal = 1,
    Iconic = 3,
}

x11rb::atom_manager! {
    AtomCollection: AtomCollectionCookie {
        WM_STATE,
        WM_CHANGE_STATE,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let atoms = AtomCollection::new(&conn)?.reply()?;

    let attrs = ChangeWindowAttributesAux::new().event_mask(EventMask::SubstructureNotify);
    conn.change_window_attributes(screen.root, &attrs)?;
    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;
        match event.response_type() {
            CLIENT_MESSAGE_EVENT => {
                let event: ClientMessageEvent = event.into();
                if event.type_ == atoms.WM_CHANGE_STATE
                    && event.data.as_data32()[0] == IcccmWmState::Iconic as u32
                {
                    conn.change_property32(
                        PropMode::Replace,
                        event.window,
                        atoms.WM_STATE,
                        atoms.WM_STATE,
                        &[IcccmWmState::Normal as u32, 0],
                    )?;
                    conn.flush()?;
                }
            }
            _ => {}
        }
    }
}
