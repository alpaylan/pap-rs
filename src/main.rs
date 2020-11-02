use std::vec::Vec;
use rand;
use std::ops::{
    Add,
    Sub,
};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum OneWayRoadType { Up, Left, Bottom, Right }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum TwoWayRoadType { UpLeft, UpRight, DownLeft, DownRight }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum RoadType {
    OneWay(OneWayRoadType),
    TwoWay(TwoWayRoadType)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum ParkType { Free, Full }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum LightCond { Green, Red }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Tile {
    Building,
    Park(ParkType),
    Light(LightCond),
    Road(RoadType),
    Unknown,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Position { x: u32, y: u32 }
type Direction = Position;

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        return Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;
    fn sub(self, rhs: Position) -> Self::Output {
        return Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


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
    fn calculate_tile_type(city_type: CityType, b_size: u32, i: u32, j: u32) -> Tile {
        let bottom = RoadType::OneWay(OneWayRoadType::Bottom);
        let up = RoadType::OneWay(OneWayRoadType::Up);
        let left = RoadType::OneWay(OneWayRoadType::Left);
        let right = RoadType::OneWay(OneWayRoadType::Right);
        let tile_type: Tile = match city_type {
            CityType::Default => {
                let unit = (b_size+4) as i64;
                let unit_y = i as i64 % unit as i64 - b_size as i64;
                let unit_x = j as i64 % unit as i64 - b_size as i64;
                let tile_type_x : Tile = match unit_x {
                    0 | 3 => { Tile::Park(ParkType::Free) },
                    1 => { Tile::Road(bottom) },
                    2 => { Tile::Road(up) },
                    _ => { Tile::Building }
                };
                let tile_type_y : Tile = match unit_y {
                    0 | 3 => { Tile::Park(ParkType::Free) },
                    1 => { Tile::Road(left) },
                    2 => { Tile::Road(right) },
                    _ => { Tile::Building }
                };
                let tile =
                    match tile_type_x {
                        Tile::Park(..) => {
                            match tile_type_y {
                                Tile::Park(..) => {
                                    Tile::Light(LightCond::Green)
                                },
                                Tile::Building => {
                                    Tile::Park(ParkType::Free)
                                },
                                Tile::Road(x) => {
                                    Tile::Road(x)
                                }
                                _ => {
                                    Tile::Unknown
                                }
                            }
                        },
                        Tile::Building => {
                            match tile_type_y {
                                Tile::Building => {
                                    Tile::Building
                                },
                                Tile::Park(..) => {
                                    Tile::Park(ParkType::Free)
                                },
                                Tile::Road(x) => {
                                    Tile::Road(x)
                                },
                                _ => {
                                    Tile::Unknown
                                }
                            }
                        },
                        Tile::Road(x) => {
                            match tile_type_y {
                                Tile::Building | Tile::Park(..)=> {
                                    Tile::Road(x)
                                },
                                Tile::Road(y) => {
                                    match (y, x) {
                                        (RoadType::OneWay(OneWayRoadType::Right),
                                        RoadType::OneWay(OneWayRoadType::Up))  => {
                                            Tile::Road(RoadType::TwoWay(TwoWayRoadType::UpRight))
                                        },
                                        (RoadType::OneWay(OneWayRoadType::Left),
                                            RoadType::OneWay(OneWayRoadType::Up))  => {
                                            Tile::Road(RoadType::TwoWay(TwoWayRoadType::UpLeft))
                                        },
                                        (RoadType::OneWay(OneWayRoadType::Right),
                                            RoadType::OneWay(OneWayRoadType::Bottom))  => {
                                            Tile::Road(RoadType::TwoWay(TwoWayRoadType::DownRight))
                                        },
                                        (RoadType::OneWay(OneWayRoadType::Left),
                                            RoadType::OneWay(OneWayRoadType::Bottom))  => {
                                            Tile::Road(RoadType::TwoWay(TwoWayRoadType::DownLeft))
                                        },
                                        _ => {
                                            Tile::Unknown
                                        }
                                    }
                                },
                                _ => {
                                Tile::Unknown
                            }
                            }
                        }
                        _ => {
                            Tile::Unknown
                        }
                    };
                return tile
            },
            CityType::Bordered => {
                Tile::Unknown
            },
            CityType::Line => {
                Tile::Unknown
            },
            _ => {
                Tile::Unknown
            }
        };
        return tile_type;
    }
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
        for _ in 0..len_x {
            tiles.push(vec![Tile::Building; len_y as usize]);
        }
        for i in 0..len_x {
            for j in 0..len_y {
                tiles[i as usize][j as usize] = City::calculate_tile_type(city_type, b_size, i, j);
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
    pub fn generate_cars(city: &City, n_cars: u32) -> Vec<Car> {
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
