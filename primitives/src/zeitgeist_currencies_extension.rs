use crate::ZeitgeistMultiReservableCurrency;
use frame_support::dispatch::DispatchResult;
use orml_traits::MultiCurrency;

type CurrencyIdOf<T> = <<T as orml_currencies::Trait>::MultiCurrency as MultiCurrency<
    <T as frame_system::Trait>::AccountId,
>>::CurrencyId;

pub trait ZeitgeistCurrenciesExtension: orml_currencies::Trait
where
    Self::MultiCurrency: ZeitgeistMultiReservableCurrency<Self::AccountId>,
{
    /// Destroy all storage items for a `currency_id`.
    fn destroy_all(currency_id: CurrencyIdOf<Self>) -> DispatchResult {
        let accounts = Self::MultiCurrency::accounts_by_currency_id(currency_id);
        Self::MultiCurrency::destroy_all(currency_id, accounts.iter().cloned());
        Ok(())
    }
}