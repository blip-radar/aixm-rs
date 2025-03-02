#![allow(clippy::print_stderr, clippy::print_stdout, reason = "example")]

use std::{env::args, fs::File, io::BufReader};

use aixm::MessageAixmBasicMessage;

fn main() {
    let mut aixm_data = vec![];
    for file_path in args().skip(1) {
        let reader = BufReader::new(File::open(&file_path).unwrap());

        // parse XML into generated struct
        if let Ok(aixm) = quick_xml::de::from_reader::<_, MessageAixmBasicMessage>(reader)
            .inspect_err(|e| eprintln!("{file_path}: {e}"))
        {
            aixm_data.extend(aixm.message_has_member.into_iter().map(|m| m.member));
        }
    }

    println!("{aixm_data:#?}");
}
