#![allow(dead_code)]
#[derive(Debug)]
enum DrumPiece {
    Kick,
    Snare,
    HighTom,
    MidTom,
    FloorTom,
    HiHat,
    Crash,
    Ride,
    Splash,
}

#[derive(Debug)]
enum MusicalInstrument {
    Piano,
    Guitar,
    Bass,
    DrumKit(DrumPiece),
}

fn can_i_play(gear: &MusicalInstrument) {
    match gear {
        MusicalInstrument::Guitar => println!("I can play a {:?}", gear),
        MusicalInstrument::DrumKit(piece) => println!("I totally play this {:?}", piece),
        _ => println!("I play some"),
    }
}

fn main() {
    let gear = MusicalInstrument::DrumKit(DrumPiece::Snare);
    can_i_play(&gear);

    let gear2 = MusicalInstrument::Bass;
    can_i_play(&gear2);

    let gear3 = MusicalInstrument::Piano;
    can_i_play(&gear3);

    let gear4 = MusicalInstrument::Guitar;
    can_i_play(&gear4);
}
