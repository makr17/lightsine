use std::cmp::Ordering;
use std::f32;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

extern crate sacn;
use sacn::DmxSource;

struct Wave  { amp: f32, per: f32 }
struct Zone  { head: u8, body: u8, tail: u8, name: String }

fn main() {
    let mut dmx_source = DmxSource::new("Controller").unwrap();

    let zones: [Zone; 6] = [
        Zone { head: 3, body: 47, tail: 0, name: "10".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11a".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11b".to_string() },
        Zone { head: 2, body: 90, tail: 3, name: "12a".to_string() },
        Zone { head: 2, body: 91, tail: 3, name: "12b".to_string() },
        Zone { head: 2, body: 43, tail: 0, name: "13".to_string() }
    ];

    // R, G, B per pixel, so * 3
    let live: u16 = zones.iter().map(|x| (x.body as u16) * 3).sum();
    
    // setup the curves
    let red   = Wave { amp: 0.9,  per:  5.0 };
    let blue  = Wave { amp: 0.75, per:  7.0 };
    let green = Wave { amp: 1.0,  per: 11.0 };

    let mut max_amps = [red.amp, blue.amp, green.amp];
    max_amps.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    let max_amp = max_amps[0];
    
    let mut lights = Vec::new();
    lights.reserve(live as usize + 3);
    let mut t:u32 = 0;
    loop {
        t += 1;
        lights.push(red.amp   * (red.per   * t as f32).sin());
        lights.push(green.amp * (green.per * t as f32).sin());
        lights.push(blue.amp  * (blue.per  * t as f32).sin());
        lights.truncate(live as usize);
        println!("{:?}", &lights);
        sleep(Duration::new(0, 500_000_000));
    }

    //dmx_source.send(1, &[0, 1, 2]);
    // ...

    // terminate the stream for a specific universe
    //dmx_source.terminate_stream(1);
}
