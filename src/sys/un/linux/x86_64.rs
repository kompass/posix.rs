#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_un {
    pub sun_family: ::sys::socket::sa_family_t,
    pub sun_path: [::char_t; 108],
}

impl ::AsSlice for sockaddr_un { }
impl ::AsMutSlice for sockaddr_un { }

new!(sockaddr_un);
