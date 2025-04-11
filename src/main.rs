#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Palet adı (Module)
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{self as system, pallet_prelude::*};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call]
        pub fn create_poll(origin: OriginFor<T>, question: Vec<u8>, options: Vec<Vec<u8>>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Oylama anketi oluşturuluyor
            Ok(())
        }

        #[pallet::call]
        pub fn vote(origin: OriginFor<T>, poll_id: u32, option_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Oy kullanma işlemi
            Ok(())
        }

        #[pallet::call]
        pub fn get_poll_results(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Oylama sonuçlarını almak
            Ok(())
        }
    }
}





fn main() {
    
}
