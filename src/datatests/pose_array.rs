//! Perform tests with geometry_msgs/PoseArray

#[cfg(test)]
mod tests {
    use from_slice;
    use std::collections::HashMap;

    #[derive(Clone,Debug,Deserialize,PartialEq)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Clone,Debug,Deserialize,PartialEq)]
    struct Orientation {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[derive(Clone,Debug,Deserialize,PartialEq)]
    struct Pose {
        position: Position,
        orientation: Orientation,
    }

    #[derive(Debug,Deserialize,PartialEq)]
    struct Time {
        secs: u32,
        nsecs: u32,
    }

    #[derive(Debug,Deserialize,PartialEq)]
    struct Header {
        id: u32,
        time: Time,
        frame_id: String,
    }

    #[derive(Debug,Deserialize,PartialEq)]
    struct Structure {
        header: Header,
        poses: Vec<Pose>,
    }

    #[test]
    fn reads_message() {
        let pose = Pose {
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
        };
        assert_eq!(Structure {
                       header: Header {
                           id: 1,
                           time: Time {
                               secs: 0,
                               nsecs: 0,
                           },
                           frame_id: "ABC".into(),
                       },
                       poses: vec![pose.clone(),
                                   pose.clone(),
                                   pose.clone(),
                                   pose.clone(),
                                   pose.clone()],
                   },
                   from_slice(include_bytes!("pose_array_msg.bin")).unwrap());
    }

    #[test]
    fn reads_request_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("pose_array_req.bin"))
            .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/xax", header.get("topic").unwrap());
        assert_eq!("0", header.get("tcp_nodelay").unwrap());
        assert_eq!(include_str!("pose_array_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/PoseArray", header.get("type").unwrap());
        assert_eq!("916c28c5764443f268b296bb671b9d97",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_3051_1487462805714",
                   header.get("callerid").unwrap());
    }

    #[test]
    fn reads_response_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("pose_array_res.bin"))
            .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/xax", header.get("topic").unwrap());
        assert_eq!("1", header.get("latching").unwrap());
        assert_eq!(include_str!("pose_array_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/PoseArray", header.get("type").unwrap());
        assert_eq!("916c28c5764443f268b296bb671b9d97",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_3078_1487462814794",
                   header.get("callerid").unwrap());
    }
}
