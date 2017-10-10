use yaml_rust::*;

enum Endianness {

}

pub struct ClassSpec {
    meta : MetaSpec,
    seq: Vec<AttrSpec>
}

pub struct  MetaSpec {
    id : String,
    endian :Option<Endianness>

}
impl  MetaSpec {
    fn from_yaml(t: Vec<Yaml>  ) -> MetaSpec {
        MetaSpec {id:"dfdf".to_owned(),endian:None}
    }
}
pub struct AttrSpec {

}