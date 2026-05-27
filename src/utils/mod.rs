use std::fmt::Display;

pub(crate) fn format_list_truncated(mut iter: impl Iterator<Item = impl Display>) -> String {
    let mut buffer = String::new();
    buffer.push('[');
    if let Some(next) = iter.next() {
        buffer.push_str(&next.to_string());
    }
    if iter.next().is_some() {
        buffer.push_str(", ...]");
    } else {
        buffer.push(']');
    }
    buffer
}
