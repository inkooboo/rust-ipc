pub enum CreateMode {
    CreateOnly,
    OpenOrCreate,
    OpenOnly,
}

pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

pub enum Permissions {
    User,
    Group,
    Everybody
}
