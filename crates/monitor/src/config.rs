use com::class;
use com::sys::{HRESULT, NOERROR};

use crate::sdk::{Config, IConfig};

class! {
    #[derive(Debug)]
    pub class ConfigImpl: IConfig {
    }

    impl IConfig for ConfigImpl {
        fn Set(&self, config: *const Config) -> HRESULT {
            assert!(!config.is_null());
            let config = unsafe { *config };
            println!("Setting config with magic number {}...", config.magic);
            NOERROR
        }

        fn Get(&self) -> *mut Config {
            let config = Config { magic: 0xdeadbeef };
            let config = Box::into_raw(Box::new(config));
            config
        }
    }
}
