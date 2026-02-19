use crate::models::{Channel, ChannelCategory, ChannelKind, Message, Server, User, UserStatus};

pub fn make_user(id: &str, name: &str, disc: &str, status: UserStatus) -> User {
    User {
        id: id.to_string(),
        username: name.to_string(),
        discriminator: disc.to_string(),
        status,
    }
}

pub fn make_servers() -> Vec<Server> {
    vec![
        Server {
            id: "1".to_string(),
            name: "Tripwire HQ".to_string(),
            categories: vec![
                ChannelCategory {
                    name: "Information".to_string(),
                    channels: vec![
                        Channel {
                            id: "101".to_string(),
                            name: "announcements".to_string(),
                            kind: ChannelKind::Announcement,
                            unread: 1,
                            topic: Some("Official announcements only.".to_string()),
                        },
                        Channel {
                            id: "102".to_string(),
                            name: "rules".to_string(),
                            kind: ChannelKind::Text,
                            unread: 0,
                            topic: Some("Read before participating.".to_string()),
                        },
                    ],
                    collapsed: false,
                },
                ChannelCategory {
                    name: "General".to_string(),
                    channels: vec![
                        Channel {
                            id: "103".to_string(),
                            name: "general".to_string(),
                            kind: ChannelKind::Text,
                            unread: 5,
                            topic: Some("Chat about anything!".to_string()),
                        },
                        Channel {
                            id: "104".to_string(),
                            name: "introductions".to_string(),
                            kind: ChannelKind::Text,
                            unread: 0,
                            topic: Some("Introduce yourself to the community.".to_string()),
                        },
                        Channel {
                            id: "105".to_string(),
                            name: "off-topic".to_string(),
                            kind: ChannelKind::Text,
                            unread: 12,
                            topic: None,
                        },
                    ],
                    collapsed: false,
                },
                ChannelCategory {
                    name: "Voice".to_string(),
                    channels: vec![
                        Channel {
                            id: "106".to_string(),
                            name: "Lounge".to_string(),
                            kind: ChannelKind::Voice,
                            unread: 0,
                            topic: None,
                        },
                        Channel {
                            id: "107".to_string(),
                            name: "Gaming".to_string(),
                            kind: ChannelKind::Voice,
                            unread: 0,
                            topic: None,
                        },
                    ],
                    collapsed: false,
                },
            ],
            members: vec![
                make_user("u1", "Alice", "0001", UserStatus::Online),
                make_user("u2", "Bob", "0002", UserStatus::Idle),
                make_user("u3", "Carol", "0003", UserStatus::DoNotDisturb),
                make_user("u4", "Dave", "0004", UserStatus::Offline),
                make_user("u5", "Eve", "0005", UserStatus::Offline),
            ],
            notification_count: 0,
        },
        Server {
            id: "2".to_string(),
            name: "Dev Corner".to_string(),
            categories: vec![
                ChannelCategory {
                    name: "Dev".to_string(),
                    channels: vec![
                        Channel {
                            id: "201".to_string(),
                            name: "general-dev".to_string(),
                            kind: ChannelKind::Text,
                            unread: 3,
                            topic: Some("Development discussion".to_string()),
                        },
                        Channel {
                            id: "202".to_string(),
                            name: "rust".to_string(),
                            kind: ChannelKind::Text,
                            unread: 0,
                            topic: Some("Rustaceans unite!".to_string()),
                        },
                        Channel {
                            id: "203".to_string(),
                            name: "code-review".to_string(),
                            kind: ChannelKind::Text,
                            unread: 2,
                            topic: None,
                        },
                    ],
                    collapsed: false,
                },
            ],
            members: vec![
                make_user("u1", "Alice", "0001", UserStatus::Online),
                make_user("u6", "Frank", "0006", UserStatus::Online),
                make_user("u7", "Grace", "0007", UserStatus::Offline),
            ],
            notification_count: 5,
        },
        Server {
            id: "3".to_string(),
            name: "Design Lab".to_string(),
            categories: vec![
                ChannelCategory {
                    name: "Design".to_string(),
                    channels: vec![
                        Channel {
                            id: "301".to_string(),
                            name: "inspiration".to_string(),
                            kind: ChannelKind::Text,
                            unread: 0,
                            topic: Some("Share design inspiration".to_string()),
                        },
                        Channel {
                            id: "302".to_string(),
                            name: "feedback".to_string(),
                            kind: ChannelKind::Text,
                            unread: 7,
                            topic: None,
                        },
                    ],
                    collapsed: false,
                },
            ],
            members: vec![
                make_user("u8", "Hank", "0008", UserStatus::Online),
                make_user("u9", "Iris", "0009", UserStatus::Idle),
            ],
            notification_count: 7,
        },
    ]
}

