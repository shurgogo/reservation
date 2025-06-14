use regex::Regex;


fn main() {
    let s = "Key (resource_id, timespan)=(ocean-view-room-777, [\"2025-05-14 22:00:00+00\",\"2025-05-16 19:00:00+00\")) conflicts with existing key (resource_id, timespan)=(ocean-view-room-777, [\"2025-05-13 22:00:00+00\",\"2025-05-15 19:00:00+00\"))."
    let re = Regex::new(
            r#"resource_id,\s*timespan\)=\(([^,]+),\s*\["([^"\\]+(?:\\.[^"\\]*)*)",\s*"([^"\\]+(?:\\.[^"\\]*)*)"\]\)"#,
        ).unwrap();

    println!("resource_id");
}
