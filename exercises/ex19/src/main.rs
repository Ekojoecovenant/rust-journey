#[derive(Debug)]
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
}

impl Weather {
    fn description(&self) -> &str {
        match self {
            Self::Cloudy => "Overcast with grey clouds, no direct sunlight",
            Self::Rainy => "Steady rain with possible light showers",
            Self::Snowy => "Heavy snowfall with possible accumulation",
            Self::Stormy => "Severe thunderstorms",
            Self::Sunny => "Clear skies and sunshine",
        }
    }

    fn advice(&self) -> &str {
        match self {
            Self::Cloudy => "Perfect for a light jacket, good day for photography",
            Self::Rainy => "Carry an umbrella and wear waterproof shoes",
            Self::Snowy => "Dress warmly, watch for slippery surfaces",
            Self::Stormy => "Stay indoors and away from windows!", // yes use linux 😂😂
            Self::Sunny => "Great day for outdoor activities!",
        }
    }

    fn temperature_range(&self) -> (i32, i32) {
        match self {
            Self::Cloudy => (18, 28),
            Self::Rainy => (15, 22),
            Self::Snowy => (-5, 5),
            Self::Stormy => (10, 18),
            Self::Sunny => (25, 35),
        }
    }

    fn is_dangerous(&self) -> bool {
        match self {
            Self::Cloudy => false,
            Self::Rainy => false,
            Self::Snowy => false,
            Self::Stormy => true,
            Self::Sunny => false,
        }
    }
}

fn main() {
    let weathers = [
        Weather::Sunny,
        Weather::Stormy,
        Weather::Rainy,
        Weather::Cloudy,
        Weather::Snowy,
    ];

    println!("=== Daily Forecast ===");
    let mut day = 1usize;

    for weather in &weathers {
        println!("\nDay {}: {:?}", day, weather);

        println!("  Description : {}", weather.description());
        println!("  Advice      : {}", weather.advice());
        println!("  Temp Range  : {}", {
            let (a, b) = weather.temperature_range();
            format!("{}°C to {}°C", a, b)
        });
        println!("  Dangerous   : {}", weather.is_dangerous());

        day += 1;
    }
}
