use gstd::{debug, msg, prelude::*};

static mut MESSAGE_LOG: Vec<String> = vec![];

#[no_mangle]
extern "C" fn handle() {
    let new_msg: String = msg::load().expect("Unable to create string");

    if new_msg == "PING" {
        msg::reply("PONG", 0).expect("Unable to reply");
    }

    unsafe {
        MESSAGE_LOG.push(new_msg);

        debug!("{:?} total message(s) stored: ", MESSAGE_LOG.len());

        for log in &MESSAGE_LOG {
            debug!(log);
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(unsafe { MESSAGE_LOG.clone() }, 0)
        .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
}

#[no_mangle]
extern "C" fn metahash() {
    msg::reply::<[u8; 32]>(include!("../.metahash"), 0)
        .expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gstd::{Encode, String};
    use gtest::{Log, Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "INIT");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, String::from("PING").encode());
        let log = Log::builder().source(1).dest(42).payload("PONG");
        assert!(res.contains(&log));
    }
}