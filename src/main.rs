use std::io::BufWriter;

use protobuf::{parse_from_bytes, CodedInputStream, CodedOutputStream, Message};

use protobuf_exploration::message::gen::core::*;
use protobuf_exploration::message::gen::request::*;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let limit = 1_000_000;

    for _ in 0..limit {
        let mut uuid = UuidMessage::new();
        uuid.set_v1(4567894567890);
        uuid.set_v2(9876549876543);

        let mut north = Request::new();
        north.set_Id(123123);
        north.set_Session(uuid.clone());
        north.set_Character(uuid.clone());
        north.set_Movement(Direction::North);

        let mut south = north.clone();
        south.set_Facing(Direction::South);

        let mut buffer1 = vec![0u8; 0];
        let mut buffer2 = vec![0u8; 0];

        let mut os1 = CodedOutputStream::new(&mut buffer1);
        let mut os2 = CodedOutputStream::new(&mut buffer2);

        let _ = north.write_to(&mut os1);
        let _ = south.write_to(&mut os2);

        let _ = os1.flush();
        let _ = os2.flush();

        let mut request1: Request = parse_from_bytes(&buffer1).unwrap();
        let mut request2: Request = parse_from_bytes(&buffer2).unwrap();

        assert_eq!(request1.Id, request2.Id);
    }

    let microseconds = SystemTime::now().duration_since(start).unwrap().as_micros();

    println!(
        "Took {}µs to iterate {} times ({} iter/µs, 1 iter every {}µs).",
        microseconds,
        limit,
        (limit as f32 / microseconds as f32),
        (microseconds as f32 / limit as f32)
    );
}
