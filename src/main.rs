// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {    
        /*
            Convert to 1 orbital earth in years
         */   
        Self { seconds: s as f64  }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        let per_orbital_earth_in_years: f64 =  d.seconds / 31557600.9 ;
        per_orbital_earth_in_years
    }

    fn per_orbital_earth_in_seconds() -> f64{
        return 31557600.0
    }
}

pub struct Earth;
pub struct Mercury;
pub struct Venus;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 0.2408467;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
    
}
impl Planet for Venus {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 0.61519726;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 1.0;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 1.8808158;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 11.862615;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}
impl Planet for Saturn {
    
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 29.447498;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }

}
impl Planet for Uranus {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 84.016846;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration ) -> f64 {
        let per_orbital_earth_in_seconds: f64 = Self::per_orbital_earth_in_seconds() * 164.79132;
        let per_orbital_earth_in_years: f64 = d.seconds / per_orbital_earth_in_seconds;
        per_orbital_earth_in_years
    }
}

fn age_on_earth() {
    let seconds = 1000000000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    let expected = 31.69;
    assert_in_delta(expected, output);
}
fn age_on_venus() {
    let seconds = 189839836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);
    let expected = 9.78;
    assert_in_delta(expected, output);
}

fn age_on_mercury() {
    let seconds = 2134835688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    assert_in_delta(expected, output);
}

fn age_on_mars() {
    let seconds = 2129871239;
    let duration = Duration::from(seconds);
    let output = Mars::years_during(&duration);
    let expected = 35.88;
    assert_in_delta(expected, output);
}
fn age_on_jupiter() {
    let seconds = 901876382;
    let duration = Duration::from(seconds);
    let output = Jupiter::years_during(&duration);
    let expected = 2.41;
    assert_in_delta(expected, output);
}
fn age_on_saturn() {
    let seconds = 2000000000;
    let duration = Duration::from(seconds);
    let output = Saturn::years_during(&duration);
    let expected = 2.15;
    assert_in_delta(expected, output);
}
fn age_on_uranus() {
    let seconds = 1210123456;
    let duration = Duration::from(seconds);
    let output = Uranus::years_during(&duration);
    let expected = 0.46;
    assert_in_delta(expected, output);
}
fn age_on_neptune() {
    let seconds = 1821023456;
    let duration = Duration::from(seconds);
    let output = Neptune::years_during(&duration);
    let expected = 0.35;
    assert_in_delta(expected, output);
}
fn main(){
    age_on_earth();
    age_on_venus();
    age_on_jupiter();
    age_on_mars();
    age_on_mercury();
    age_on_saturn();
    age_on_uranus();
    age_on_neptune();
}