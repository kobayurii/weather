# weather
###dependencies
rust - 1.64.0

### Compile

```bash
$ cargo build --release
```

### Run configuration 

```bash
$ ./target/release/weather config
```
Follow the instructions to create the configuration
#### Parameters
```
Provider
API_KEY
Latitude
Longitude
```
To get the Latitude and Longitude you can use the https://www.latlong.net/ service
#### Supported providers 
* AccuWeather
* OpenWeather

### Run get weather

```bash
$ ./target/release/weather get
```

#### Expected output
```
City           : Kyiv
Weather        : Clear
Temperature    : 9.53 °C
Feels Like     : 6.65 °C
Pressure       : 1007 mBar
Humidity       : 77 %
Wind Speed     : 6 m/s
Wind Direction : 194 degree
```