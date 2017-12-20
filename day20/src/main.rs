extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct V3 {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Debug)]
struct Particle {
    id: usize,
    position: V3,
    velocity: V3,
    acceleration: V3
}

fn main() {
    let input = include_str!("input.txt").trim();

    let mut particles = parse_particles(input);

    let mut closest_particle_last1000 = None;
    for iteration in 0.. {
        let mut closest_particle = None;
        let mut closest_particle_distance = None;
        
        for ref mut particle in &mut particles {
            particle.velocity.x = particle.velocity.x + particle.acceleration.x;
            particle.velocity.y = particle.velocity.y + particle.acceleration.y;
            particle.velocity.z = particle.velocity.z + particle.acceleration.z;
            particle.position.x = particle.position.x + particle.velocity.x;
            particle.position.y = particle.position.y + particle.velocity.y;
            particle.position.z = particle.position.z + particle.velocity.z;

            let distance = manhattan_distance(&particle.position);
            let is_closest = match closest_particle_distance {
                None => true,
                Some(x) => x > distance
            };
            if is_closest {
                closest_particle_distance = Some(distance);
                closest_particle = Some(particle.id);
            }
        }

        if iteration % 1000 == 0 {
            if closest_particle_last1000 == closest_particle {
                println!("Closest: {} (distance {})", closest_particle.unwrap(), closest_particle_distance.unwrap());
                break;
            }
                
            closest_particle_last1000 = closest_particle;
        }
    }

    let mut particles = parse_particles(input);
    let mut destroyedParticles = HashSet::new();
    let mut last_active_particles = None;
    for iteration in 0..100000 {
        let mut position_map: HashMap<V3, Vec<usize>> = HashMap::new();
        
        for ref mut particle in &mut particles {
            if destroyedParticles.contains(&particle.id) {
                continue;
            }

            particle.velocity.x = particle.velocity.x + particle.acceleration.x;
            particle.velocity.y = particle.velocity.y + particle.acceleration.y;
            particle.velocity.z = particle.velocity.z + particle.acceleration.z;
            particle.position.x = particle.position.x + particle.velocity.x;
            particle.position.y = particle.position.y + particle.velocity.y;
            particle.position.z = particle.position.z + particle.velocity.z;

            let position_list = position_map.entry(particle.position.clone()).or_insert(vec![]);
            position_list.push(particle.id);
        }

        for ref mut particle in &mut particles {
            if destroyedParticles.contains(&particle.id) {
                continue;
            }

            let position_list = position_map.get(&particle.position).unwrap();
            if position_list.len() > 1 {
                destroyedParticles.insert(particle.id);
            }
        }

        if iteration % 1000 == 0 {
            let num_active_particles = particles.iter().filter(|x| !destroyedParticles.contains(&x.id)).count();
            if Some(num_active_particles) == last_active_particles {
                println!("Num active particles after iteration {}: {}", iteration, num_active_particles);
                break;
            }
            last_active_particles = Some(num_active_particles);
        }
    }
}















fn manhattan_distance(v: &V3) -> usize {
    v.x.abs() as usize + v.y.abs() as usize + v.z.abs() as usize
}



fn parse_particles(input: &str) -> Vec<Particle> {
    let mut particles = Vec::new();
    for (id, line) in input.lines().enumerate() {
        // p=<-3787,-3683,3352>, v=<41,-25,-124>, a=<5,9,1>
        let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        let capture = re.captures_iter(line).last().expect("Regex should match");

        let position = {
            let x: isize = capture.get(1).unwrap().as_str().parse().unwrap();
            let y: isize = capture.get(2).unwrap().as_str().parse().unwrap();
            let z: isize = capture.get(3).unwrap().as_str().parse().unwrap();
            V3 { x, y, z }
        };
        let velocity = {
            let x: isize = capture.get(4).unwrap().as_str().parse().unwrap();
            let y: isize = capture.get(5).unwrap().as_str().parse().unwrap();
            let z: isize = capture.get(6).unwrap().as_str().parse().unwrap();
            V3 { x, y, z }
        };
        let acceleration = {
            let x: isize = capture.get(7).unwrap().as_str().parse().unwrap();
            let y: isize = capture.get(8).unwrap().as_str().parse().unwrap();
            let z: isize = capture.get(9).unwrap().as_str().parse().unwrap();
            V3 { x, y, z }
        };
        let particle = Particle { id, position, velocity, acceleration };
        particles.push(particle);
    }
    particles
}