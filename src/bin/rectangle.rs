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

fn main() {
    let gear = MusicalInstrument::DrumKit(DrumPiece::Snare);
    println!("It's a {:?}", gear);
}