pub fn make_messages_for(channel_id: &str) -> Vec<Message> {
    let alice = make_user("u1", "Alice", "0001", UserStatus::Online);
    let bob = make_user("u2", "Bob", "0002", UserStatus::Idle);
    let carol = make_user("u3", "Carol", "0003", UserStatus::DoNotDisturb);

    match channel_id {
        "103" => vec![
            Message {
                id: "m1".to_string(),
                author: alice.clone(),
                content: "Hey everyone! How's it going? 👋".to_string(),
                timestamp: "Today at 9:00 AM".to_string(),
                edited: false,
            },
            Message {
                id: "m2".to_string(),
                author: bob.clone(),
                content: "Doing great! Just finished setting up Tripwire locally.".to_string(),
                timestamp: "Today at 9:02 AM".to_string(),
                edited: false,
            },
            Message {
                id: "m3".to_string(),
                author: carol.clone(),
                content: "This UI is looking amazing. Love the Discord vibe!".to_string(),
                timestamp: "Today at 9:05 AM".to_string(),
                edited: false,
            },
            Message {
                id: "m4".to_string(),
                author: alice.clone(),
                content: "Thanks! Built entirely with GPUI components. The component library is fantastic.".to_string(),
                timestamp: "Today at 9:07 AM".to_string(),
                edited: true,
            },
            Message {
                id: "m5".to_string(),
                author: bob.clone(),
                content: "I saw the auth screen — nice touch with the dev bypass button for testing.".to_string(),
                timestamp: "Today at 9:10 AM".to_string(),
                edited: false,
            },
            Message {
                id: "m6".to_string(),
                author: carol.clone(),
                content: "The resizable panels from the Dock system would make a great addition here.".to_string(),
                timestamp: "Today at 9:12 AM".to_string(),
                edited: false,
            },
            Message {
                id: "m7".to_string(),
                author: alice.clone(),
                content: "100% on the roadmap. Also planning markdown rendering for messages using the Text component.".to_string(),
                timestamp: "Today at 9:15 AM".to_string(),
                edited: false,
            },
        ],
        "101" => vec![
            Message {
                id: "a1".to_string(),
                author: alice.clone(),
                content: "🎉 Welcome to Tripwire! This is our brand-new communication platform.".to_string(),
                timestamp: "Yesterday at 8:00 AM".to_string(),
                edited: false,
            },
            Message {
                id: "a2".to_string(),
                author: alice.clone(),
                content: "We're in early alpha — expect rapid changes. Your feedback is welcome in #general.".to_string(),
                timestamp: "Yesterday at 8:01 AM".to_string(),
                edited: false,
            },
        ],
        "201" => vec![
            Message {
                id: "d1".to_string(),
                author: make_user("u6", "Frank", "0006", UserStatus::Online),
                content: "Anyone familiar with the GPUI entity system? I'm trying to share state across views.".to_string(),
                timestamp: "Today at 10:30 AM".to_string(),
                edited: false,
            },
            Message {
                id: "d2".to_string(),
                author: alice.clone(),
                content: "Use a Global or pass Entity<T> handles around. Globals are easiest for app-wide state.".to_string(),
                timestamp: "Today at 10:35 AM".to_string(),
                edited: false,
            },
        ],
        _ => vec![
            Message {
                id: "empty1".to_string(),
                author: alice,
                content: "Be the first to send a message in this channel!".to_string(),
                timestamp: "Today".to_string(),
                edited: false,
            },
        ],
    }
}
