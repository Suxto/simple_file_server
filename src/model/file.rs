use std::collections::BTreeMap;

const READ_MASK: u8 = 0b100u8;
const WRITE_MASK: u8 = 0b010u8;
const VIEW_MASK: u8 = 0b001u8;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Path {
    pub path: String,
    pub name: String,
    pub permission: u8, // rwv - read, write, view
    pub sub_path: BTreeMap<String, Path>,
}

impl Path {
    pub fn can_read(&self) -> bool {
        (self.permission & READ_MASK) == READ_MASK
    }

    pub fn can_write(&self) -> bool {
        (self.permission & WRITE_MASK) == WRITE_MASK
    }

    pub fn can_view(&self) -> bool {
        (self.permission & VIEW_MASK) == VIEW_MASK
    }

    pub fn extract_sub_paths(&mut self) {
        let permission = self.permission;
        self.permission = 0;

        let binding = self.path.clone();
        let path_parts: Vec<&str> = binding.split('/').filter(|s| !s.is_empty()).collect();
        self.insert_paths(&path_parts, 0, permission);
    }

    fn insert_paths(&mut self, parts: &[&str], index: usize, final_permission: u8) {
        if index >= parts.len() {
            if index == 0 {
                self.permission = final_permission;
            }
            return;
        }

        let dir = parts[index];
        let dirs = if dir.starts_with("?") && dir.ends_with("?") {
            dir[1..dir.len() - 1]
                .split("|")
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
        } else {
            vec![dir.to_string()]
        };

        for p in dirs {
            self.sub_path.insert(
                p.clone(),
                Path {
                    path: format!("{}/{}", self.path, p),
                    name: p,
                    permission: 0,
                    sub_path: BTreeMap::new(),
                },
            );
        }

        // 递归处理下一层
        if let Some((_, next)) = self.sub_path.iter_mut().next() {
            next.insert_paths(parts, index + 1, final_permission);
        }

        // 最后一层设置权限
        if index == parts.len() - 1 {
            self.sub_path
                .iter_mut()
                .for_each(|(_, p)| p.permission = final_permission);
        }
    }

    pub fn merge_path(&mut self, from: &Path, permission: u8) {
        self.permission = from.permission | permission;
        from.sub_path.iter().for_each(|(name, other_sub_path)| {
            self.sub_path
                .entry(name.clone())
                .or_insert_with(|| other_sub_path.clone())
                .merge_path(other_sub_path, permission);
        });
    }
}
