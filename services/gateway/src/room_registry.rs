#[derive(Clone, Copy)]
pub struct RoomTarget {
    pub slug: &'static str,
    pub base_url: &'static str,
    pub actions: &'static [&'static str],
}

impl RoomTarget {
    pub fn action_url(&self, action: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            action.trim_start_matches('/')
        )
    }
}

#[derive(Clone)]
pub struct RoomRegistry {
    targets: &'static [RoomTarget],
}

impl RoomRegistry {
    pub fn default() -> Self {
        Self {
            targets: &[
                RoomTarget {
                    slug: "rce",
                    base_url: "http://room-rce:9000",
                    actions: &["exec"],
                },
                RoomTarget {
                    slug: "xss",
                    base_url: "http://room-xss:9000",
                    actions: &["post", "comments", "change-password"],
                },
            ],
        }
    }

    pub fn get(&self, slug: &str) -> Option<RoomTarget> {
        self.targets
            .iter()
            .find(|target| target.slug == slug)
            .copied()
    }
}
