use std::collections::BTreeMap;

use crate::model::file::Path;

pub fn check_permission(
    user_permissions_tree: &Path,
    requested_path: &str,
    required_permission: u8,
) -> bool {
    let path_parts: Vec<&str> = requested_path.split('/').filter(|s| !s.is_empty()).collect();
    check_permission_recursive(user_permissions_tree, &path_parts, required_permission)
}

fn check_permission_recursive(
    current_path: &Path,
    remaining_parts: &[&str],
    required_permission: u8,
) -> bool {
    if remaining_parts.is_empty() {
        // 检查当前路径是否有权限
        has_permission(current_path, required_permission)
    } else {
        // 检查当前路径是否有权限
        if !has_permission(current_path, required_permission) {
            return false;
        }

        let part = remaining_parts[0];
        let rest_parts = &remaining_parts[1..];

        // 如果有子路径，检查子路径权限
        if let Some(sub_path) = current_path.sub_path.get(part) {
            return check_permission_recursive(sub_path, rest_parts, required_permission);
        }

        // 如果没有找到特定子路径，但当前路径有权限，可以继续
        true
    }
}

fn has_permission(path: &Path, required_permission: u8) -> bool {
    (path.permission & required_permission) == required_permission
}