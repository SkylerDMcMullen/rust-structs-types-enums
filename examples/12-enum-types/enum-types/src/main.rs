#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Veneto,
}

// create an iterable for all the enum types
impl WineRegions {
    fn all() -> Vec<WineRegions> {
        vec![
            WineRegions::Bordeaux,
            WineRegions::Burgundy,
            WineRegions::Champagne,
            WineRegions::Tuscany,
            WineRegions::Rioja,
            WineRegions::NapaValley,
            WineRegions::Veneto,
        ]
    }
}

#[derive(Debug)]
struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn wine_popularity(w: &WineRegions) -> String {
    match w {
        WineRegions::Veneto => "Insanely popular!".to_string(),
        WineRegions::Tuscany => "Very popular".to_string(),
        WineRegions::Bordeaux => "Pretty popular".to_string(),
        _ => "Not popular at all".to_string(),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Pinot Gris"),
        region: WineRegions::Veneto,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    println!("Wine 3: {:?}", wine3);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    supported_regions(&WineRegions::Veneto);

    for region in WineRegions::all() {
        println!("{:?}: {}", region, wine_popularity(&region));
    }

    Option

}
