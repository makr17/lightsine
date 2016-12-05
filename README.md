# lightsine
initial experiment with sACN and rust

very much my first foray into rust...

I'm using strips of APA-102C LED pixels, 60/meter, mounted under the
eaves at home.  they're mounted to 1x2 strips of wood, which are mounted
under the eaves via hinges, so they'll fold down to reveal the strip
just below the eaves.

I'm using [Streaming ACN implementation for Rust](https://github.com/lschmierer/sacn)
to stream sACN over the network to an [Advatek Pixlite 16](http://www.advateklights.com/shop/home/51-pixlite-16-long-range-mkii.html)
which then drives the light strips

the code accounts for the zones (10, 11a, 11b, 12a, 12b, 13) that we
defined while planning this all out.  it knows how many pixels are in
each zone, and how many null pixels at each end of a zone should not be
lit.  it generates three sine waves, one each for red/blue/green, with
settable amplitude and period per-wave.  at each point in time new values
for each color are calculated from the curves, and added at the start
of zone 10, pushing the rest of the values to the right.
