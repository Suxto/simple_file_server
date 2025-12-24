use std::collections::HashSet;

pub struct Path {
    pub path: String,
    pub name: String,
    pub permission: u8, // rwv - read, write, view
    pub sub_path: HashSet<Path>,
}

impl Path {
    pub fn can_read(&self) -> bool {
        (self.permission & 0b100) == 0b100
    }

    pub fn can_write(&self) -> bool {
        (self.permission & 0b010) == 0b010
    }

    pub fn can_view(&self) -> bool {
        (self.permission & 0b001) == 0b001
    }

    pub fn extract_sub_paths(&mut self) {
        let mut current = self;
        self.path.split('/').for_each(|dir| {
            if dir.starts_with("?") && dir.ends_with("?") {
                dir[1..dir.len() - 1].split("|").collect();
            } else {
                vec![dir.to_string()]
            }
            .into_iter()
            .for_each(|p| {
                current.sub_path.insert(Path {
                    path: current.path + "/" + p,
                    name: p,
                    permission: 0,
                    sub_path: HashSet::new(),
                });
            });
        });
    }
}
