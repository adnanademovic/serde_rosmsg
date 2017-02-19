//! Perform tests with geometry_msgs/PoseWithCovariance

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
        covariance: [[f64; 6]; 6],
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
                       covariance: [[8.0, 9.0, 10.0, 11.0, 12.0, 13.0],
                                    [14.0, 15.0, 16.0, 17.0, 18.0, 19.0],
                                    [20.0, 21.0, 22.0, 23.0, 24.0, 25.0],
                                    [26.0, 27.0, 28.0, 29.0, 30.0, 31.0],
                                    [32.0, 33.0, 34.0, 35.0, 36.0, 37.0],
                                    [38.0, 39.0, 40.0, 41.0, 42.0, 43.0]],
                   },
                   from_slice(include_bytes!("pose_with_covariance_msg.bin")).unwrap());
    }

    #[test]
    fn reads_request_header() {
        let header =
            from_slice::<HashMap<String, String>>(include_bytes!("pose_with_covariance_req.bin"))
                .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/woah", header.get("topic").unwrap());
        assert_eq!("0", header.get("tcp_nodelay").unwrap());
        assert_eq!(include_str!("pose_with_covariance_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/PoseWithCovariance",
                   header.get("type").unwrap());
        assert_eq!("c23e848cf1b7533a8d7c259073a97e6f",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_8651_1487460349584",
                   header.get("callerid").unwrap());
    }

    #[test]
    fn reads_response_header() {
        let header =
            from_slice::<HashMap<String, String>>(include_bytes!("pose_with_covariance_res.bin"))
                .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/woah", header.get("topic").unwrap());
        assert_eq!("1", header.get("latching").unwrap());
        assert_eq!(include_str!("pose_with_covariance_message_definition.txt"),
                   header.get("message_definition").unwrap());
        assert_eq!("geometry_msgs/PoseWithCovariance",
                   header.get("type").unwrap());
        assert_eq!("c23e848cf1b7533a8d7c259073a97e6f",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_8959_1487460376004",
                   header.get("callerid").unwrap());
    }
}
