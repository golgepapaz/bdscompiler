
enum Endianness {

}

pub struct ClassSpec {
    meta : MetaSpec,
    seq: Vec<AttrSpec>
}

pub struct  MetaSpec {
    id : str,
    endian :Option<Endiannes>

}
impl  MetaSpec {
    fn from_yaml() -> MetaSpec {

    }
}
pub struct AttrSpec {

}