//! Perform tests with std_msgs/String

#[cfg(test)]
mod tests {
    use from_slice;
    use std::collections::HashMap;

    #[derive(Debug,Deserialize,PartialEq)]
    struct Structure {
        data: String,
    }

    #[test]
    fn reads_message() {
        assert_eq!(Structure { data: "Hello, World!".into() },
                   from_slice(include_bytes!("string_msg.bin")).unwrap());
    }

    #[test]
    fn reads_request_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("string_req.bin"))
            .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/moo", header.get("topic").unwrap());
        assert_eq!("0", header.get("tcp_nodelay").unwrap());
        assert_eq!("string data\n", header.get("message_definition").unwrap());
        assert_eq!("std_msgs/String", header.get("type").unwrap());
        assert_eq!("992ce8a1687cec8c8bd883ec73ca41d1",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_4349_1487459109528",
                   header.get("callerid").unwrap());
    }

    #[test]
    fn reads_response_header() {
        let header = from_slice::<HashMap<String, String>>(include_bytes!("string_res.bin"))
            .unwrap();
        assert_eq!(6, header.len());
        assert_eq!("/moo", header.get("topic").unwrap());
        assert_eq!("1", header.get("latching").unwrap());
        assert_eq!("string data\n", header.get("message_definition").unwrap());
        assert_eq!("std_msgs/String", header.get("type").unwrap());
        assert_eq!("992ce8a1687cec8c8bd883ec73ca41d1",
                   header.get("md5sum").unwrap());
        assert_eq!("/rostopic_5125_1487459213058",
                   header.get("callerid").unwrap());
    }
}
