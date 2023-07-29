use core::fmt;

pub struct DataStruct<'a> {
    pub extension : String,
    pub single : &'a str,
}

impl<'a> fmt::Debug for DataStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataStruct").field("extension", &self.extension).field("single", &self.single).finish()
    }
}

pub fn get_data() -> Vec<DataStruct<'static>> {
    let data : Vec<DataStruct> = vec![
        DataStruct {
            extension : String::from("py"),
            single : r"#.*$",
        },
        DataStruct{
            extension : String::from("c"),
            single : r"^\/\/.*",
        }
    ];
    
    return data;
}

// fn getRegex(file_name: &String) -> string {
    
// }