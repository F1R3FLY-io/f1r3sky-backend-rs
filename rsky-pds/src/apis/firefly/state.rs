use chrono::TimeZone;

use crate::apis::firefly::models::{
    Boost,
    Direction,
    Request,
    RequestStatus,
    Transfer,
    WalletStateAndHistory,
};

pub fn example_wallet_history() -> WalletStateAndHistory {
    let requests = vec![
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 12, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 1000000000000000,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 11, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 1000000000000000,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 2, 22, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 6000000000000000,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 2, 21, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 3000000000000000,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 2, 20, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 1000000000000000,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 1, 14, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 2000000000000000,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2025, 1, 7, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 7000000000000000,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2024, 12, 3, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 2000000000000000,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2024, 12, 2, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 7000000000000000,
            status: RequestStatus::CANCELLED,
        },
        Request {
            id: "deade3123f".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2024, 11, 24, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 4000000000000000,
            status: RequestStatus::ONGOING,
        },
        Request {
            id: "darggxc45".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2024, 11, 23, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 3000000000000000,
            status: RequestStatus::DONE,
        },
        Request {
            id: "j634gf5".to_string(),
            date: chrono::Utc
                .with_ymd_and_hms(2024, 11, 22, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 2000000000000000,
            status: RequestStatus::CANCELLED,
        },
    ];
    let boosts = vec![
        Boost {
            id: "deade3123f".to_string(),
            direction: Direction::OUTGOING,
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 22, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 3000000000000000,
            username: "foo.test".to_string(),
            post: Some("3lmeun6nxfc27".to_string()),
        },
        Boost {
            id: "darggxc45".to_string(),
            direction: Direction::INCOMING,
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 21, 12, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 1000000000000000,
            username: "SomeUser33".to_string(),
            post: Some("1234412".to_string()),
        },
    ];
    let transfers = vec![
        Transfer {
            id: "j634gf5".to_string(),
            direction: Direction::INCOMING,
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 21, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 10000000000000000,
            to_address: "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA".to_string(),
        },
        Transfer {
            id: "darggxc45".to_string(),
            direction: Direction::OUTGOING,
            date: chrono::Utc
                .with_ymd_and_hms(2025, 3, 20, 11, 0, 0)
                .unwrap()
                .timestamp_millis() as _,
            amount: 10000000000000000,
            to_address: "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA".to_string(),
        },
    ];

    WalletStateAndHistory {
        address: "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh".to_string(),
        balance: 123456789,
        requests,
        exchanges: vec![],
        boosts,
        transfers,
    }
}
