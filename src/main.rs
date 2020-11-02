use std::vec::Vec;
use rand;

#[derive(Clone, Copy)]
#[derive(Debug)]
enum OneWayRoadType { Up, Left, Bottom, Right }
#[derive(Clone, Copy)]
#[derive(Debug)]
enum TwoWayRoadType { UpLeft, UpRight, DownLeft, DownRight }
#[derive(Clone, Copy)]
#[derive(Debug)]
enum RoadType {
    OneWay(OneWayRoadType),
    TwoWay(TwoWayRoadType)
}
#[derive(Clone, Copy)]
#[derive(Debug)]
enum ParkType { Free, Full }
#[derive(Clone, Copy)]
#[derive(Debug)]
enum LightCond { Green, Red }
#[derive(Debug)]
#[derive(Clone, Copy)]
enum Tile {
    Building,
    Park(ParkType),
    Light(LightCond),
    Road(RoadType),
}
#[derive(Clone, Copy)]
#[derive(Debug)]
struct Position { x: u32, y: u32 }
type Direction = Position;
#[derive(Debug)]
struct Layout {
    b_num_x: u32,
    b_num_y: u32,
    b_size: u32,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
enum CityType {
    Default,
    Bordered,
    Line
}
#[derive(Debug)]
struct City {
    tiles: Vec<Vec<Tile>>,
    t: CityType,
    layout: Layout,
    entry_points: Vec<Position>,
    buildings: Vec<TargetType>,
}
impl City {
    fn generate_entry_points(city_type: CityType, b_num_x: u32, b_num_y: u32, b_size: u32) -> Vec<Position> {
        let mut entry_point_vec: Vec<Position> = Vec::new();
        match city_type {
            CityType::Default => {
                for i in 1..b_num_x {
                    entry_point_vec.push(Position {
                        x: (b_size + 4)*i - 1,
                        y: 0
                    });
                    entry_point_vec.push(Position {
                        x: (b_size + 4)*i - 1,
                        y: (b_size + 4)*b_num_y - 4
                    });
                }
                for i in 1..b_num_y {
                    entry_point_vec.push(Position {
                        x: 0,
                        y: (b_size + 4)*i - 1
                    });
                    entry_point_vec.push(Position {
                        x: (b_size + 4)*b_num_x - 4,
                        y: (b_size + 4)*i - 1
                    });
                }
            },
            CityType::Bordered => {

            },
            CityType::Line => {
                unimplemented!()
            }
        }
        return entry_point_vec
    }
    fn building_positions(city_type: CityType, b_num_x: u32, b_num_y: u32, b_size: u32) -> Vec<TargetType> {
        match city_type {
            CityType::Default => {

            },
            CityType::Bordered => {

            },
            CityType::Line => {
                unimplemented!()
            }
        }
        return Vec::new()
    }
    fn city_length_x(city_type: CityType, b_num_x: u32, b_size: u32) -> u32 {
        match city_type {
            CityType::Default => { (b_size + 4) * b_num_x - 4 },
            CityType::Bordered => { (b_size + 4) * b_num_x + 2 },
            CityType::Line => {
                unimplemented!()
            }
        }
    }
    fn city_length_y(city_type: CityType, b_num_y: u32, b_size: u32) -> u32 {
        match city_type {
            CityType::Default => { (b_size + 4) * b_num_y - 4 },
            CityType::Bordered => { (b_size + 4) * b_num_y + 2 },
            CityType::Line => {
                unimplemented!()
            }
        }
    }
    fn generate_city_grid(city_type: CityType, b_num_x: u32, b_num_y: u32, b_size: u32) -> Vec<Vec<Tile>>{
        let len_x = City::city_length_x(city_type, b_num_x, b_size);
        let len_y = City::city_length_y(city_type, b_num_y, b_size);
        let mut tiles : Vec<Vec<Tile>> = Vec::with_capacity(len_x as usize);
        for i in 0..len_x {
            tiles.push(vec![Tile::Building; len_y as usize]);
        }
        for i in 0..len_x {
            for j in 0..len_y {
                tiles[i as usize][j as usize] = Tile::Building;
            }
        }
        return tiles
    }
    pub fn new(city_type: CityType, b_num_x: u32, b_num_y: u32, b_size: u32) -> City {
        City {
            t: city_type,
            layout: Layout {
                b_num_x: b_num_x,
                b_num_y: b_num_y,
                b_size: b_size
            },
            tiles: City::generate_city_grid(city_type, b_num_x, b_num_y, b_size),
            entry_points: City::generate_entry_points(city_type, b_num_x, b_num_y, b_size),
            buildings: City::building_positions(city_type, b_num_x, b_num_y, b_size),
        }
    }
}
#[derive(Clone, Copy)]
#[derive(Debug)]
enum TargetType {
    Building(Position),
    Exit(Position),
    JustPosition(Position),
}
#[derive(Debug)]
enum ParkingType {
    Searching,
    Circling(Position, u32),
    Found(Position),
}
#[derive(Debug)]
enum CarState {
    Idle,
    Moving,
    Parking(ParkingType),
    Parked
}
#[derive(Debug)]
struct Car {
    id: u32,
    p: Position,
    dir: Direction,
    target: TargetType,
    state: CarState
}
#[derive(Debug)]
enum SimulationMode {
    Verbose,
    Quite
}
#[derive(Debug)]
struct Simulation {
    city: City,
    epoch: u32,
    cars: Vec<Car>,
    mode: SimulationMode,
    seed: u32,
}
impl Simulation {
    pub fn random_entry_point(city: &City) -> Position {
        let l = city.entry_points.len();
        let r = rand::random::<usize>()%l;
        return city.entry_points[r];
    }
    pub fn random_building(city: &City) -> TargetType {
        let l = city.buildings.len();
        let r = rand::random::<usize>()%l;
        return city.buildings[r];
    }
    pub fn generate_cars(city: &City, n_cars: u32) -> Vec<Car>{
            let mut c_vec = Vec::with_capacity(n_cars as usize);
            for i in 0..n_cars {
                c_vec.push(Car {
                    id: i,
                    p: Simulation::random_entry_point(&city),
                    dir: Position { x: 0, y: 0 },
                    target: TargetType::Building(Position { x: 0, y: 0 }),
                    state: CarState::Idle,
                });
            }
            return c_vec
    }
    pub fn new(
            b_num_x: u32,
            b_num_y: u32,
            b_size: u32,
            n_cars: u32,
            city_type: CityType,
            seed: u32,
            mode: SimulationMode
    ) -> Simulation {
        let city = City::new(city_type, b_num_x, b_num_y, b_size);
        let cars =  Simulation::generate_cars(&city, n_cars);
        Simulation {
            city: city,
            epoch: 0,
            cars: cars,
            mode: mode,
            seed: seed,
        }
    }

}
fn main() {
    let mut sim = Simulation::new(
        2,
        2,
        3,
        4,
        CityType::Default,
        0,
        SimulationMode::Verbose
    );
    print!("{:#?}",sim);
}
