use crate::apis::firefly::models::{
    Boost, Direction, Request, RequestStatus, Transfer, WalletStateAndHistory,
};
pub fn example_wallet_history() -> WalletStateAndHistory {
    let requests = vec![
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 11 * 60) + 14 * 60) * 1000) as u64,
            amount: 12,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 12 * 60) + 14 * 60) * 1000) as u64,
            amount: 37,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 13 * 60) + 14 * 60) * 1000) as u64,
            amount: 13,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 14 * 60) + 14 * 60) * 1000) as u64,
            amount: 12,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 15 * 60) + 14 * 60) * 1000) as u64,
            amount: 37,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 16 * 60) + 14 * 60) * 1000) as u64,
            amount: 13,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 17 * 60) + 14 * 60) * 1000) as u64,
            amount: 12,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 18 * 60) + 14 * 60) * 1000) as u64,
            amount: 37,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 19 * 60) + 14 * 60) * 1000) as u64,
            amount: 13,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 20 * 60) + 14 * 60) * 1000) as u64,
            amount: 12,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 21 * 60) + 14 * 60) * 1000) as u64,
            amount: 37,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 22 * 60) + 14 * 60) * 1000) as u64,
            amount: 13,
            status: RequestStatus::CANCELLED,
        },
    ];
    let boosts = vec![
        Boost {
            id: "deade3123f".to_string(),
            direction: Direction::OUTGOING,
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((24 + 23 * 60) + 14 * 60) * 1000) as u64,
            amount: 33,
            username: "foo.test".to_string(),
            post: "3lmeun6nxfc27".to_string(),
        },
        Boost {
            id: "darggxc45".to_string(),
            direction: Direction::INCOMING,
            date: chrono::Utc::now().timestamp_millis() as u64
                - (((2 * 24 * 60) + 14 * 60) * 1000) as u64,
            amount: 12,
            username: "SomeUser33".to_string(),
            post: "1234412".to_string(),
        },
    ];
    let transfers = vec![
        Transfer {
            id: "j634gf5".to_string(),
            direction: Direction::INCOMING,
            date: chrono::Utc::now().timestamp_millis() as u64
                - ((((2 * 24) + 1 * 60) + 14 * 60) * 1000) as u64,
            amount: 40,
            to_address: "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA".to_string(),
        },
        Transfer {
            id: "darggxc45".to_string(),
            direction: Direction::OUTGOING,
            date: chrono::Utc::now().timestamp_millis() as u64
                - ((((2 * 24) + 2 * 60) + 14 * 60) * 1000) as u64,
            amount: 7,
            to_address: "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA".to_string(),
        },
    ];

    WalletStateAndHistory{
        address: "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh".to_string(),
        balance: 123456789,
        requests,
        exchanges: vec![],
        boosts,
        transfers,
    }
}
