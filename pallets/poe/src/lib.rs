#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

///模板引入依赖

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system,ensure_signed};
use sp_std::vec::Vec;
use sp_std::prelude::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
//存储单元
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		//用于存储存证信息
		Proofs get(fn proofs):map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,T::BlockNumber)
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
//事件
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Config>::AccountId {
		///创建存证触发事件
		ClaimCreated(AccountId,Vec<u8>),
		///移除存证事件
		ClaimRevoked(AccountId,Vec<u8>),
		ClaimMoved(AccountId, AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
//异常处理
decl_error! {
	pub enum Error for Module<T: Config> {
		///存证已存在
		ProofAlreadyExist,
		///存证不存在
		ClaimNotExist,
		///不是存证本人
		NotClaimOwner,
		DestinationIsClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.


decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		fn deposit_event() = default;

		//创建存证
		#[weight = 0]
		pub fn create_claim(origin,claim:Vec<u8>)->dispatch::DispatchResult{

			//调用方法人是否是存证人
			let sender = ensure_signed(origin)?;
  			//判断是否存证过，存证过throw error
			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			//进行存证插入数据
			Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));
			// 触发事件
			Self::deposit_event(RawEvent::ClaimCreated(sender,claim));   // ClaimCreated事件，需要decl_event处理
			// 返回ok
			Ok(())

		}

		//撤销存证
		#[weight = 0]
		pub fn revoke_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

  			// 判断存证是否存在
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			//获取存证
			let (owner,_block_number) = Proofs::<T>::get(&claim);

			//校验存证人
			ensure!(owner == sender,Error::<T>::NotClaimOwner);

			//删除存证
		    Proofs::<T>::remove(&claim);

			// 触发事件
		    Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));

			// 返回
			Ok(())
		}
		//转移存证
		#[weight = 0]
		pub fn move_claim(origin,_destination: T::AccountId,claim: Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			let (owner,_block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            ensure!(owner != _destination, Error::<T>::DestinationIsClaimOwner);

            Proofs::<T>::remove(&claim);

            Proofs::<T>::insert(
                &claim,
                (_destination.clone(), <frame_system::Pallet::<T>>::block_number()),
            );

            Self::deposit_event(RawEvent::ClaimMoved(sender, _destination, claim));

			// 返回
			Ok(())
		}


	}
}
