use std::collections::HashMap;

use once_cell::sync::Lazy;

use rucman::Version;

pub static VERSIONS: Lazy<HashMap<&'static str, Version>> = Lazy::new(|| {
    HashMap::from([
        (
            "1:2.3.5+r3+gd9d61d87f-3",
            Version::new(1, "2.3.5+r3+gd9d61d87f", 3),
        ),
        ("2.28.3-1", Version::new(None, "2.28.3", 1)),
        ("2.3.2.post1-1", Version::new(None, "2.3.2.post1", 1)),
        (
            "20220913.f09bebf-1",
            Version::new(None, "20220913.f09bebf", 1),
        ),
        (
            "2:2.06.r322.gd9b4638c5-4",
            Version::new(2, "2.06.r322.gd9b4638c5", 4),
        ),
        ("4.3-3", Version::new(None, "4.3", 3)),
        (
            "6.04.pre2.r11.gbf6db5b4-3",
            Version::new(None, "6.04.pre2.r11.gbf6db5b4", 3),
        ),
        ("7.4.3-1", Version::new(None, "7.4.3", 1)),
        ("r2322+3aebf69d-1", Version::new(None, "r2322+3aebf69d", 1)),
        ("0.4.4-1", Version::new(None, "0.4.4", 1)),
        ("2.14.2-363", Version::new(None, "2.14.2", 363)),
    ])
});
