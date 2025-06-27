#[derive(Debug)]
enum Channel {
    Stable(Semver),
    Beta(Semver),
    Nightly,
}

#[derive(Debug)]
struct Semver {
    major: u32,
    minor: u32,
    patch: u32,
}

fn release_info() -> Channel {
    Channel::Stable(Semver {
        major: 1,
        minor: 88,
        patch: 0,
    })
}

fn maybe_number() -> Option<u32> {
    Some(42)
}

fn main() {
    // Example 1: Your original example
    if let Channel::Stable(v) = release_info()
        && let Semver { major, minor, .. } = v
        && major == 1
        && minor == 88
    {
        println!("`let_chains` was stabilized in this version");
    }

    // Example 2: Matching Beta channel
    if let Channel::Beta(v) = Channel::Beta(Semver {
        major: 1,
        minor: 90,
        patch: 0,
    }) && let Semver { minor, .. } = v
        && minor >= 90
    {
        println!("Beta channel with minor >= 90: {:?}", v);
    }

    // Example 3: Combining enum match and Option
    if let Channel::Stable(v) = release_info()
        && let Some(n) = maybe_number()
        && n > 40
    {
        println!("Stable release {:?} and number {} > 40", v, n);
    }

    // Example 4: Fallback case
    if let Channel::Nightly = Channel::Nightly {}
    {
        println!("This is the nightly channel");
    }
}
