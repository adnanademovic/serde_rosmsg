error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        UnsupportedCharType {
            description("Chars are not supported in ROSMSG")
                display("Chars are not supported in ROSMSG")
        }
        UnsupportedEnumType {
            description("Enumerations are not supported in ROSMSG")
                display("Enumerations are not supported in ROSMSG")
        }
        UnsupportedMapType {
            description("Maps are not supported in ROSMSG")
                display("Maps are not supported in ROSMSG")
        }
        VariableArraySizeAnnotation {
            description("Size annotation in variable size array is missing")
                display("Size annotation in variable size array is missing")
        }
    }
}
