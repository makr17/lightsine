use std::cmp::Ordering;
use std::f32;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

extern crate sacn;
use sacn::DmxSource;

struct Wave  { amp: f32, per: f32 }
struct Zone  { head: u8, body: u8, tail: u8, name: String }

const PIXEL_SIZE: u8 = 3;
const MAX_INTENSITY: f32 = 100_f32;
const UNIVERSE_SIZE: u16 = 510 * 3;

fn main() {
    let mut dmx_source = DmxSource::new("Controller").unwrap();

    let refresh = Duration::new(0, 200_000_000);
    
    let zones: [Zone; 6] = [
        Zone { head: 3, body: 47, tail: 0, name: "10".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11a".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11b".to_string() },
        Zone { head: 2, body: 90, tail: 3, name: "12a".to_string() },
        Zone { head: 2, body: 91, tail: 3, name: "12b".to_string() },
        Zone { head: 2, body: 43, tail: 0, name: "13".to_string() }
    ];

    // R, G, B per pixel, so * 3
    let live: u16 = zones.iter().map(|x| (x.body as u16) * PIXEL_SIZE as u16).sum();

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
        lights.push(wave_value(&red,   t, max_amp));
        lights.push(wave_value(&green, t, max_amp));
        lights.push(wave_value(&blue,  t, max_amp));
        // reverse before truncate so we're dropping oldest values
        // plus we want to output the values reversed
        lights.reverse();
        // only keep enough values for the live pixels
        lights.truncate(live as usize);
        // copy to we can play with the data
        let mut copy = lights.clone();
        lights.reverse();
        let mut offset: usize = 0;
        // splice in 0 values for null pixels in .head and .tail
        for zone in zones.iter() {
            let mut idx = offset;
            if idx > copy.len() as usize {
                break;
            }
            for n in 0..(zone.head * PIXEL_SIZE) {
                copy.insert(idx as usize, 0);
            }
            idx += zone.head as usize * PIXEL_SIZE as usize + zone.body as usize * PIXEL_SIZE as usize;
            if idx > copy.len() as usize {
                break;
            }
            for n in 0..(zone.tail * PIXEL_SIZE) {
                copy.insert(idx, 0);
            }
            offset += (zone.head as usize + zone.body as usize + zone.tail as usize) * PIXEL_SIZE as usize;
        }
        let output = copy.as_slice();
        println!("{:?}", output);
        sleep(refresh);
    }

    //dmx_source.send(1, &[0, 1, 2]);
    // ...

    // terminate the stream for a specific universe
    //dmx_source.terminate_stream(1);
}

fn wave_value (f: &Wave, t: u32, max_amp: f32) -> u8 {
    (MAX_INTENSITY * (f.amp * (f.per * t as f32).sin() + f.amp)/(max_amp * 2_f32)) as u8
}
