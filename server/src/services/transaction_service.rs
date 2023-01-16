use crate::alchemy_api::*;

pub type UserTransaction = TransactionApiObj;

pub async fn get_transactions_for_address(wallet_address: String) -> Vec<UserTransaction> {
    let transactions = get_wallet_transactions(Network::ETH, wallet_address).await;
    let mut user_transactions: Vec<UserTransaction> = vec![];

    for tx in transactions.transfers {
        user_transactions.push(tx);
    }

    user_transactions
}