use int_enum::IntEnum;



#[derive(Debug)]
pub enum Command {
    Init(),
    Status,

}

#[derive(Debug)]
pub enum Response {
    Init(),
    Status(Status, Option<String>),
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, IntEnum)]
pub enum Status {
    Ok,
    Uninit,
    Error
}

#[derive(Debug)]
pub enum ResponseError {
    Uninit
}