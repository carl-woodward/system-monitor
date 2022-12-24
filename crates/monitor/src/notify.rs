use com::class;
use com::sys::{HRESULT, NOERROR};

use crate::sdk::INotify;

class! {
    #[derive(Debug)]
    pub class NotifyImpl: INotify {
    }

    impl INotify for NotifyImpl {
        fn WatchFiles(&self) -> HRESULT {
            println!("Watching files...");
            NOERROR
        }
    }

}
