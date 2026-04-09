use std::fmt;

trait Media {
    fn title(&self) -> String;

    fn duration_mins(&self) -> f64;

    fn media_type(&self) -> String;

    fn describe(&self) {
        println!(
            "[{}] {} ({} mins)",
            self.media_type(),
            self.title(),
            self.duration_mins()
        );
    }
}

struct Song {
    title: String,
    artist: String,
    duration_mins: f64,
}

struct Podcast {
    title: String,
    host: String,
    duration_mins: f64,
    episode: u32,
}

struct Audiobook {
    title: String,
    author: String,
    duration_mins: f64,
    chapters: u32,
}

impl Media for Song {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn media_type(&self) -> String {
        String::from("Song")
    }

    fn duration_mins(&self) -> f64 {
        self.duration_mins
    }
}

impl Media for Podcast {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn media_type(&self) -> String {
        String::from("Podcast")
    }

    fn duration_mins(&self) -> f64 {
        self.duration_mins
    }
}

impl Media for Audiobook {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn media_type(&self) -> String {
        String::from("Audiobook")
    }

    fn duration_mins(&self) -> f64 {
        self.duration_mins
    }
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {} [{} mins]",
            self.title, self.artist, self.duration_mins
        )
    }
}

impl fmt::Display for Podcast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - Episode {} hosted by {}",
            self.title, self.episode, self.host
        )
    }
}

impl fmt::Display for Audiobook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {} - {} chapters",
            self.title, self.author, self.chapters
        )
    }
}

fn print_media(item: &impl Media) {
    item.describe();
}

fn longest<T: Media, U: Media>(a: &T, b: &U) -> String {
    let dur_a = a.duration_mins();
    let dur_b = b.duration_mins();
    if dur_a == dur_b {
        "Equal".to_string()
    } else if dur_a > dur_b {
        a.title()
    } else {
        b.title()
    }
}

fn total_duration(items: &[&dyn Media]) -> f64 {
    let mut duration = 0f64;
    for item in items {
        duration += item.duration_mins();
    }
    duration
}

fn main() {
    let song: Song = Song {
        title: "Smooth Criminal".to_string(),
        artist: "Michael Jackson".to_string(),
        duration_mins: 5.7,
    };

    let podcast: Podcast = Podcast {
        title: "AI fears".to_string(),
        host: "Anthropic".to_string(),
        duration_mins: 90.6,
        episode: 3,
    };

    let audiobook: Audiobook = Audiobook {
        title: "How to think like a developer".to_string(),
        author: "Brian Tracy".to_string(), // 🌚
        duration_mins: 40.3,
        chapters: 4,
    };

    print_media(&song);
    println!("");
    print_media(&podcast);
    println!("");
    print_media(&audiobook);

    println!("\nLongest : {}", longest(&song, &podcast));
    println!("Longest : {}", longest(&song, &audiobook));

    let items: Vec<&dyn Media> = vec![&song, &podcast, &audiobook];
    println!("\nTotal duration : {}", total_duration(&items))
}
