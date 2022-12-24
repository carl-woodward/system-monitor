use com::{interfaces, interfaces::iunknown::IUnknown, sys::HRESULT};

use com::sys::IID;

pub const CLSID_SDK_FACTORY: IID = IID {
    data1: 0x64CB45AE,
    data2: 0x917D,
    data3: 0x4BA9,
    data4: [0xBF, 0xFE, 0xa3, 0x3B, 0x9A, 0x9C, 0xB4, 0xFE],
};

interfaces! {
    #[uuid("4E045146-A90E-4EC8-A1F4-EAB3F8AC4DD5")]
    pub unsafe interface IConfig : IUnknown {
        pub fn Set(&self, config: *const Config) -> HRESULT;
        pub fn Get(&self) -> *mut Config;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Config {
    pub magic: u128,
}
