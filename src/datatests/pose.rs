//! Perform tests with geometry_msgs/Pose

#[cfg(test)]
mod tests {
    use from_slice;
    use std::collections::HashMap;

    #[derive(Debug,Deserialize,PartialEq)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Debug,Deserialize,PartialEq)]
    struct Orientation {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[derive(Debug,Deserialize,PartialEq)]
    struct Structure {
        position: Position,
        orientation: Orientation,
    }

    #[test]
    fn reads_message() {
        assert_eq!(Structure {
                       position: Position {
                           x: 1.0,
                           y: 2.0,
                           z: 3.0,
                       },
                       orientation: Orientation {
                           x: 4.0,
                           y: 5.0,
                           z: 6.0,
                           w: 7.0,
                       },
                   },
                   from_slice(include_bytes!("pose_msg.bin")).unwrap());
    }

    #[test]
    fn reads_request_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("pose_req.bin")).unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/meow", header.get("topic").unwrap());
        assert_eq!("0", header.get("tcp_nodelay").unwrap());
        assert_eq!(include_str!("pose_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/Pose", header.get("type").unwrap());
        assert_eq!("e45d45a5a1ce597b249e23fb30fc871f",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_8282_1487460134332",
                   header.get("callerid").unwrap());
    }

    #[test]
    fn reads_response_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("pose_res.bin")).unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/meow", header.get("topic").unwrap());
        assert_eq!("1", header.get("latching").unwrap());
        assert_eq!(include_str!("pose_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/Pose", header.get("type").unwrap());
        assert_eq!("e45d45a5a1ce597b249e23fb30fc871f",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_8315_1487460159629",
                   header.get("callerid").unwrap());
    }
}
